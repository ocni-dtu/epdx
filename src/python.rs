use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;
use crate::parse;

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn _convert_ilcd(json: String) -> PyResult<String> {
    let epd = parse::parse_ilcd(json);
    match epd {
        Ok(epd) => Ok(serde_json::to_string(&epd).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string()))
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[cfg(feature = "pybindings")]
#[pymodule]
fn epdx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_convert_ilcd, m)?)?;

    Ok(())
}