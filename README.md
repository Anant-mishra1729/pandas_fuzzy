# pandas-fuzzy

Fast fuzzy string matching for pandas, powered by Rust and Apache Arrow.

`pandas-fuzzy` filters DataFrame rows by fuzzy string similarity, using a Rust core (via [rapidfuzz-rs](https://github.com/rapidfuzz/rapidfuzz-rs)) and zero-copy Arrow array access to avoid the overhead of per-row Python calls.

## Install

```bash
pip install pandas-fuzzy
```

## Quickstart

```python
import pandas as pd
from pandas_fuzzy import fuzzy_filter

df = pd.DataFrame({"name": ["Anant", "John", "Johnas", "Rolles"]})

fuzzy_filter(df, col="name", query="Ann", score_cutoff=0.5)
```
Output

```
name
0  Anant
```

## How it works

- Your column is converted to an Arrow array (zero-copy if it's already Arrow-backed).
- Scoring runs in Rust, outside Python's GIL, using `rapidfuzz-rs` for the actual similarity computation.
- Only rows clearing `score_cutoff` are kept.

## API

```python
fuzzy_filter(
    df: pd.DataFrame,
    col: str,
    query: str,
    score_cutoff: float = 0.8,
    method: Literal["levenshtein", "jaro_winkler"] = "jaro_winkler",
) -> pd.DataFrame
```

| Parameter      | Description                                              |
|----------------|-----------------------------------------------------------|
| `col`          | Column to match against                                  |
| `query`        | String to compare each value to                           |
| `score_cutoff` | Minimum similarity (0.0–1.0) required to keep a row       |
| `method`       | Similarity metric: `"levenshtein"` or `"jaro_winkler"`    |

## Status

Early (`0.1.0`) — API may change. Currently supports `fuzzy_filter`; `fuzzy_join`, `fuzzy_isin`, and `fuzzy_dedupe` are planned.

## License

MIT