use serde::Serialize;

/// 统一 API 响应格式
///
/// 与Java CRMEB保持一致：成功时message为null，失败时message为错误信息
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// 状态码：200=成功，其他=失败
    pub code: i32,

    /// 响应消息（成功时为null）
    pub message: Option<String>,

    /// 响应数据（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应（带数据，message为null）
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: None,
            data: Some(data),
        }
    }

    /// 成功响应（带自定义消息）
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            code: 200,
            message: Some(message.into()),
            data: Some(data),
        }
    }

    /// 失败响应（带错误码）
    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: Some(message.into()),
            data: None,
        }
    }

    /// 失败响应
    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            code: 500,
            message: Some(message.into()),
            data: None,
        }
    }

    /// 成功响应（无数据，message为null）
    pub fn success_empty() -> Self {
        Self {
            code: 200,
            message: None,
            data: None,
        }
    }
}

/// 便捷宏：快速创建成功响应
#[macro_export]
macro_rules! ok {
    ($data:expr) => {
        loco_rs::prelude::format::json($crate::common::response::ApiResponse::success($data))
    };
    ($data:expr, $msg:expr) => {
        loco_rs::prelude::format::json($crate::common::response::ApiResponse::success_with_message($data, $msg))
    };
}

/// 便捷宏：快速创建错误响应
#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        loco_rs::prelude::format::json($crate::common::response::ApiResponse::<()>::failed($msg))
    };
    ($code:expr, $msg:expr) => {
        loco_rs::prelude::format::json($crate::common::response::ApiResponse::<()>::error($code, $msg))
    };
}
