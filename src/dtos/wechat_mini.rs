use serde::{Deserialize, Serialize};

/// 微信小程序码响应
/// Java: QrCodeVo
#[derive(Debug, Serialize)]
pub struct QrCodeVo {
    /// 二维码（base64已处理）
    pub code: String,
}

/// 微信access_token响应
/// Java: WeChatAccessTokenVo
#[derive(Debug, Deserialize)]
pub struct WeChatAccessTokenVo {
    pub access_token: Option<String>,
    pub expires_in: Option<i64>,
    pub errcode: Option<i64>,
    pub errmsg: Option<String>,
}
