use crate::modules::types::diff::Diff;
use crate::modules::types::differ::Differ;
use crate::modules::types::toast::Toast;
use pyo3::pymethods;
use std::collections::HashSet;

/// 通知列表差异计算工具类
#[pymethods]
impl Differ {
    /// 基于完整指纹 (含时间) 对比通知差异
    ///
    /// 逻辑:
    /// - 新通知: 新列表中有、旧列表中无的指纹
    /// - 移除通知: 旧列表中有、新列表中无的指纹
    ///
    /// Arguments:
    ///
    ///     old (list[Toast]): &[Toast] - 旧通知列表
    ///     new (list[Toast]): &[Toast] - 新通知列表
    ///
    /// Returns:
    ///
    ///     Diff[list[Toast], list[Toast]]: 包含新通知(new)和移除通知(remove)的差异结构体
    #[staticmethod]
    pub fn diff_full(old: Vec<Toast>, new: Vec<Toast>) -> Diff {
        let old_set: HashSet<&String> = old.iter().map(|n| &n.fingerprint).collect();
        let new_set: HashSet<&String> = new.iter().map(|n| &n.fingerprint).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_set.contains(&n.fingerprint))
            .cloned()
            .collect();
        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_set.contains(&n.fingerprint))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 基于通知ID对比通知差异
    ///
    /// 逻辑:
    /// - 新通知: 新列表中有、旧列表中无的ID
    /// - 移除通知: 旧列表中有、新列表中无的ID
    ///
    /// Arguments:
    ///
    ///     old (list[Toast]): &[Toast] - 旧通知列表
    ///
    ///     new (list[Toast]): &[Toast] - 新通知列表
    ///
    /// Returns:
    ///
    ///     Diff[list[Toast], list[Toast]]: 包含新通知(new)和移除通知(remove)的差异结构体
    #[staticmethod]
    pub fn diff_by_id(old: Vec<Toast>, new: Vec<Toast>) -> Diff {
        let old_ids: HashSet<u32> = old.iter().map(|n| n.id.clone()).collect();
        let new_ids: HashSet<u32> = new.iter().map(|n| n.id.clone()).collect();

        let new_items: Vec<Toast> = new
            .into_iter()
            .filter(|n| !old_ids.contains(&n.id))
            .collect();

        let remove_items: Vec<Toast> = old
            .into_iter()
            .filter(|n| !new_ids.contains(&n.id))
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 基于无时间指纹对比通知差异
    ///
    /// 逻辑:
    /// - 新通知: 新列表中有、旧列表中无的无时间指纹
    /// - 移除通知: 旧列表中有、新列表中无的无时间指纹
    ///
    /// Arguments:
    ///
    ///     old (list[Toast]): &[Toast] - 旧通知列表
    ///
    ///     new (list[Toast]): &[Toast] - 新通知列表
    ///
    /// Returns:
    ///
    ///     Diff: 包含新通知(new)和移除通知(remove)的差异结构体
    #[staticmethod]
    pub fn diff_without_time(old: Vec<Toast>, new: Vec<Toast>) -> Diff {
        let old_set: HashSet<&String> = old.iter().map(|n| &n.fingerprint_without_time).collect();
        let new_set: HashSet<&String> = new.iter().map(|n| &n.fingerprint_without_time).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_set.contains(&n.fingerprint_without_time))
            .cloned()
            .collect();
        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_set.contains(&n.fingerprint_without_time))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 将通知列表序列化为格式化的 JSON 数组字符串
    ///
    /// 逻辑:
    /// - 使用serde_json序列化, 失败时返回空数组JSON字符串 ("[]")
    ///
    /// Arguments:
    ///
    ///     notifications (list[Toast]): &[Toast] - 待序列化的通知列表
    ///
    /// Returns:
    ///     str: 格式化的JSON字符串, 失败返回"[]"
    #[staticmethod]
    pub fn serialize(notifications: Vec<Toast>) -> String {
        serde_json::to_string_pretty(&notifications).unwrap_or_else(|_| "[]".to_string())
    }
}
