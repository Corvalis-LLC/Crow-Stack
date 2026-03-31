---
name: auto-python
description: "Python standards for type hints, error handling, async patterns, testing with pytest, data classes, dependency management, and modern Python idioms. MUST USE WHEN writing Python code, defining type hints, using pytest, managing dependencies with uv, writing async code, building data pipelines, training ML models, or configuring ruff/mypy. Triggers: python, .py, pytest, type hint, typing, Protocol, ABC, dataclass, pydantic, async, await, asyncio, import, pip, uv, poetry, ruff, mypy, pyright, pyproject.toml, match, f-string, pathlib, hypothesis, fixture, conftest, __init__, __main__, venv, virtualenv, pandas, polars, numpy, torch, transformers, PEFT, LoRA, QLoRA, fine-tune, training, ollama, SFTTrainer, wandb."
---

# Python — Patterns Claude Inconsistently Applies

## Exception Specificity

Claude catches broad `Exception` where specific types exist. Always catch the narrowest type:

```python
# WRONG — overly broad, masks bugs
try:
    result = parse(data)
except Exception:
    result = default

# WRONG — Exception alongside specific type is redundant and broad
except (ProcessingError, Exception) as exc:
    handle(exc)

# RIGHT — catch exactly what can fail
except (ValueError, json.JSONDecodeError) as exc:
    logger.warning("Parse failed: %s", exc)
    result = default
```

Only catch `Exception` at the outermost boundary (CLI entry point, top-level server handler) where you log-and-exit. Everywhere else, name the specific exceptions.

## Dataclass Defaults

Always apply all three flags — Claude often omits `frozen` or `kw_only`:

```python
@dataclass(frozen=True, slots=True, kw_only=True)
class SensorReading:
    station_id: str
    temperature: float
    timestamp: datetime
```

- `frozen=True` — immutable, hashable, prevents accidental mutation
- `slots=True` — ~50% memory reduction per instance
- `kw_only=True` — prevents positional arg ordering bugs
- Pydantic `BaseModel` at trust boundaries only (API input, config files, external data)
