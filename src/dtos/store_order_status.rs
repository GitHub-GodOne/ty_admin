use serde::{Deserialize, Serialize};

/// 订单操作记录搜索请求
#[derive(Debug, Deserialize)]
pub struct StoreOrderStatusSearchRequest {
    /// 订单编号
    #[serde(rename = "orderNo")]
    pub order_no: String,
}

/// 订单操作记录响应
#[derive(Debug, Serialize)]
pub struct StoreOrderStatusResponse {
    pub id: i32,
    /// 订单id
    pub oid: i32,
    /// 操作类型
    #[serde(rename = "changeType")]
    pub change_type: String,
    /// 操作备注
    #[serde(rename = "changeMessage")]
    pub change_message: String,
    /// 操作时间
    #[serde(rename = "createTime")]
    pub create_time: String,
}
