mod impls;

use polars::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn factorial(n: u128) -> PyResult<u128> {
    Ok(_factorial(n))
}

fn _factorial(n: u128) -> u128 {
    if n <= 1 {
        return n;
    } else {
        return n * _factorial(n - 1);
    }
}

#[pyfunction]
fn hamming_distance(
    series_a: &Bound<PyAny>,
    series_b: &Bound<PyAny>,
) -> PyResult<PyObject> {
    let series_a = impls::py_series_to_rust_series(series_a)?;
    let series_b = impls::py_series_to_rust_series(series_b)?;

    let out = hamming_distance_impl(&series_a, &series_b).map_err(|e| {
        PyValueError::new_err(format!("Something went wrong: {:?}", e))
    })?;
    impls::rust_series_to_py_series(&out.into_series())
}

/// This function iterates over 2 `StringChunked` arrays and computes the hamming distance between the values .
fn hamming_distance_impl(
    a: &Series,
    b: &Series,
) -> PolarsResult<UInt32Chunked> {
    Ok(a.str()?
        .into_iter()
        .zip(b.str()?)
        .map(|(lhs, rhs)| hamming_distance_strs(lhs, rhs))
        .collect())
}

/// Compute the hamming distance between 2 string values.
fn hamming_distance_strs(a: Option<&str>, b: Option<&str>) -> Option<u32> {
    match (a, b) {
        (None, _) => None,
        (_, None) => None,
        (Some(a), Some(b)) => {
            if a.len() != b.len() {
                None
            } else {
                Some(
                    a.chars()
                        .zip(b.chars())
                        .map(|(a_char, b_char)| (a_char != b_char) as u32)
                        .sum::<u32>(),
                )
            }
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn options_dataframes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    Ok(())
}
