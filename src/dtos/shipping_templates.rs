use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        other => Ok(other),
    }
}

// ==================== 请求 DTO ====================

/// 运费模板搜索请求
/// Java: ShippingTemplatesSearchRequest
#[derive(Debug, Deserialize)]
pub struct ShippingTemplatesSearchRequest {
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub keywords: Option<String>,
}

/// 运费模板新增/修改请求
/// Java: ShippingTemplatesRequest
#[derive(Debug, Deserialize)]
pub struct ShippingTemplatesRequest {
    /// 模板名称
    pub name: String,

    /// 计费方式 0未选择 1按件数 2按重量 3按体积
    #[serde(rename = "type")]
    pub template_type: i32,

    /// 包邮类型 0全国包邮 1部分包邮 2自定义
    pub appoint: i32,

    /// 排序
    pub sort: i32,

    /// 配送区域及运费
    #[serde(rename = "shippingTemplatesRegionRequestList", default)]
    pub region_list: Vec<ShippingTemplatesRegionRequest>,

    /// 包邮区域设置
    #[serde(rename = "shippingTemplatesFreeRequestList", default)]
    pub free_list: Vec<ShippingTemplatesFreeRequest>,
}

/// 运费模板区域请求
/// Java: ShippingTemplatesRegionRequest
#[derive(Debug, Deserialize, Clone)]
pub struct ShippingTemplatesRegionRequest {
    /// 城市ID, 多个逗号分割。全国 0 或 all
    #[serde(rename = "cityId")]
    pub city_id: String,

    /// 城市名称描述 (JSON数组字符串)
    pub title: String,

    /// 首件
    pub first: Decimal,

    /// 首件运费
    #[serde(rename = "firstPrice")]
    pub first_price: Decimal,

    /// 续件
    pub renewal: Decimal,

    /// 续件运费
    #[serde(rename = "renewalPrice")]
    pub renewal_price: Decimal,

    /// 分组唯一值
    pub uniqid: Option<String>,
}

/// 免费模板请求
/// Java: ShippingTemplatesFreeRequest
#[derive(Debug, Deserialize, Clone)]
pub struct ShippingTemplatesFreeRequest {
    /// 城市ID, 多个逗号分割。全国 all
    #[serde(rename = "cityId")]
    pub city_id: String,

    /// 城市名称描述 (JSON数组字符串)
    pub title: String,

    /// 包邮件数
    pub number: Decimal,

    /// 包邮金额
    pub price: Decimal,
}

/// 模板id查询参数
#[derive(Debug, Deserialize)]
pub struct TempIdQuery {
    #[serde(rename = "tempId")]
    pub temp_id: i32,
}

// ==================== 响应 DTO ====================

/// 运费模板列表响应（直接返回实体）
/// Java: ShippingTemplates
#[derive(Debug, Serialize)]
pub struct ShippingTemplatesListResponse {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub template_type: i32,
    pub appoint: i32,
    pub sort: i32,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

/// 运费模板详情响应
/// Java: ShippingTemplatesInfoResponse
#[derive(Debug, Serialize)]
pub struct ShippingTemplatesInfoResponse {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub template_type: i32,
    pub appoint: i32,
    pub sort: i32,
    #[serde(rename = "regionList")]
    pub region_list: Vec<ShippingTemplatesRegionResponse>,
    #[serde(rename = "freeList")]
    pub free_list: Vec<ShippingTemplatesFreeResponse>,
}

/// 运费模板区域响应
/// Java: ShippingTemplatesRegionResponse
#[derive(Debug, Serialize)]
pub struct ShippingTemplatesRegionResponse {
    pub title: String,
    pub first: Decimal,
    #[serde(rename = "firstPrice")]
    pub first_price: Decimal,
    pub renewal: Decimal,
    #[serde(rename = "renewalPrice")]
    pub renewal_price: Decimal,
    pub uniqid: String,
}

/// 运费模板包邮响应
/// Java: ShippingTemplatesFreeResponse
#[derive(Debug, Serialize)]
pub struct ShippingTemplatesFreeResponse {
    pub title: String,
    pub number: Decimal,
    pub price: Decimal,
    pub uniqid: String,
}
