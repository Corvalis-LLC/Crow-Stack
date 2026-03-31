#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILLS_SRC="$SCRIPT_DIR/skills"
SKILLS_DST="$HOME/.claude/skills"
TIMESTAMP="$(date +%Y%m%d-%H%M%S)"
BACKUP_DIR="$HOME/.claude/skills-backup-$TIMESTAMP"
BACKED_UP=false

if [ ! -d "$SKILLS_SRC" ]; then
  echo "Error: skills/ directory not found in $SCRIPT_DIR"
  exit 1
fi

mkdir -p "$SKILLS_DST"

for skill_dir in "$SKILLS_SRC"/*/; do
  skill_name="$(basename "$skill_dir")"
  target="$SKILLS_DST/$skill_name"

  # Back up existing skill if it exists and isn't already a symlink to us
  if [ -e "$target" ]; then
    if [ -L "$target" ] && [ "$(readlink "$target")" = "$skill_dir" ]; then
      echo "  skip: $skill_name (already linked)"
      continue
    fi
    if [ "$BACKED_UP" = false ]; then
      mkdir -p "$BACKUP_DIR"
      BACKED_UP=true
    fi
    echo "  backup: $skill_name -> $BACKUP_DIR/$skill_name"
    mv "$target" "$BACKUP_DIR/$skill_name"
  fi

  ln -s "$skill_dir" "$target"
  echo "  linked: $skill_name"
done

echo ""
echo "Done. Skills linked to $SKILLS_DST"
if [ "$BACKED_UP" = true ]; then
  echo "Backups saved to $BACKUP_DIR"
fi
