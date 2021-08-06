use pyo3::prelude::*;
pub mod credentials;
use credentials::Credentials;

#[pymodule]
fn creds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Credentials>()?;
    Ok(())
}

