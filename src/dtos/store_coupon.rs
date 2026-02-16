/// 优惠券管理 DTO
///
/// Java参考:
/// - StoreCouponSearchRequest (列表搜索)
/// - StoreCouponRequest (新增/修改)
/// - StoreCouponInfoResponse (详情响应)
/// - SearchAndPageRequest (发送列表搜索)
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

// ==================== 搜索请求 ====================

/// 优惠券列表搜索请求
/// Java: StoreCouponSearchRequest
#[derive(Debug, Deserialize)]
pub struct StoreCouponSearchRequest {
    pub name: Option<String>,
    #[serde(rename = "type", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub coupon_type: Option<i32>,
    #[serde(rename = "useType", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub use_type: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i32>,
}

/// 发送优惠券列表搜索请求
/// Java: SearchAndPageRequest
#[derive(Debug, Deserialize)]
pub struct CouponSendListRequest {
    pub keywords: Option<String>,
    #[serde(rename = "type", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub coupon_type: Option<i32>,
}

// ==================== 新增/修改请求 ====================

/// 优惠券新增请求
/// Java: StoreCouponRequest
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreCouponRequest {
    pub id: Option<i32>,
    pub name: String,
    pub money: Decimal,
    /// 是否限量: true=限量, false=不限量
    #[serde(rename = "isLimited")]
    pub is_limited: bool,
    /// 发放总量
    pub total: Option<i32>,
    /// 使用类型: 1=通用, 2=商品, 3=品类
    #[serde(rename = "useType")]
    pub use_type: i32,
    /// 关联ID (useType>1时必填, 逗号分隔)
    #[serde(rename = "primaryKey")]
    pub primary_key: Option<String>,
    /// 最低消费金额
    #[serde(rename = "minPrice")]
    pub min_price: Decimal,
    /// 是否有固定领取时间: true=有, false=不限时
    #[serde(rename = "isForever")]
    pub is_forever: Option<bool>,
    /// 领取开始时间
    #[serde(rename = "receiveStartTime")]
    pub receive_start_time: Option<String>,
    /// 领取结束时间
    #[serde(rename = "receiveEndTime")]
    pub receive_end_time: Option<String>,
    /// 是否固定使用时间: true=固定, false=领取后N天
    #[serde(rename = "isFixedTime")]
    pub is_fixed_time: Option<bool>,
    /// 使用开始时间
    #[serde(rename = "useStartTime")]
    pub use_start_time: Option<String>,
    /// 使用结束时间
    #[serde(rename = "useEndTime")]
    pub use_end_time: Option<String>,
    /// 天数 (非固定时间时使用)
    pub day: Option<i32>,
    /// 优惠券类型: 1=手动领取, 2=新人券, 3=赠送券
    #[serde(rename = "type")]
    pub coupon_type: i32,
    pub sort: Option<i32>,
    pub status: Option<bool>,
}

// ==================== 响应 ====================

/// 优惠券列表响应
#[derive(Debug, Serialize, Clone)]
pub struct StoreCouponResponse {
    pub id: i32,
    pub name: String,
    pub money: Decimal,
    #[serde(rename = "isLimited")]
    pub is_limited: bool,
    pub total: i32,
    #[serde(rename = "lastTotal")]
    pub last_total: i32,
    #[serde(rename = "useType")]
    pub use_type: i32,
    #[serde(rename = "primaryKey")]
    pub primary_key: String,
    #[serde(rename = "minPrice")]
    pub min_price: Decimal,
    #[serde(rename = "receiveStartTime")]
    pub receive_start_time: Option<String>,
    #[serde(rename = "receiveEndTime")]
    pub receive_end_time: Option<String>,
    #[serde(rename = "isFixedTime")]
    pub is_fixed_time: bool,
    #[serde(rename = "useStartTime")]
    pub use_start_time: Option<String>,
    #[serde(rename = "useEndTime")]
    pub use_end_time: Option<String>,
    pub day: i32,
    #[serde(rename = "type")]
    pub coupon_type: i32,
    pub sort: i32,
    pub status: bool,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

/// 发送优惠券列表响应 (精简字段)
#[derive(Debug, Serialize, Clone)]
pub struct StoreCouponSendResponse {
    pub id: i32,
    pub name: String,
    pub money: Decimal,
    #[serde(rename = "isLimited")]
    pub is_limited: bool,
    pub total: i32,
    #[serde(rename = "lastTotal")]
    pub last_total: i32,
    #[serde(rename = "useType")]
    pub use_type: i32,
    #[serde(rename = "minPrice")]
    pub min_price: Decimal,
    #[serde(rename = "type")]
    pub coupon_type: i32,
}

/// 优惠券详情响应
/// Java: StoreCouponInfoResponse
#[derive(Debug, Serialize)]
pub struct StoreCouponInfoResponse {
    pub coupon: StoreCouponRequest,
    pub product: Vec<serde_json::Value>,
    pub category: Vec<serde_json::Value>,
}

/// 更新状态请求
#[derive(Debug, Deserialize)]
pub struct CouponStatusRequest {
    pub id: i32,
    pub status: bool,
}
