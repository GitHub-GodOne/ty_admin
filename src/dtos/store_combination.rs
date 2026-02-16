/// 拼团管理 DTO
///
/// 对应Java:
/// - StoreCombinationSearchRequest
/// - StoreCombinationRequest (新增/修改)
/// - StoreCombinationResponse (列表响应)
/// - StorePinkSearchRequest
/// - StorePinkAdminListResponse
/// - StorePinkDetailResponse
use serde::{Deserialize, Deserializer, Serialize};
use rust_decimal::Decimal;

/// 空字符串反序列化为 None
fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => s.trim().parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
}

// ==================== 拼团商品 ====================

/// 拼团商品搜索请求
///
/// Java: StoreCombinationSearchRequest
#[derive(Debug, Deserialize)]
pub struct StoreCombinationSearchRequest {
    pub keywords: Option<String>,
    #[serde(rename = "isShow", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub is_show: Option<i16>,
}

/// 拼团商品新增/修改请求
///
/// Java: StoreCombinationRequest
#[derive(Debug, Deserialize)]
pub struct StoreCombinationRequest {
    pub id: Option<i32>,
    #[serde(rename = "productId")]
    pub product_id: i32,
    pub image: String,
    pub images: String,
    pub title: String,
    /// 拼团人数
    pub people: i32,
    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: bool,
    /// 活动开始时间
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// 活动结束时间
    #[serde(rename = "stopTime")]
    pub stop_time: String,
    /// 拼团有效时间(小时)
    #[serde(rename = "effectiveTime")]
    pub effective_time: i32,
    /// 单位名
    #[serde(rename = "unitName")]
    pub unit_name: String,
    /// 运费模板id
    #[serde(rename = "tempId")]
    pub temp_id: i32,
    /// 限购总数
    pub num: Option<i32>,
    /// 单次限购
    #[serde(rename = "onceNum")]
    pub once_num: Option<i32>,
    /// 虚拟成团比例
    #[serde(rename = "virtualRation")]
    pub virtual_ration: Option<i32>,
    /// 商品属性
    pub attr: Option<serde_json::Value>,
    /// 商品属性值
    #[serde(rename = "attrValue")]
    pub attr_value: Option<serde_json::Value>,
    /// 商品详情
    pub content: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 规格类型: 0=单规格, 1=多规格
    #[serde(rename = "specType")]
    pub spec_type: Option<i16>,
}

/// 拼团商品列表响应
///
/// Java: StoreCombinationResponse
#[derive(Debug, Serialize, Clone)]
pub struct StoreCombinationResponse {
    pub id: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
    pub image: String,
    pub images: String,
    pub title: String,
    pub price: Decimal,
    pub cost: Decimal,
    #[serde(rename = "otPrice")]
    pub ot_price: Decimal,
    pub sort: i32,
    pub stock: i32,
    pub sales: i32,
    #[serde(rename = "unitName")]
    pub unit_name: String,
    pub postage: Option<Decimal>,
    #[serde(rename = "isPostage")]
    pub is_postage: bool,
    #[serde(rename = "isShow")]
    pub is_show: bool,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    pub people: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "stopTime")]
    pub stop_time: String,
    #[serde(rename = "effectiveTime")]
    pub effective_time: i32,
    #[serde(rename = "addTime")]
    pub add_time: String,
    pub info: String,
    #[serde(rename = "tempId")]
    pub temp_id: i32,
    pub quota: i32,
    #[serde(rename = "quotaShow")]
    pub quota_show: i32,
    #[serde(rename = "onceNum")]
    pub once_num: i32,
    #[serde(rename = "virtualRation")]
    pub virtual_ration: i32,
    pub weight: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub num: Option<i32>,
    /// 参团人数（所有参与者）
    #[serde(rename = "countPeopleAll")]
    pub count_people_all: i64,
    /// 成团数量（status=2的团长数）
    #[serde(rename = "countPeoplePink")]
    pub count_people_pink: i64,
    /// 开团数量（kId=0的团长数）
    #[serde(rename = "countPeople")]
    pub count_people: i64,
    /// 限量剩余
    #[serde(rename = "remainingQuota")]
    pub remaining_quota: i32,
    /// 结束时间字符串
    #[serde(rename = "stopTimeStr")]
    pub stop_time_str: String,
}

/// 更新拼团状态请求
#[derive(Debug, Deserialize)]
pub struct CombinationStatusRequest {
    pub id: i32,
    #[serde(rename = "isShow")]
    pub is_show: bool,
}

/// 拼团统计响应
#[derive(Debug, Serialize)]
pub struct CombinationStatisticsResponse {
    /// 参与人数
    #[serde(rename = "countPeople")]
    pub count_people: i64,
    /// 成团数量
    #[serde(rename = "countPeoplePink")]
    pub count_people_pink: i64,
}

// ==================== 拼团记录(StorePink) ====================

/// 拼团记录搜索请求
///
/// Java: StorePinkSearchRequest
#[derive(Debug, Deserialize)]
pub struct StorePinkSearchRequest {
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i16>,
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,
}

/// 拼团记录列表响应
///
/// Java: StorePinkAdminListResponse
#[derive(Debug, Serialize, Clone)]
pub struct StorePinkAdminListResponse {
    pub id: i32,
    pub uid: i32,
    pub people: i32,
    #[serde(rename = "addTime")]
    pub add_time: String,
    #[serde(rename = "stopTime")]
    pub stop_time: String,
    #[serde(rename = "kId")]
    pub k_id: i32,
    pub status: i16,
    pub nickname: String,
    pub avatar: String,
    /// 参团人数
    #[serde(rename = "countPeople")]
    pub count_people: i64,
    /// 拼团商品标题
    pub title: String,
}

/// 拼团订单详情响应
///
/// Java: StorePinkDetailResponse
#[derive(Debug, Serialize, Clone)]
pub struct StorePinkDetailResponse {
    pub id: i32,
    pub uid: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "totalPrice")]
    pub total_price: Decimal,
    pub nickname: String,
    pub avatar: String,
    /// 订单状态
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    /// 退款状态
    #[serde(rename = "refundStatus")]
    pub refund_status: String,
}
