use super::super::error::ConvertToPyErr;
use super::super::types::{listener::Listener, toast::Toast};
use super::super::utils::notification_tools::parse_notification;

use pyo3::{PyResult, pymethods};

use windows::UI::Notifications::{
    Management::UserNotificationListener, Management::UserNotificationListenerAccessStatus,
    NotificationKinds, UserNotification,
};
use windows_collections::IVectorView;
use windows_future::IAsyncOperation;

/// 获取桌面通知的主类
#[pymethods]
impl Listener {
    /// 创建通知监听器实例
    ///
    /// Returns:
    ///
    ///     self (object): Result<Self> - 成功返回Listener实例, 失败返回错误
    #[new]
    pub fn new() -> PyResult<Self> {
        let listener = UserNotificationListener::Current().to()?;
        Ok(Self { listener })
    }

    /// 请求通知访问权限 (提权)
    ///
    /// Type:
    ///
    ///     async: 异步接口, 需要使用 `await` 来获取数据
    ///
    /// Notes:
    ///     建议从UI线程调用, 否则容易报错
    ///
    ///     但这是 C-Sharp 的规矩, 我也不知道转 Python 会发生什么, 大家用着看就是了
    ///
    /// Returns:
    ///
    ///     str: PyResult<String> -> IAsyncOperation<UserNotificationListenerAccessStatus>
    ///         也就是说拿到的就是权限枚举的字符串
    ///
    /// Example:
    /// ```python
    ///     import win_notice_lite as wnl
    ///     listener = wnl.Listener()
    ///     permission: str = await listener.elevate_privilege()
    /// ```
    /// 这里的枚举字符串长这样:
    ///
    ///     UserNotificationListenerAccessStatus(level)
    ///
    /// level:
    ///  - 0: 无操作, 也有可能是未弹出授权窗口或者超时
    ///  - 1: Access 已授权
    ///  - 2: Denied 用户明确拒绝授权
    ///
    /// 应该还有一些值, 据说还有权限 3, 但是我真的没找到对应含义故不表
    pub async fn elevate_privilege(&self) -> PyResult<String> {
        let operation: IAsyncOperation<UserNotificationListenerAccessStatus> =
            self.listener.RequestAccessAsync().to()?;
        let status = operation.await.to()?;
        Ok(format!("{:?}", status))
    }

    /// 获取当前系统中所有Toast类型的通知
    ///
    /// Type:
    ///
    ///     async: 异步接口, 需要使用 `await` 来获取数据
    ///
    /// 逻辑:
    /// 1. 检查通知访问权限, 无权限直接返回空数组
    /// 2. 异步获取所有Toast类型通知, 解析为Toast结构体数组
    ///
    /// Returns:
    ///
    ///     list[Toast]: Result<Vec<Toast>> - 成功返回Toast数组, 失败返回Windows API错误
    pub async fn get_all_notifications(&self) -> PyResult<Vec<Toast>> {
        let status = self.listener.GetAccessStatus().to()?;
        if status != UserNotificationListenerAccessStatus::Allowed {
            return Ok(vec![]);
        }

        let operation: IAsyncOperation<IVectorView<UserNotification>> = self
            .listener
            .GetNotificationsAsync(NotificationKinds::Toast)
            .to()?;
        let raw_notifications = operation.await.to()?;

        let mut notifications = Vec::with_capacity(raw_notifications.Size().to()? as usize);
        for i in 0..raw_notifications.Size().to()? {
            let notif = raw_notifications.GetAt(i).to()?;
            notifications.push(parse_notification(&notif)?);
        }
        Ok(notifications)
    }
}
