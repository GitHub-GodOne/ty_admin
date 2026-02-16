/// 资金监控 DTO
///
/// Java参考:
/// - FundsMonitorRequest
/// - BrokerageRecordRequest
/// - MonitorResponse
/// - UserBillResponse
use serde::{Deserialize, Deserializer, Serialize};
use rust_decimal::Decimal;

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

/// 资金监控搜索请求
///
/// Java: FundsMonitorRequest
#[derive(Debug, Deserialize)]
pub struct FundsMonitorRequest {
    /// 搜索关键字（uid或昵称）
    pub keywords: Option<String>,

    /// 时间区间 格式: "2024-01-01,2024-01-31"
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,

    /// 类型: recharge/admin/productRefund/payProduct
    pub title: Option<String>,
}

/// 佣金记录搜索请求
///
/// Java: BrokerageRecordRequest
#[derive(Debug, Deserialize)]
pub struct BrokerageRecordRequest {
    /// 类型: 1=订单返佣, 2=申请提现, 3=提现失败, 4=提现成功, 5=佣金转余额
    #[serde(rename = "type", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub record_type: Option<i32>,
}

/// 资金监控响应
///
/// Java: MonitorResponse
#[derive(Debug, Serialize)]
pub struct MonitorResponse {
    pub id: i32,
    pub uid: i32,
    pub pm: i16,
    pub title: String,
    pub number: Decimal,
    pub mark: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
}

/// 佣金记录响应
///
/// Java: UserBrokerageRecord + userName
#[derive(Debug, Serialize)]
pub struct BrokerageRecordResponse {
    pub id: i32,
    pub uid: i32,
    #[serde(rename = "linkId")]
    pub link_id: String,
    #[serde(rename = "linkType")]
    pub link_type: String,
    #[serde(rename = "type")]
    pub record_type: i32,
    pub title: String,
    pub price: Decimal,
    pub balance: Decimal,
    pub mark: String,
    pub status: i32,
    #[serde(rename = "frozenTime")]
    pub frozen_time: i32,
    #[serde(rename = "thawTime")]
    pub thaw_time: i64,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    #[serde(rename = "brokerageLevel")]
    pub brokerage_level: Option<i32>,
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
}
