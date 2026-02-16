use serde::{Deserialize, Serialize};

/// 通知搜索请求
///
/// Java: NotificationSearchRequest
#[derive(Debug, Deserialize)]
pub struct NotificationSearchRequest {
    /// 发送类型: 1=通知用户, 2=通知平台
    #[serde(rename = "sendType", default)]
    pub send_type: Option<i32>,
}

/// 通知详情请求
///
/// Java: NotificationInfoRequest
#[derive(Debug, Deserialize)]
pub struct NotificationInfoRequest {
    /// 通知id
    pub id: i32,

    /// 类型: wechat/routine/sms
    #[serde(rename = "detailType")]
    pub detail_type: String,
}

/// 通知修改请求
///
/// Java: NotificationUpdateRequest
#[derive(Debug, Deserialize)]
pub struct NotificationUpdateRequest {
    /// 通知id
    pub id: i32,

    /// 类型: wechat/routine/sms
    #[serde(rename = "detailType")]
    pub detail_type: String,

    /// 模板id
    #[serde(rename = "tempId", default)]
    pub temp_id: Option<String>,

    /// 状态: 1=开启, 2=关闭
    pub status: i16,
}

/// 通知详情响应
///
/// Java: NotificationInfoResponse
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationInfoResponse {
    pub id: i32,
    pub temp_id: Option<String>,
    pub title: Option<String>,
    pub temp_key: Option<String>,
    pub content: Option<String>,
    pub name: Option<String>,
    pub status: i16,
}

/// 通知列表响应
///
/// Java: SystemNotification
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemNotificationResponse {
    pub id: i32,
    pub mark: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub description: String,
    pub is_wechat: i16,
    pub wechat_id: i32,
    pub is_routine: i16,
    pub routine_id: i32,
    pub is_sms: i16,
    pub sms_id: i32,
    pub send_type: i16,
    pub create_time: Option<String>,
}
