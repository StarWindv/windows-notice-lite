use crate::modules::types::events::features::callback_token::CallbackToken;

use pyo3::pymethods;
use std::sync::atomic::Ordering;

#[pymethods]
impl CallbackToken {
    #[staticmethod]
    pub fn new() -> Self {
        let id = crate::modules::types::events::features::callback_token::TOKEN_COUNTER
            .fetch_add(1, Ordering::SeqCst);
        Self { id }
    }

    pub fn __str__(&self) -> String {
        format!("CallbackToken({})", self.id)
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}
