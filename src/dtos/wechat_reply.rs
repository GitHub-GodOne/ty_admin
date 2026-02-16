/// 微信关键字回复 DTO
///
/// 对应Java:
/// - WechatReplySearchRequest
/// - WechatReplyRequest
/// - WechatReply (直接返回)
use serde::{Deserialize, Serialize};

/// 微信关键字回复搜索请求
///
/// Java: WechatReplySearchRequest
#[derive(Debug, Deserialize)]
pub struct WechatReplySearchRequest {
    /// 关键字
    pub keywords: Option<String>,

    /// 回复类型 text=文本 image=图片 news=图文 voice=音频
    #[serde(rename = "type")]
    pub reply_type: Option<String>,
}

/// 微信关键字回复新增/修改请求
///
/// Java: WechatReplyRequest
#[derive(Debug, Deserialize)]
pub struct WechatReplyRequest {
    /// 关键字id（修改时必填）
    pub id: Option<i32>,

    /// 关键字（必填）: 关注 = subscribe，默认 = default
    pub keywords: String,

    /// 回复类型（必填）: text=文本 image=图片 news=图文 voice=音频
    #[serde(rename = "type")]
    pub reply_type: String,

    /// 回复数据（必填）
    pub data: String,

    /// 回复状态 0=不可用 1=可用（必填）
    pub status: bool,
}

/// 微信关键字回复响应
///
/// Java: WechatReply model 直接返回
#[derive(Debug, Serialize, Clone)]
pub struct WechatReplyResponse {
    pub id: i32,
    pub keywords: String,
    #[serde(rename = "type")]
    pub reply_type: String,
    pub data: String,
    pub status: bool,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

/// 修改状态请求
#[derive(Debug, Deserialize)]
pub struct WechatReplyStatusRequest {
    pub id: i32,
    pub status: bool,
}
