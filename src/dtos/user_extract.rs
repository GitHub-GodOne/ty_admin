/// 用户提现 DTO
///
/// Java参考:
/// - UserExtractSearchRequest
/// - UserExtractRequest
/// - BalanceResponse (提现统计)
use serde::{Deserialize, Deserializer, Serialize};
use rust_decimal::Decimal;

/// 空字符串反序列化为None (i32)
fn deserialize_empty_string_as_none_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
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

/// 提现搜索请求
///
/// Java: UserExtractSearchRequest
#[derive(Debug, Deserialize)]
pub struct UserExtractSearchRequest {
    /// 搜索关键字（微信号/姓名/银行卡/开户行/支付宝/失败原因）
    pub keywords: Option<String>,

    /// 提现方式: bank=银行卡, alipay=支付宝, weixin=微信
    #[serde(rename = "extractType")]
    pub extract_type: Option<String>,

    /// 提现状态: -1=未通过, 0=审核中, 1=已提现
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none_i32")]
    pub status: Option<i32>,

    /// 时间区间: today,yesterday,lately7,lately30,month,year,/自定义/
    #[serde(rename = "dateLimit")]
    pub date_limit: Option<String>,
}

/// 提现审核请求参数
///
/// Java: UserExtractController.updateStatus() @RequestParam
#[derive(Debug, Deserialize)]
pub struct ExtractApplyRequest {
    /// 提现申请id
    pub id: i32,
    /// 审核状态: -1=未通过, 0=审核中, 1=已提现
    pub status: i32,
    /// 驳回原因
    #[serde(rename = "backMessage", default)]
    pub back_message: Option<String>,
}

/// 修改提现申请请求
///
/// Java: UserExtractRequest (用于update接口)
#[derive(Debug, Deserialize)]
pub struct UserExtractUpdateRequest {
    /// 申请id (query param)
    pub id: i32,
    /// 姓名
    #[serde(rename = "name")]
    pub real_name: Option<String>,
    /// 提现方式: alipay/bank/weixin
    #[serde(rename = "extractType")]
    pub extract_type: Option<String>,
    /// 银行卡号
    #[serde(rename = "cardum")]
    pub bank_code: Option<String>,
    /// 银行名称
    #[serde(rename = "bankName")]
    pub bank_name: Option<String>,
    /// 支付宝账号
    #[serde(rename = "alipayCode")]
    pub alipay_code: Option<String>,
    /// 提现金额
    #[serde(rename = "money")]
    pub extract_price: Option<Decimal>,
    /// 微信号
    pub wechat: Option<String>,
    /// 备注
    pub mark: Option<String>,
    /// 微信收款码
    #[serde(rename = "qrcodeUrl")]
    pub qrcode_url: Option<String>,
}

/// 提现统计响应
///
/// Java: BalanceResponse
#[derive(Debug, Serialize)]
pub struct ExtractBalanceResponse {
    /// 已提现
    pub withdrawn: Decimal,
    /// 未提现
    #[serde(rename = "unDrawn")]
    pub un_drawn: Decimal,
    /// 佣金总金额
    #[serde(rename = "commissionTotal")]
    pub commission_total: Decimal,
    /// 待提现(审核中)
    #[serde(rename = "ToBeWithdrawn")]
    pub to_be_withdrawn: Decimal,
}
