use serde::{Deserialize, Serialize};

/// 用户分组请求 - 对应Java的UserGroupRequest
#[derive(Debug, Deserialize)]
pub struct UserGroupRequest {
    /// 分组名称
    #[serde(rename = "groupName")]
    pub group_name: String,
}

/// 用户分组响应
#[derive(Debug, Serialize)]
pub struct UserGroupResponse {
    /// 分组ID
    pub id: i32,

    /// 分组名称
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
}
