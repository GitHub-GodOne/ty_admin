use serde::{Deserialize, Serialize};

/// 快递公司搜索请求
#[derive(Debug, Deserialize)]
pub struct ExpressSearchRequest {
    /// 搜索关键字
    pub keywords: Option<String>,
}

/// 快递公司编辑请求
#[derive(Debug, Deserialize)]
pub struct ExpressUpdateRequest {
    /// 快递公司id
    pub id: i32,

    /// 月结账号
    pub account: Option<String>,

    /// 月结密码
    pub password: Option<String>,

    /// 网点名称
    #[serde(rename = "netName")]
    pub net_name: Option<String>,

    /// 排序
    pub sort: i32,

    /// 是否可用
    pub status: bool,
}

/// 快递公司修改显示状态请求
#[derive(Debug, Deserialize)]
pub struct ExpressUpdateShowRequest {
    /// 快递公司id
    pub id: i32,

    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: bool,
}

/// 查询全部物流公司请求
#[derive(Debug, Deserialize)]
pub struct ExpressAllQuery {
    /// 类型：normal-普通，elec-电子面单
    #[serde(rename = "type")]
    pub express_type: String,
}

/// 查询物流公司面单模板请求
#[derive(Debug, Deserialize)]
pub struct ExpressTemplateQuery {
    /// 快递公司编号
    pub com: String,
}

/// 快递公司响应
#[derive(Debug, Serialize)]
pub struct ExpressResponse {
    pub id: i32,
    /// 快递公司编号
    pub code: String,
    /// 快递公司名称
    pub name: String,
    /// 是否需要月结账号
    #[serde(rename = "partnerId")]
    pub partner_id: bool,
    /// 是否需要月结密码
    #[serde(rename = "partnerKey")]
    pub partner_key: bool,
    /// 是否需要取件网点
    pub net: bool,
    /// 月结账号
    pub account: String,
    /// 月结密码
    pub password: String,
    /// 网点名称
    #[serde(rename = "netName")]
    pub net_name: String,
    /// 排序
    pub sort: i32,
    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: bool,
    /// 是否可用
    pub status: bool,
}
