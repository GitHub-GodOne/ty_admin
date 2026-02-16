use serde::{Deserialize, Serialize};

/// 用户标签请求 - 对应Java的UserTagRequest
#[derive(Debug, Deserialize)]
pub struct UserTagRequest {
    /// 标签名称
    pub name: String,
}

/// 用户标签响应
#[derive(Debug, Serialize)]
pub struct UserTagResponse {
    /// 标签ID
    pub id: i32,

    /// 标签名称
    pub name: Option<String>,
}
