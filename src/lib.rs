mod modules;
use modules::types;

use pyo3::prelude::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python, pymodule};

#[pymodule]
fn win_notice_lite(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let m_features = PyModule::new(_py, "features")?;
    
    (&*m).add("__author__", env!("CARGO_PKG_AUTHORS"))?;
    (&*m).add("__version__", env!("CARGO_PKG_VERSION"))?;
    (&*m).add("__home__", env!("CARGO_PKG_HOMEPAGE"))?;
    (&*m).add("__license__", env!("CARGO_PKG_LICENSE"))?;
    (&*m).add("__description__", env!("CARGO_PKG_DESCRIPTION"))?;
    
    (&*m).add_class::<types::listener::Listener>()?;
    (&*m).add_class::<types::toast::Toast>()?;
    (&*m).add_class::<types::mutable_toast::MutableToast>()?;
    (&*m).add_class::<types::diff::Diff>()?;
    (&*m).add_class::<types::diff_tool::DiffTool>()?;
    (&*m).add_class::<types::serialize_format::SerializeFormat>()?;

    (&m_features).add_class::<types::events::features::callback_token::CallbackToken>()?;
    (&m_features).add_class::<types::events::features::events_type::EventsType>()?;
    (&m_features).add_class::<types::events::features::polling_status::PollingStatus>()?;
    (&m_features).add_class::<types::events::features::polling_eventify::Polling>()?;

    (&*m).add_submodule(&m_features)?;
    Ok(())
}
