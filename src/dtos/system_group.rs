use serde::{Deserialize, Serialize};

// ==================== 请求 DTO ====================

/// 组合数据搜索请求
/// Java: SystemGroupSearchRequest
#[derive(Debug, Deserialize)]
pub struct SystemGroupSearchRequest {
    /// 关键字
    pub keywords: Option<String>,
}

/// 组合数据新增/编辑请求
/// Java: SystemGroupRequest
#[derive(Debug, Deserialize)]
pub struct SystemGroupRequest {
    /// 数据组名称
    pub name: Option<String>,

    /// 简介
    pub info: Option<String>,

    /// form 表单 id
    #[serde(rename = "formId")]
    pub form_id: Option<i32>,
}

// ==================== 响应 DTO ====================

/// 组合数据响应
/// Java: SystemGroup
#[derive(Debug, Serialize)]
pub struct SystemGroupResponse {
    pub id: i32,
    pub name: String,
    pub info: String,
    #[serde(rename = "formId")]
    pub form_id: i32,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}
