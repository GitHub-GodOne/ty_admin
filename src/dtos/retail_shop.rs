/// 分销管理 DTO
///
/// Java参考:
/// - RetailShopRequest (分销配置)
/// - RetailShopStairUserRequest (推广等级查询)
/// - SpreadUserResponse (分销员列表响应)
/// - SpreadOrderResponse (推广订单响应)
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

// ==================== 分销员列表搜索 ====================

/// 分销员列表搜索请求
#[derive(Debug, Deserialize)]
pub struct RetailShopListRequest {
    pub keywords: Option<String>,
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,
}

// ==================== 推广等级查询 ====================

/// 推广人列表/推广订单列表查询请求
#[derive(Debug, Deserialize)]
pub struct RetailShopStairUserRequest {
    /// 搜索关键字
    #[serde(rename = "nickName")]
    pub nick_name: Option<String>,
    /// 时间参数
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,
    /// 类型 0=全部 1=一级推广人 2=二级推广人
    #[serde(rename = "type")]
    pub spread_type: i32,
    /// 用户id
    pub uid: i32,
}

// ==================== 清除推广关系 ====================

/// 清除推广关系请求（使用query参数代替path参数）
#[derive(Debug, Deserialize)]
pub struct CleanSpreadRequest {
    pub id: i32,
}

// ==================== 分销配置 ====================

/// 分销配置请求/响应
#[derive(Debug, Deserialize, Serialize)]
pub struct RetailShopConfigRequest {
    /// 是否启用分销:1-启用，0-禁止
    #[serde(rename = "brokerageFuncStatus")]
    pub brokerage_func_status: i32,
    /// 分销额度：-1-关闭，0--用户购买金额大于等于设置金额时自动成为分销员
    #[serde(rename = "storeBrokerageQuota")]
    pub store_brokerage_quota: i32,
    /// 一级返佣比例
    #[serde(rename = "storeBrokerageRatio")]
    pub store_brokerage_ratio: i32,
    /// 二级返佣比例
    #[serde(rename = "storeBrokerageTwo")]
    pub store_brokerage_two: i32,
    /// 分销关系绑定:0-所有用户，1-新用户
    #[serde(rename = "brokerageBindind")]
    pub brokerage_bindind: i32,
    /// 用户提现最低金额
    #[serde(rename = "userExtractMinPrice")]
    pub user_extract_min_price: Decimal,
    /// 提现银行
    #[serde(rename = "userExtractBank")]
    pub user_extract_bank: String,
    /// 冻结时间
    #[serde(rename = "extractTime")]
    pub extract_time: i32,
    /// 是否展示分销气泡：0-不展示，1-展示
    #[serde(rename = "storeBrokerageIsBubble")]
    pub store_brokerage_is_bubble: i32,
}

// ==================== 分销员列表响应 ====================

/// 分销员列表响应
#[derive(Debug, Serialize)]
pub struct SpreadUserResponse {
    pub uid: i32,
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
    /// 佣金金额(未提现金额)
    #[serde(rename = "brokeragePrice")]
    pub brokerage_price: Option<Decimal>,
    /// 推广人id
    #[serde(rename = "spreadUid")]
    pub spread_uid: Option<i32>,
    /// 上级推广员名称
    #[serde(rename = "spreadNickname")]
    pub spread_nickname: String,
    /// 用户购买次数
    #[serde(rename = "payCount")]
    pub pay_count: Option<i32>,
    /// 推广用户数
    #[serde(rename = "spreadCount")]
    pub spread_count: Option<i32>,
    /// 推广订单数
    #[serde(rename = "spreadOrderNum")]
    pub spread_order_num: i32,
    /// 推广订单额
    #[serde(rename = "spreadOrderTotalPrice")]
    pub spread_order_total_price: Decimal,
    /// 佣金总金额
    #[serde(rename = "totalBrokeragePrice")]
    pub total_brokerage_price: Decimal,
    /// 已提现金额
    #[serde(rename = "extractCountPrice")]
    pub extract_count_price: Decimal,
    /// 已提现次数
    #[serde(rename = "extractCountNum")]
    pub extract_count_num: i32,
    /// 冻结佣金
    #[serde(rename = "freezeBrokeragePrice")]
    pub freeze_brokerage_price: Decimal,
    /// 成为分销员时间
    #[serde(rename = "promoterTime")]
    pub promoter_time: Option<String>,
}

// ==================== 推广人用户响应 ====================

/// 推广人用户列表响应
#[derive(Debug, Serialize)]
pub struct SpreadLevelUserResponse {
    pub uid: i32,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    #[serde(rename = "isPromoter")]
    pub is_promoter: bool,
    #[serde(rename = "spreadCount")]
    pub spread_count: Option<i32>,
    #[serde(rename = "payCount")]
    pub pay_count: Option<i32>,
}

// ==================== 推广订单响应 ====================

/// 推广订单响应
#[derive(Debug, Serialize)]
pub struct SpreadOrderResponse {
    pub id: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "realName")]
    pub real_name: String,
    #[serde(rename = "userPhone")]
    pub user_phone: String,
    pub price: Decimal,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}
