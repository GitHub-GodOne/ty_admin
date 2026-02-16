/// 优惠券领取记录 DTO
///
/// Java参考:
/// - StoreCouponUserSearchRequest (列表搜索)
/// - StoreCouponUserRequest (发放/领取)
/// - StoreCouponUserResponse (列表响应)
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

/// 优惠券领取记录搜索请求
#[derive(Debug, Deserialize)]
pub struct StoreCouponUserSearchRequest {
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub uid: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i32>,
    #[serde(rename = "couponId", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub coupon_id: Option<i32>,
}

// ==================== 发放请求 ====================

/// 发放优惠券请求 (管理员给用户发券)
#[derive(Debug, Deserialize)]
pub struct StoreCouponUserRequest {
    /// 优惠券ID
    #[serde(rename = "couponId")]
    pub coupon_id: i32,
    /// 用户ID列表 (逗号分隔, 如 "1,2,3")
    pub uid: String,
}

// ==================== 响应 ====================

/// 优惠券领取记录响应
#[derive(Debug, Serialize, Clone)]
pub struct StoreCouponUserResponse {
    pub id: i32,
    #[serde(rename = "couponId")]
    pub coupon_id: i32,
    pub cid: i32,
    pub uid: i32,
    pub name: String,
    pub money: Decimal,
    #[serde(rename = "minPrice")]
    pub min_price: Decimal,
    #[serde(rename = "type")]
    pub coupon_type: String,
    pub status: i32,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "useTime")]
    pub use_time: Option<String>,
    #[serde(rename = "useType")]
    pub use_type: i32,
    /// 用户昵称
    pub nickname: Option<String>,
    /// 用户头像
    pub avatar: Option<String>,
}
