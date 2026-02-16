/// DIY页面 DTO
///
/// Java参考:
/// - PageDiyRequest
/// - PageDiyEditNameRequest
/// - PageDiyResponse
use serde::{Deserialize, Serialize};

/// DIY页面搜索参数
#[derive(Debug, Deserialize)]
pub struct PageDiySearchParam {
    /// 搜索关键字（名称模糊搜索）
    #[serde(default)]
    pub name: Option<String>,
}

/// DIY页面新增/修改请求
///
/// Java: PageDiyRequest
/// value字段前端传JSONObject，这里用serde_json::Value接收
#[derive(Debug, Deserialize)]
pub struct PageDiyRequest {
    pub id: Option<i32>,
    pub version: Option<String>,
    pub name: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<String>,
    #[serde(rename = "templateName")]
    pub template_name: Option<String>,
    /// 页面数据 (JSONObject)
    pub value: Option<serde_json::Value>,
    pub status: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
    #[serde(rename = "isShow")]
    pub is_show: Option<i32>,
    #[serde(rename = "isBgColor")]
    pub is_bg_color: Option<i32>,
    #[serde(rename = "isBgPic")]
    pub is_bg_pic: Option<i32>,
    #[serde(rename = "isDiy")]
    pub is_diy: Option<i32>,
    #[serde(rename = "colorPicker")]
    pub color_picker: Option<String>,
    #[serde(rename = "bgPic")]
    pub bg_pic: Option<String>,
    #[serde(rename = "bgTabVal")]
    pub bg_tab_val: Option<i32>,
    #[serde(rename = "isDel")]
    pub is_del: Option<i32>,
    #[serde(rename = "returnAddress")]
    pub return_address: Option<String>,
    #[serde(rename = "titleBgColor")]
    pub title_bg_color: Option<String>,
    #[serde(rename = "titleColor")]
    pub title_color: Option<String>,
    #[serde(rename = "serviceStatus")]
    pub service_status: Option<i32>,
    #[serde(rename = "merId")]
    pub mer_id: Option<i32>,
    #[serde(rename = "isDefault")]
    pub is_default: Option<i32>,
    #[serde(rename = "textPosition")]
    pub text_position: Option<i32>,
}

/// DIY模版名称修改请求
///
/// Java: PageDiyEditNameRequest
#[derive(Debug, Deserialize)]
pub struct PageDiyEditNameRequest {
    pub id: i32,
    pub name: String,
}

/// DIY页面详情响应
///
/// Java: PageDiyResponse
/// value字段返回JSONObject而非String
#[derive(Debug, Serialize)]
pub struct PageDiyResponse {
    pub id: i32,
    pub version: String,
    pub name: String,
    pub title: String,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    #[serde(rename = "templateName")]
    pub template_name: String,
    /// 页面数据 (解析为JSON对象)
    pub value: Option<serde_json::Value>,
    #[serde(rename = "addTime")]
    pub add_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    pub status: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "isShow")]
    pub is_show: i32,
    #[serde(rename = "isBgColor")]
    pub is_bg_color: i32,
    #[serde(rename = "isBgPic")]
    pub is_bg_pic: i32,
    #[serde(rename = "isDiy")]
    pub is_diy: i32,
    #[serde(rename = "colorPicker")]
    pub color_picker: String,
    #[serde(rename = "bgPic")]
    pub bg_pic: String,
    #[serde(rename = "bgTabVal")]
    pub bg_tab_val: i32,
    #[serde(rename = "isDel")]
    pub is_del: i32,
    #[serde(rename = "returnAddress")]
    pub return_address: String,
    #[serde(rename = "titleBgColor")]
    pub title_bg_color: String,
    #[serde(rename = "titleColor")]
    pub title_color: String,
    #[serde(rename = "serviceStatus")]
    pub service_status: i32,
    #[serde(rename = "merId")]
    pub mer_id: i32,
    #[serde(rename = "isDefault")]
    pub is_default: i32,
    #[serde(rename = "textPosition")]
    pub text_position: i32,
}
