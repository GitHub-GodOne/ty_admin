/// 用户积分管理 DTO
///
/// 对应Java:
/// - AdminIntegralSearchRequest
/// - UserIntegralRecordResponse
use serde::{Deserialize, Serialize};

/// 管理端积分搜索请求
///
/// Java: AdminIntegralSearchRequest
#[derive(Debug, Deserialize)]
pub struct AdminIntegralSearchRequest {
    /// 时间区间 (today, yesterday, lately7, lately30, month, year, 或 yyyy/MM/dd-yyyy/MM/dd)
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,

    /// 搜索关键字（用户昵称）
    pub keywords: Option<String>,

    /// 用户uid
    pub uid: Option<i32>,
}

/// 积分记录响应
///
/// Java: UserIntegralRecordResponse
/// 在 UserIntegralRecord 基础上增加 nickName 字段
#[derive(Debug, Serialize, Clone)]
pub struct UserIntegralRecordResponse {
    pub id: i32,
    pub uid: i32,
    #[serde(rename = "linkId")]
    pub link_id: String,
    #[serde(rename = "linkType")]
    pub link_type: String,
    #[serde(rename = "type")]
    pub record_type: i32,
    pub title: String,
    pub integral: i32,
    pub balance: i32,
    pub mark: String,
    pub status: i16,
    #[serde(rename = "frozenTime")]
    pub frozen_time: i32,
    #[serde(rename = "thawTime")]
    pub thaw_time: i64,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    /// 用户昵称（关联查询）
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
}
