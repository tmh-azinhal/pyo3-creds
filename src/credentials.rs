use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub usertype: String,
}

#[pymethods]
impl Credentials {
    #[new]
    fn new(username: String, password: String, usertype: String) -> Self {
        Credentials {
            username,
            password,
            usertype,
        }
    }
}
