/// 砍价管理 DTO
///
/// 对应Java:
/// - StoreBargainSearchRequest
/// - StoreBargainRequest (新增/修改)
/// - StoreBargainResponse (列表响应)
/// - StoreBargainUserSearchRequest
/// - StoreBargainUserResponse
/// - StoreBargainUserHelpResponse
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

// ==================== 砍价商品 ====================

/// 砍价商品搜索请求
#[derive(Debug, Deserialize)]
pub struct StoreBargainSearchRequest {
    pub keywords: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i16>,
}

/// 砍价商品新增/修改请求
#[derive(Debug, Deserialize)]
pub struct StoreBargainRequest {
    pub id: Option<i32>,
    #[serde(rename = "productId")]
    pub product_id: i32,
    pub title: String,
    pub image: String,
    #[serde(rename = "unitName")]
    pub unit_name: String,
    pub images: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "stopTime")]
    pub stop_time: String,
    #[serde(rename = "storeName")]
    pub store_name: Option<String>,
    pub num: i32,
    #[serde(rename = "bargainNum")]
    pub bargain_num: i32,
    pub status: bool,
    #[serde(rename = "tempId")]
    pub temp_id: i32,
    #[serde(rename = "peopleNum")]
    pub people_num: i32,
    /// 商品属性
    pub attr: Option<serde_json::Value>,
    /// 商品属性值
    #[serde(rename = "attrValue")]
    pub attr_value: Option<serde_json::Value>,
    /// 商品详情
    pub content: Option<String>,
    pub sort: Option<i32>,
    pub info: Option<String>,
    pub rule: Option<String>,
}

/// 砍价商品列表响应
#[derive(Debug, Serialize, Clone)]
pub struct StoreBargainResponse {
    pub id: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
    pub title: String,
    pub image: String,
    #[serde(rename = "unitName")]
    pub unit_name: Option<String>,
    pub stock: Option<i32>,
    pub sales: Option<i32>,
    pub images: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "stopTime")]
    pub stop_time: String,
    #[serde(rename = "storeName")]
    pub store_name: Option<String>,
    pub price: Option<Decimal>,
    #[serde(rename = "minPrice")]
    pub min_price: Option<Decimal>,
    pub num: Option<i32>,
    #[serde(rename = "bargainMaxPrice")]
    pub bargain_max_price: Option<Decimal>,
    #[serde(rename = "bargainMinPrice")]
    pub bargain_min_price: Option<Decimal>,
    #[serde(rename = "bargainNum")]
    pub bargain_num: i32,
    pub status: i16,
    #[serde(rename = "giveIntegral")]
    pub give_integral: Option<i32>,
    pub info: Option<String>,
    pub cost: Option<Decimal>,
    pub sort: i32,
    #[serde(rename = "isHot")]
    pub is_hot: bool,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    #[serde(rename = "addTime")]
    pub add_time: Option<String>,
    #[serde(rename = "isPostage")]
    pub is_postage: bool,
    pub postage: Option<Decimal>,
    pub rule: Option<String>,
    pub look: Option<i32>,
    pub share: Option<i32>,
    #[serde(rename = "tempId")]
    pub temp_id: Option<i32>,
    pub weight: Option<Decimal>,
    pub volume: Option<Decimal>,
    pub quota: i32,
    #[serde(rename = "quotaShow")]
    pub quota_show: i32,
    #[serde(rename = "peopleNum")]
    pub people_num: Option<i32>,
    /// 限量剩余
    #[serde(rename = "surplusQuota")]
    pub surplus_quota: i32,
    /// 砍价参与人数
    #[serde(rename = "countPeopleAll")]
    pub count_people_all: i64,
    /// 帮忙砍价人数
    #[serde(rename = "countPeopleHelp")]
    pub count_people_help: i64,
    /// 砍价成功人数
    #[serde(rename = "countPeopleSuccess")]
    pub count_people_success: i64,
}

// ==================== 砍价用户 ====================

/// 砍价用户搜索请求
#[derive(Debug, Deserialize)]
pub struct StoreBargainUserSearchRequest {
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i16>,
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,
}

/// 砍价用户列表响应
#[derive(Debug, Serialize, Clone)]
pub struct StoreBargainUserResponse {
    pub id: i32,
    pub uid: Option<i32>,
    #[serde(rename = "bargainId")]
    pub bargain_id: Option<i32>,
    #[serde(rename = "bargainPriceMin")]
    pub bargain_price_min: Option<Decimal>,
    #[serde(rename = "bargainPrice")]
    pub bargain_price: Option<Decimal>,
    pub price: Option<Decimal>,
    pub status: i16,
    #[serde(rename = "addTime")]
    pub add_time: Option<String>,
    pub avatar: Option<String>,
    #[serde(rename = "dataTime")]
    pub data_time: Option<String>,
    pub nickname: Option<String>,
    /// 当前价格 = bargainPrice - price
    #[serde(rename = "nowPrice")]
    pub now_price: Option<Decimal>,
    /// 剩余砍价次数
    pub num: i32,
    /// 总砍价人数
    #[serde(rename = "peopleNum")]
    pub people_num: Option<i32>,
    pub title: Option<String>,
}

/// 砍价帮助记录响应
#[derive(Debug, Serialize, Clone)]
pub struct StoreBargainUserHelpResponse {
    pub id: i32,
    pub uid: Option<i32>,
    #[serde(rename = "bargainId")]
    pub bargain_id: Option<i32>,
    #[serde(rename = "bargainUserId")]
    pub bargain_user_id: Option<i32>,
    pub price: Option<Decimal>,
    #[serde(rename = "addTime")]
    pub add_time: Option<String>,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
}

/// 更新状态请求
#[derive(Debug, Deserialize)]
pub struct BargainStatusRequest {
    pub id: i32,
    pub status: bool,
}
