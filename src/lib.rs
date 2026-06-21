use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
mod _fuzzy {
    use arrow_array::{Array, StringArray};
    use pyo3::prelude::*;
    use pyo3_arrow::PyArray;
    use rapidfuzz::distance::{jaro_winkler, levenshtein};

    #[pyfunction]
    fn levenshtein_similarity(s1: &str, s2: &str) -> f64 {
        levenshtein::normalized_similarity(s1.bytes(), s2.bytes())
    }

    #[pyfunction]
    fn levenshtein_similarity_with_cutoff(s1: &str, s2: &str, cutoff: f64) -> Option<f64> {
        levenshtein::normalized_similarity_with_args(
            s1.bytes(),
            s2.bytes(),
            &levenshtein::Args::default().score_cutoff(cutoff),
        )
    }

    #[pyfunction]
    fn jaro_winkler_similarity(s1: &str, s2: &str) -> f64 {
        jaro_winkler::normalized_similarity(s1.bytes(), s2.bytes())
    }

    #[pyfunction]
    fn jaro_winkler_similarity_with_cutoff(s1: &str, s2: &str, cutoff: f64) -> Option<f64> {
        jaro_winkler::normalized_similarity_with_args(
            s1.bytes(),
            s2.bytes(),
            &jaro_winkler::Args::default().score_cutoff(cutoff),
        )
    }

    type ScorerFn = fn(&str, &str, f64) -> Option<f64>;

    fn resolve_scorer(method: &str) -> PyResult<ScorerFn> {
        match method {
            "levenshtein" => Ok(levenshtein_similarity_with_cutoff),
            "jaro_winkler" => Ok(jaro_winkler_similarity_with_cutoff),
            other => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unknown method '{other}', expected 'levenshtein' or 'jaro_winkler'"
            ))),
        }
    }

    #[pyfunction]
    fn fuzzy_filter_mask(
        py: Python<'_>,
        array: PyArray,
        query: &str,
        score_cutoff: f64,
        method: &str,
    ) -> PyResult<Vec<bool>> {
        let scorer = resolve_scorer(method)?;

        let string_array = array
            .as_ref()
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                pyo3::exceptions::PyTypeError::new_err(
                    "fuzzy_filter expects a string-typed Arrow array",
                )
            })?
            .clone();

        let query = query.to_owned();

        Ok(py.detach(move || {
            (0..string_array.len())
                .map(|i| {
                    if string_array.is_null(i) {
                        false
                    } else {
                        let s = string_array.value(i);
                        scorer(s, &query, score_cutoff).is_some()
                    }
                })
                .collect()
        }))
    }
}
