use pyo3::pyclass;
use serde::Serialize;

/// 此类用于定义 Python 侧的可变通知对象
///
/// 相关字段定义同 Toast
#[pyclass(from_py_object, get_all, set_all)]
#[derive(Serialize, Clone, Debug)]
pub struct MutableToast {
    pub id: u32,
    pub name: String,
    pub logo_uri: String,
    pub title: String,
    pub message: String,
    pub hero_image_uri: String,
    pub inline_images: Vec<String>,
    pub tag: String,
    pub group: String,
    pub creation_time: String,
    pub fingerprint: String,
    pub fingerprint_without_time: String,
}
