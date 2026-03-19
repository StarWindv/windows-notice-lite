use super::toast::Toast;
use pyo3::pyclass;

///
/// Attributes:
///
///     new (list[Toast]): Vec<Toast> 一个列表属性, 代表新增的通知, 直接访问即可
///
///     remove (list[Toast]): Vec<Toast> 一个列表属性, 代表 已去除/过期 的通知
#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone)]
pub struct Diff {
    pub new: Vec<Toast>,
    pub remove: Vec<Toast>,
}
