use loco_rs::prelude::*;
use std::fmt;

/// 业务错误类型
#[derive(Debug)]
pub enum AppError {
    /// 参数验证错误
    ValidationError(String),

    /// 业务逻辑错误
    BusinessError(String),

    /// 资源不存在
    NotFound(String),

    /// 未授权
    Unauthorized(String),

    /// 数据库错误
    DatabaseError(String),

    /// 内部错误
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "参数验证失败: {}", msg),
            AppError::BusinessError(msg) => write!(f, "业务错误: {}", msg),
            AppError::NotFound(msg) => write!(f, "资源不存在: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "未授权: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
            AppError::InternalError(msg) => write!(f, "内部错误: {}", msg),
        }
    }
}

impl From<AppError> for Error {
    fn from(err: AppError) -> Self {
        match err {
            AppError::ValidationError(msg) => Error::BadRequest(msg),
            AppError::NotFound(_msg) => Error::NotFound,
            AppError::Unauthorized(msg) => Error::Unauthorized(msg),
            _ => Error::string(&err.to_string()),
        }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

/// Result 类型别名
pub type AppResult<T> = std::result::Result<T, AppError>;
