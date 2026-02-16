use serde::{Deserialize, Serialize};

// ==================== 一号通常量 ====================

/// 一号通API基础URL
pub const ONE_PASS_API_URL: &str = "https://sms.crmeb.net/api/";

/// 一号通token Redis key前缀
pub const ONE_PASS_TOKEN_KEY_PREFIX: &str = "one_pass_token_";

/// 用户token前缀
pub const ONE_PASS_USER_TOKEN_PREFIX: &str = "Bearer-";

/// 接口异常错误码
pub const ONE_PASS_ERROR_CODE: i32 = 400;

/// 用户登录地址
pub const USER_LOGIN_URI: &str = "v2/user/login";

/// 获取账号信息
pub const USER_INFO_URI: &str = "v2/user/info";

/// 商家寄件 - 创建订单
pub const SHIPMENT_CREATE_ORDER_URI: &str = "v2/shipment/create_order";

/// 商家寄件 - 取消订单
pub const SHIPMENT_CANCEL_ORDER_URI: &str = "v2/shipment/cancel_order";

/// 商家寄件 - 获取快递公司列表
pub const SHIPMENT_GET_KUAIDI_COMS_URI: &str = "v2/shipment/get_kuaidi_coms";

/// 商家寄件 - 回调地址后缀
pub const SHIPMENT_CALLBACK_URI: &str = "/shipment/callback";

/// 一号通当前应用 access_key 配置名
pub const ONE_PASS_ACCESS_KEY: &str = "access_key";

/// 一号通当前应用 secret_key 配置名
pub const ONE_PASS_SECRET_KEY: &str = "secret_key";

// ==================== 请求 DTO ====================

/// 一号通应用登录/保存请求
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OnePassLoginRequest {
    /// AccessKey
    #[serde(rename = "accessKey")]
    pub access_key: String,

    /// SecretKey
    #[serde(rename = "secretKey")]
    pub secret_key: String,
}

/// 取消商家寄件请求
#[derive(Debug, Deserialize)]
pub struct OnePassShipmentCancelOrderRequest {
    /// 任务ID
    #[serde(rename = "taskId")]
    pub task_id: String,

    /// 订单ID
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// 取消理由
    #[serde(rename = "cancelMsg")]
    pub cancel_msg: String,
}

/// 商家寄件回调请求
#[derive(Debug, Deserialize)]
pub struct OnePassShipmentCallBackRequest {
    /// 回调类型: order_success, order_take, order_cancel, order_fail, order_receipt
    #[serde(rename = "type")]
    pub callback_type: Option<String>,

    /// 加密的回调数据
    pub data: Option<String>,
}

// ==================== 内部 VO ====================

/// 一号通登录凭证（内部使用）
#[derive(Debug, Clone)]
pub struct OnePassLoginVo {
    pub access_key: String,
    pub secret_key: String,
}
