/// 易联云打印 DTO
///
/// Java参考: YlyPrintRequest, YlyPrintRequestGoods, YlyAccessTokenResponse
use serde::{Deserialize, Serialize};

/// 打印请求 - 控制器入参
/// Java: GET /api/admin/yly/print/{ordid} → 改为 query param
#[derive(Debug, Deserialize)]
pub struct YlyPrintQueryRequest {
    /// 订单号
    pub ordid: String,
}

/// 打印请求数据（内部使用）
/// Java: YlyPrintRequest
#[derive(Debug, Clone, Serialize)]
pub struct YlyPrintRequest {
    pub business_name: String,
    pub order_no: String,
    pub date: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub note: String,
    pub goods: Vec<YlyPrintRequestGoods>,
    pub amount: String,
    pub discount: String,
    pub postal: String,
    pub deduction: String,
    pub pay_money: String,
}

/// 打印商品项（内部使用）
/// Java: YlyPrintRequestGoods
#[derive(Debug, Clone, Serialize)]
pub struct YlyPrintRequestGoods {
    pub goods_name: String,
    pub unit_price: String,
    pub num: String,
    pub money: String,
}

impl YlyPrintRequestGoods {
    /// 创建商品项，商品名称截断到10个字符（与Java一致）
    pub fn new(goods_name: &str, unit_price: String, num: String, money: String) -> Self {
        let truncated_name: String = goods_name.chars().take(10).collect();
        Self {
            goods_name: truncated_name,
            unit_price,
            num,
            money,
        }
    }
}

/// 易联云AccessToken响应
/// Java: YlyAccessTokenResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YlyAccessTokenResponse {
    pub error: String,
    pub error_description: String,
    pub body: Option<YlyAccessTokenBody>,
}

/// 易联云AccessToken响应体
/// Java: YlyAccessTokenBodyResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YlyAccessTokenBody {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub machine_code: Option<String>,
    pub expires_in: Option<i64>,
    pub scope: Option<String>,
}
