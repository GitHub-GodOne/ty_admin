/// 用户充值 DTO
///
/// Java参考:
/// - UserRechargeSearchRequest
/// - UserRechargeResponse
use serde::{Deserialize, Deserializer, Serialize};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// 空字符串反序列化为None
fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => s.parse::<i32>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

/// 充值搜索请求
///
/// Java: UserRechargeSearchRequest
#[derive(Debug, Deserialize)]
pub struct UserRechargeSearchRequest {
    /// 搜索关键字（订单号模糊搜索）
    pub keywords: Option<String>,

    /// 时间区间 格式: "2024-01-01,2024-01-31"
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,

    /// 用户uid
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub uid: Option<i32>,
}

/// 充值记录响应
///
/// Java: UserRechargeResponse
#[derive(Debug, Serialize)]
pub struct UserRechargeResponse {
    pub id: i32,
    pub uid: Option<i32>,
    #[serde(rename = "orderId")]
    pub order_id: Option<String>,
    pub price: Option<Decimal>,
    #[serde(rename = "givePrice")]
    pub give_price: Decimal,
    #[serde(rename = "rechargeType")]
    pub recharge_type: Option<String>,
    pub paid: Option<i32>,
    #[serde(rename = "payTime")]
    pub pay_time: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "refundPrice")]
    pub refund_price: Option<Decimal>,
    /// 用户头像
    pub avatar: Option<String>,
    /// 用户昵称
    pub nickname: Option<String>,
}

/// 充值余额统计响应
///
/// Java: HashMap<String, BigDecimal>
pub type BalanceResponse = HashMap<String, Decimal>;
