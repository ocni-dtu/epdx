use pyo3::prelude::*;
use crate::parse;


#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn convert_ilcd(json: String) -> String {
    let epd = parse::parse_ilcd(json);
    serde_json::to_string(&epd).unwrap()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[cfg(feature = "pybindings")]
#[pymodule]
fn epdx(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_ilcd, m)?)?;

    Ok(())
}