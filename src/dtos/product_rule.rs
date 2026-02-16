use serde::{Deserialize, Serialize};

/// 商品规格搜索请求
/// Java: StoreProductRuleSearchRequest
#[derive(Debug, Deserialize)]
pub struct StoreProductRuleSearchRequest {
    /// 搜索关键字
    pub keywords: Option<String>,
}

/// 商品规格新增/编辑请求
/// Java: StoreProductRuleRequest
#[derive(Debug, Deserialize)]
pub struct StoreProductRuleRequest {
    /// 规则id（编辑时必填）
    pub id: Option<i32>,

    /// 规格名称
    #[serde(rename = "ruleName")]
    pub rule_name: String,

    /// 规格值 JSON字符串 [{"detail": ["string"],"title": "string"}]
    #[serde(rename = "ruleValue")]
    pub rule_value: String,
}

/// 商品规格响应
/// Java: StoreProductRule
#[derive(Debug, Serialize)]
pub struct StoreProductRuleResponse {
    pub id: i32,
    #[serde(rename = "ruleName")]
    pub rule_name: String,
    #[serde(rename = "ruleValue")]
    pub rule_value: String,
}
