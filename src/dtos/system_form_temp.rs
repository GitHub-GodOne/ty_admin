use serde::{Deserialize, Serialize};

/// 表单模板搜索请求
#[derive(Debug, Deserialize)]
pub struct SystemFormTempSearchRequest {
    /// 搜索关键字
    pub keywords: Option<String>,
}

/// 表单模板新增/修改请求
#[derive(Debug, Deserialize)]
pub struct SystemFormTempRequest {
    /// 表单名称（必填，最大500字符）
    pub name: String,

    /// 表单简介（必填，最大500字符）
    pub info: String,

    /// 表单内容（必填，JSON格式）
    pub content: String,
}

/// 表单模板响应
#[derive(Debug, Serialize)]
pub struct SystemFormTempResponse {
    pub id: i32,
    /// 表单名称
    pub name: String,
    /// 表单简介
    pub info: String,
    /// 表单内容
    pub content: String,
    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}
