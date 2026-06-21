import pandas as pd
from pandas_fuzzy._fuzzy import fuzzy_filter_mask
from typing import Literal
import pyarrow as pa


def fuzzy_filter(
    df: pd.DataFrame,
    col: str,
    query: str,
    score_cutoff: float = 0.8,
    method: Literal["levenshtein", "jaro_winkler"] = "jaro_winkler",
):
    series = df[col]
    arrow_array = pa.array(series)
    mask = fuzzy_filter_mask(arrow_array, query, score_cutoff, method)
    return df.loc[mask]
