use crate::modules::types::diff::Diff;
use crate::modules::types::toast::Toast;
use pyo3::{PyResult, pymethods};

#[pymethods]
impl Diff {
    #[new]
    pub fn __init__(new: Vec<Toast>, remove: Vec<Toast>) -> PyResult<Self> {
        Ok(Self { new, remove })
    }
}
