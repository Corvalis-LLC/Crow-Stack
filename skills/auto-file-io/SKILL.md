---
name: auto-file-io
description: "File I/O discipline: atomic writes, streaming large files, error path cleanup, file locking, and platform awareness. Corrects direct overwrites that corrupt on crash, loading entire files into memory, orphaned temp files on error, and missing platform-specific path handling. Use when reading or writing files, processing uploads, managing temp files, or working with filesystem paths. Triggers: file, write file, read file, atomic write, temp file, tempfile, streaming, upload, download, path, PathBuf, fs::write, fs::read, open, truncate, rename, file lock, flock, platform, symlink."
---

# File I/O — What Claude Gets Wrong

You write `fs::write(path, data)` and assume it works. In production: power cuts mid-write leave a corrupted file, your 2GB CSV load OOMs because you read it all into memory, your error path leaves temp files scattered everywhere, and your path handling breaks on Windows or when symlinks are involved.

## The Five Rules

1. **Atomic writes** — write to temp file, then rename. Never overwrite in place.
2. **Stream large files** — don't load entire files into memory
3. **Clean up on all paths** — error paths must remove temp files
4. **Lock when sharing** — concurrent writers need advisory locks
5. **Use path abstractions** — no string concatenation for paths

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Direct overwrite | `fs::write("config.toml", data)` | Write to `.tmp`, then `fs::rename` |
| Full memory load | `fs::read_to_string("big.csv")` | `BufReader` + line iteration |
| Orphaned temp files | Error after creating temp, no cleanup | `tempfile::NamedTempFile` (auto-cleanup on drop) |
| String path concat | `format!("{}/file.txt", dir)` | `dir.join("file.txt")` via `Path`/`PathBuf` |
| No fsync | Rename without sync — data in OS buffer | `file.sync_all()` before rename |
| Ignoring rename limits | `rename()` across filesystems | Temp file in SAME directory as target |

## Atomic Writes

```rust
// WRONG: crash mid-write = corrupted file
fn save_config(path: &Path, config: &Config) -> Result<()> {
    let data = toml::to_string(config)?;
    fs::write(path, data)?; // Partial write on crash = corrupt
    Ok(())
}

// RIGHT: write-to-temp + fsync + rename (atomic on POSIX)
fn save_config(path: &Path, config: &Config) -> Result<()> {
    let data = toml::to_string(config)?;
    let dir = path.parent().context("path has no parent")?;

    // Temp file in same directory — rename must not cross filesystem boundary
    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;
    tmp.write_all(data.as_bytes())?;
    tmp.as_file().sync_all()?; // Flush to disk before rename
    tmp.persist(path)?;        // Atomic rename; temp auto-cleaned if this fails
    Ok(())
}
```

```python
# WRONG: direct write
with open("config.toml", "w") as f:
    f.write(data)

# RIGHT: atomic write
import tempfile, os
dir_name = os.path.dirname(target_path)
with tempfile.NamedTemporaryFile(mode="w", dir=dir_name, delete=False) as tmp:
    tmp.write(data)
    tmp.flush()
    os.fsync(tmp.fileno())
    tmp_path = tmp.name
os.replace(tmp_path, target_path)  # Atomic on POSIX
```

```typescript
// WRONG: direct write
await fs.writeFile("config.json", data);

// RIGHT: atomic write (write-tmp-rename)
import { writeFile, rename, fdatasync, open } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

async function atomicWrite(target: string, data: string): Promise<void> {
    const dir = dirname(target);
    const tmp = join(dir, `.${Date.now()}.tmp`);
    const fh = await open(tmp, "w");
    try {
        await fh.writeFile(data);
        await fh.datasync();
        await fh.close();
        await rename(tmp, target);
    } catch (e) {
        await fh.close().catch(() => {});
        await fs.unlink(tmp).catch(() => {});
        throw e;
    }
}
```

## Streaming Large Files

```rust
// WRONG: loads entire file into memory
let contents = fs::read_to_string("huge.csv")?;
for line in contents.lines() { /* ... */ }

// RIGHT: stream line by line
let file = File::open("huge.csv")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    process_line(&line)?;
}
```

```python
# WRONG: reads entire file
data = open("huge.csv").read()

# RIGHT: iterate lines (constant memory)
with open("huge.csv") as f:
    for line in f:
        process_line(line)
```

## Error Path Cleanup

```rust
// WRONG: temp file leaks if processing fails
fn process_upload(data: &[u8]) -> Result<PathBuf> {
    let tmp_path = PathBuf::from("/tmp/upload.dat");
    fs::write(&tmp_path, data)?;
    validate(&tmp_path)?;       // If this fails, temp file is orphaned
    let final_path = dest.join("upload.dat");
    fs::rename(&tmp_path, &final_path)?;
    Ok(final_path)
}

// RIGHT: NamedTempFile auto-cleans on drop (including error paths)
fn process_upload(data: &[u8]) -> Result<PathBuf> {
    let mut tmp = tempfile::NamedTempFile::new()?;
    tmp.write_all(data)?;
    validate(tmp.path())?;      // If this fails, tmp is dropped → file deleted
    let final_path = dest.join("upload.dat");
    tmp.persist(&final_path)?;  // Only persists on success
    Ok(final_path)
}
```

## Path Handling

```rust
// WRONG: string concatenation
let path = format!("{}/{}", base_dir, filename);

// RIGHT: Path::join handles separators and normalization
let path = base_dir.join(filename);

// WRONG: assuming path structure
let name = path.split('/').last().unwrap();

// RIGHT: use Path methods
let name = path.file_name().context("no filename")?;
let ext = path.extension().unwrap_or_default();
let parent = path.parent().context("no parent")?;
```

## File Locking

```rust
// When multiple processes might write the same file
use fs2::FileExt;

fn update_shared_state(path: &Path) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
    file.lock_exclusive()?; // Blocks until lock acquired
    // ... read, modify, write ...
    file.unlock()?;
    Ok(())
}
// For non-blocking: file.try_lock_exclusive()? with proper error handling
```
