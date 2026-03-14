use crate::modules::types::toast::Toast;
use pyo3::{PyResult, pymethods};

#[pymethods]
impl Toast {
    #[new]
    pub fn __init__(
        id: u32,
        name: String,
        logo_uri: String,
        title: String,
        message: String,
        hero_image_uri: String,
        inline_images: Vec<String>,
        tag: String,
        group: String,
        creation_time: String,
        fingerprint: String,
        fingerprint_without_time: String,
    ) -> PyResult<Self> {
        Ok(Self {
            id,
            name,
            logo_uri,
            title,
            message,
            hero_image_uri,
            inline_images,
            tag,
            group,
            creation_time,
            fingerprint,
            fingerprint_without_time,
        })
    }
}
