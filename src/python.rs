use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::epd::EPD;
use crate::parse;

// #[cfg(feature = "pybindings")]
// #[pyfunction]
// fn make_dict<'py>(py: Python<'py>, epd: &EPD) -> PyResult<&'py PyDict> {
//     let dict = PyDict::new(py);
//     for value in epd.iter() {
//         dict.set_item(value.name, value.value).unwrap();
//     }
//     Ok(dict)
// }

// #[pyclass]
// struct EPDWrapper(EPD);
//
// impl IntoPy<Py<PyDict>> for EPDWrapper {
//     // https://stackoverflow.com/questions/71676419/iterate-over-struct-in-rust
//     fn into_py(self, py: Python<'_>) -> Py<PyDict> {
//         let mut dict = PyDict::new(py).into_py(py);
//         let epd = self.0;
//         for value in epd.iter() {
//             dict.set_item(value.name, value.value).unwrap();
//         }
//         dict
//     }
// }

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn convert_ilcd(json: String) -> String {
    let epd = parse::parse_ilcd(json);
    serde_json::to_string(&epd).unwrap()
    // Python::with_gil(|py| {
    //     EPDWrapper(epd).into_py(py)
    // })
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