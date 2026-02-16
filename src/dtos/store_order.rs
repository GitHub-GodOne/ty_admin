use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::common::pagination::PageResponse;

// ==================== 自定义反序列化 ====================

fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        other => Ok(other),
    }
}

fn deserialize_optional_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        Some(ref v) => Ok(v.parse::<i32>().ok()),
        None => Ok(None),
    }
}

// ==================== 请求 DTO ====================

/// 订单列表搜索请求
#[derive(Debug, Deserialize)]
pub struct StoreOrderSearchRequest {
    /// 订单号
    #[serde(default, rename = "orderNo", deserialize_with = "deserialize_empty_string_as_none")]
    pub order_no: Option<String>,

    /// 创建时间区间
    #[serde(default, rename = "dateLimit", deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,

    /// 订单状态
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<String>,

    /// 订单类型：0普通订单，1视频号订单，2全部订单
    #[serde(default, rename = "type", deserialize_with = "deserialize_optional_i32")]
    pub order_type: Option<i32>,
}

/// 订单状态数量查询参数
#[derive(Debug, Deserialize)]
pub struct OrderStatusNumQuery {
    /// 时间区间
    #[serde(default, rename = "dateLimit", deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,

    /// 订单类型：0普通，1视频号，2全部
    #[serde(default, rename = "type", deserialize_with = "deserialize_optional_i32")]
    pub order_type: Option<i32>,

    /// 订单号
    #[serde(default, rename = "orderId", deserialize_with = "deserialize_empty_string_as_none")]
    pub order_no: Option<String>,
}

/// 订单统计数据查询参数
#[derive(Debug, Deserialize)]
pub struct OrderDataQuery {
    #[serde(default, rename = "dateLimit", deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,
}

/// 订单删除/详情查询参数
#[derive(Debug, Deserialize)]
pub struct OrderNoQuery {
    #[serde(rename = "orderNo")]
    pub order_no: String,
}

/// 订单备注请求
#[derive(Debug, Deserialize)]
pub struct OrderMarkRequest {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    pub mark: String,
}

/// 订单改价请求
#[derive(Debug, Deserialize)]
pub struct StoreOrderUpdatePriceRequest {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    #[serde(rename = "payPrice")]
    pub pay_price: Decimal,
}

/// 订单发货请求
#[derive(Debug, Deserialize)]
pub struct StoreOrderSendRequest {
    pub id: Option<i32>,
    #[serde(rename = "orderNo")]
    pub order_no: String,
    /// 类型: express发货, send送货, fictitious虚拟
    #[serde(rename = "deliveryType")]
    pub delivery_type: String,
    /// 快递公司名
    #[serde(rename = "expressName")]
    pub express_name: Option<String>,
    /// 快递公司编码
    #[serde(rename = "expressCode")]
    pub express_code: Option<String>,
    /// 快递单号
    #[serde(rename = "expressNumber")]
    pub express_number: Option<String>,
    /// 发货记录类型 1快递 2电子面单 3一号通
    #[serde(rename = "expressRecordType")]
    pub express_record_type: Option<String>,
    /// 电子面单模板
    #[serde(rename = "expressTempId")]
    pub express_temp_id: Option<String>,
    /// 寄件人姓名
    #[serde(rename = "toName")]
    pub to_name: Option<String>,
    /// 寄件人电话
    #[serde(rename = "toTel")]
    pub to_tel: Option<String>,
    /// 寄件人地址
    #[serde(rename = "toAddr")]
    pub to_addr: Option<String>,
    /// 送货人姓名
    #[serde(rename = "deliveryName")]
    pub delivery_name: Option<String>,
    /// 送货人电话
    #[serde(rename = "deliveryTel")]
    pub delivery_tel: Option<String>,
}

/// 订单退款请求
#[derive(Debug, Deserialize)]
pub struct StoreOrderRefundRequest {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    pub amount: Option<Decimal>,
    #[serde(rename = "orderId")]
    pub order_id: Option<i32>,
}

/// 拒绝退款请求参数
#[derive(Debug, Deserialize)]
pub struct RefundRefuseQuery {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    pub reason: String,
}

/// 核销订单统计请求
#[derive(Debug, Deserialize)]
pub struct StoreOrderStatisticsRequest {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default, rename = "dateLimit", deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 10 }

/// 订单统计详情查询参数
#[derive(Debug, Deserialize)]
pub struct OrderTimeQuery {
    #[serde(rename = "dateLimit")]
    pub date_limit: String,
    #[serde(rename = "type")]
    pub stat_type: i32,
}

/// 核销码查询参数
#[derive(Debug, Deserialize)]
pub struct VerifyCodeQuery {
    #[serde(rename = "vCode")]
    pub v_code: String,
}

// ==================== 响应 DTO ====================

/// 订单列表响应（PC列表用）
#[derive(Debug, Serialize)]
pub struct StoreOrderDetailResponse {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "payPrice")]
    pub pay_price: Decimal,
    #[serde(rename = "payType")]
    pub pay_type: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    pub status: i32,
    #[serde(rename = "productList")]
    pub product_list: Vec<StoreOrderInfoOldVo>,
    #[serde(rename = "statusStr")]
    pub status_str: HashMap<String, String>,
    #[serde(rename = "payTypeStr")]
    pub pay_type_str: String,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    #[serde(rename = "refundReasonWapImg")]
    pub refund_reason_wap_img: Option<String>,
    #[serde(rename = "refundReasonWapExplain")]
    pub refund_reason_wap_explain: Option<String>,
    #[serde(rename = "refundReasonTime")]
    pub refund_reason_time: Option<String>,
    #[serde(rename = "refundReasonWap")]
    pub refund_reason_wap: Option<String>,
    #[serde(rename = "refundReason")]
    pub refund_reason: Option<String>,
    #[serde(rename = "refundPrice")]
    pub refund_price: Decimal,
    #[serde(rename = "refundStatus")]
    pub refund_status: i32,
    #[serde(rename = "verifyCode")]
    pub verify_code: String,
    #[serde(rename = "orderType")]
    pub order_type: String,
    pub remark: Option<String>,
    #[serde(rename = "realName")]
    pub real_name: String,
    #[serde(rename = "proTotalPrice")]
    pub pro_total_price: Decimal,
    #[serde(rename = "couponPrice")]
    pub coupon_price: Decimal,
    #[serde(rename = "beforePayPrice")]
    pub before_pay_price: Decimal,
    pub paid: bool,
    #[serde(rename = "type")]
    pub order_type_num: i32,
    #[serde(rename = "isAlterPrice")]
    pub is_alter_price: bool,
    #[serde(rename = "shipmentPic")]
    pub shipment_pic: Option<String>,
    #[serde(rename = "shipmentTaskId")]
    pub shipment_task_id: Option<String>,
    #[serde(rename = "shipmentOrderId")]
    pub shipment_order_id: Option<String>,
}

/// 订单购物详情
#[derive(Debug, Serialize, Clone)]
pub struct StoreOrderInfoOldVo {
    pub id: i32,
    #[serde(rename = "orderId")]
    pub order_id: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
    pub info: Option<serde_json::Value>,
    #[serde(rename = "unique")]
    pub unique_key: String,
}

/// 订单详情响应
#[derive(Debug, Serialize)]
pub struct StoreOrderInfoResponse {
    pub id: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub uid: i32,
    #[serde(rename = "realName")]
    pub real_name: String,
    #[serde(rename = "userPhone")]
    pub user_phone: String,
    #[serde(rename = "userAddress")]
    pub user_address: String,
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    #[serde(rename = "totalPrice")]
    pub total_price: Decimal,
    #[serde(rename = "payPrice")]
    pub pay_price: Decimal,
    #[serde(rename = "payPostage")]
    pub pay_postage: Decimal,
    #[serde(rename = "couponPrice")]
    pub coupon_price: Decimal,
    #[serde(rename = "deductionPrice")]
    pub deduction_price: Decimal,
    #[serde(rename = "payType")]
    pub pay_type: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    pub status: i32,
    #[serde(rename = "refundStatus")]
    pub refund_status: i32,
    #[serde(rename = "expressRecordType")]
    pub express_record_type: Option<i32>,
    #[serde(rename = "deliveryName")]
    pub delivery_name: Option<String>,
    #[serde(rename = "deliveryType")]
    pub delivery_type: Option<String>,
    #[serde(rename = "deliveryId")]
    pub delivery_id: Option<String>,
    #[serde(rename = "deliveryCode")]
    pub delivery_code: Option<String>,
    pub mark: String,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    pub remark: Option<String>,
    #[serde(rename = "refundPrice")]
    pub refund_price: Decimal,
    #[serde(rename = "useIntegral")]
    pub use_integral: Option<i32>,
    #[serde(rename = "backIntegral")]
    pub back_integral: Option<i32>,
    #[serde(rename = "verifyCode")]
    pub verify_code: String,
    #[serde(rename = "shippingType")]
    pub shipping_type: i32,
    #[serde(rename = "statusStr")]
    pub status_str: HashMap<String, String>,
    #[serde(rename = "payTypeStr")]
    pub pay_type_str: String,
    #[serde(rename = "nikeName")]
    pub nike_name: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "orderInfo")]
    pub order_info: Vec<StoreOrderInfoOldVo>,
    #[serde(rename = "spreadName")]
    pub spread_name: Option<String>,
    #[serde(rename = "proTotalPrice")]
    pub pro_total_price: Decimal,
    #[serde(rename = "refundReasonTime")]
    pub refund_reason_time: Option<String>,
    #[serde(rename = "refundReasonWapImg")]
    pub refund_reason_wap_img: Option<String>,
    #[serde(rename = "shipmentPic")]
    pub shipment_pic: Option<String>,
    #[serde(rename = "shipmentTaskId")]
    pub shipment_task_id: Option<String>,
    #[serde(rename = "shipmentOrderId")]
    pub shipment_order_id: Option<String>,
    #[serde(rename = "shipmentNum")]
    pub shipment_num: Option<String>,
    #[serde(rename = "orderTypeText")]
    pub order_type_text: String,
}

/// 订单各状态数量响应
#[derive(Debug, Serialize)]
pub struct StoreOrderCountItemResponse {
    pub all: i64,
    #[serde(rename = "unPaid")]
    pub un_paid: i64,
    #[serde(rename = "notShipped")]
    pub not_shipped: i64,
    pub spike: i64,
    pub bargain: i64,
    pub complete: i64,
    #[serde(rename = "toBeWrittenOff")]
    pub to_be_written_off: i64,
    pub refunding: i64,
    pub refunded: i64,
    pub deleted: i64,
}

/// 订单九宫格数据响应
#[derive(Debug, Serialize)]
pub struct StoreOrderTopItemResponse {
    pub count: i64,
    pub amount: Decimal,
    #[serde(rename = "weChatAmount")]
    pub we_chat_amount: Decimal,
    #[serde(rename = "yueAmount")]
    pub yue_amount: Decimal,
}

/// 快递轨迹列表项
#[derive(Debug, Serialize)]
pub struct LogisticsResultListVo {
    pub time: Option<String>,
    pub status: Option<String>,
}

/// 快递查询结果
#[derive(Debug, Serialize)]
pub struct LogisticsResultVo {
    pub number: Option<String>,
    #[serde(rename = "type")]
    pub express_type: Option<String>,
    pub list: Vec<LogisticsResultListVo>,
    #[serde(rename = "deliverystatus")]
    pub delivery_status: Option<String>,
    #[serde(rename = "issign")]
    pub is_sign: Option<String>,
    #[serde(rename = "expName")]
    pub exp_name: Option<String>,
    #[serde(rename = "expSite")]
    pub exp_site: Option<String>,
    #[serde(rename = "expPhone")]
    pub exp_phone: Option<String>,
    pub courier: Option<String>,
    #[serde(rename = "courierPhone")]
    pub courier_phone: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    #[serde(rename = "takeTime")]
    pub take_time: Option<String>,
    pub logo: Option<String>,
}

/// 核销订单头部数据
#[derive(Debug, Serialize)]
pub struct StoreStaffTopDetail {
    #[serde(rename = "completeCount")]
    pub complete_count: i64,
    #[serde(rename = "evaluatedCount")]
    pub evaluated_count: i64,
    #[serde(rename = "monthCount")]
    pub month_count: i64,
    #[serde(rename = "monthPrice")]
    pub month_price: Decimal,
    #[serde(rename = "orderCount")]
    pub order_count: i64,
    #[serde(rename = "proCount")]
    pub pro_count: i64,
    #[serde(rename = "proPrice")]
    pub pro_price: Decimal,
    #[serde(rename = "receivedCount")]
    pub received_count: i64,
    #[serde(rename = "refundCount")]
    pub refund_count: i64,
    #[serde(rename = "sumPrice")]
    pub sum_price: Decimal,
    #[serde(rename = "todayCount")]
    pub today_count: i64,
    #[serde(rename = "todayPrice")]
    pub today_price: Decimal,
    #[serde(rename = "unpaidCount")]
    pub unpaid_count: i64,
    #[serde(rename = "unshippedCount")]
    pub unshipped_count: i64,
    #[serde(rename = "verificationCount")]
    pub verification_count: i64,
}

/// 核销月详情
#[derive(Debug, Serialize)]
pub struct StoreStaffDetail {
    pub count: i64,
    pub price: Decimal,
    pub time: String,
}

/// 电子面单配置
#[derive(Debug, Serialize)]
pub struct ExpressSheetVo {
    #[serde(rename = "exportId")]
    pub export_id: Option<i32>,
    #[serde(rename = "exportCom")]
    pub export_com: Option<String>,
    #[serde(rename = "exportTempId")]
    pub export_temp_id: Option<String>,
    #[serde(rename = "exportToName")]
    pub export_to_name: Option<String>,
    #[serde(rename = "exportToTel")]
    pub export_to_tel: Option<String>,
    #[serde(rename = "exportToAddress")]
    pub export_to_address: Option<String>,
    #[serde(rename = "exportSiid")]
    pub export_siid: Option<String>,
    #[serde(rename = "exportOpen")]
    pub export_open: Option<i32>,
}

/// 订单统计图表项
#[derive(Debug, Serialize)]
pub struct StoreOrderStatisticsChartItemResponse {
    pub num: String,
    pub time: String,
}

/// 订单统计响应
#[derive(Debug, Serialize)]
pub struct StoreOrderStatisticsResponse {
    pub chart: Vec<StoreOrderStatisticsChartItemResponse>,
    #[serde(rename = "growthRate")]
    pub growth_rate: i32,
    #[serde(rename = "increaseTime")]
    pub increase_time: String,
    #[serde(rename = "increaseTimeStatus")]
    pub increase_time_status: i32,
    pub time: Decimal,
}

// ==================== 核销订单 DTO ====================

/// 核销订单搜索请求
/// Java: SystemWriteOffOrderSearchRequest
#[derive(Debug, Deserialize)]
pub struct SystemWriteOffOrderSearchRequest {
    /// 核销点ID
    #[serde(default, rename = "storeId", deserialize_with = "deserialize_optional_i32")]
    pub store_id: Option<i32>,

    /// 时间
    #[serde(default, rename = "dateLimit", deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,

    /// 关键字
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub keywords: Option<String>,
}

/// 核销订单响应
/// Java: SystemWriteOffOrderResponse
#[derive(Debug, Serialize)]
pub struct SystemWriteOffOrderResponse {
    /// 订单总数量
    pub total: i64,

    /// 订单总金额
    #[serde(rename = "orderTotalPrice")]
    pub order_total_price: Decimal,

    /// 退款总金额
    #[serde(rename = "refundTotalPrice")]
    pub refund_total_price: Decimal,

    /// 退款总单数
    #[serde(rename = "refundTotal")]
    pub refund_total: i64,

    /// 订单列表
    pub list: PageResponse<StoreOrderItemResponse>,
}

/// 推广人信息
/// Java: StoreOrderSpreadInfoResponse
#[derive(Debug, Serialize)]
pub struct StoreOrderSpreadInfoResponse {
    pub id: i32,
    pub name: String,
}

/// 核销订单列表项
/// Java: StoreOrderItemResponse
#[derive(Debug, Serialize)]
pub struct StoreOrderItemResponse {
    pub id: i32,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub uid: i32,
    #[serde(rename = "realName")]
    pub real_name: String,
    #[serde(rename = "userPhone")]
    pub user_phone: String,
    #[serde(rename = "totalPrice")]
    pub total_price: Decimal,
    #[serde(rename = "payPrice")]
    pub pay_price: Decimal,
    pub paid: bool,
    #[serde(rename = "payTime")]
    pub pay_time: Option<String>,
    #[serde(rename = "payType")]
    pub pay_type: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    pub status: i32,
    #[serde(rename = "storeName")]
    pub store_name: String,
    #[serde(rename = "clerkName")]
    pub clerk_name: String,
    #[serde(rename = "productList")]
    pub product_list: Vec<StoreOrderInfoOldVo>,
    #[serde(rename = "statusStr")]
    pub status_str: HashMap<String, String>,
    #[serde(rename = "payTypeStr")]
    pub pay_type_str: String,
    #[serde(rename = "totalPostage")]
    pub total_postage: Decimal,
    #[serde(rename = "payPostage")]
    pub pay_postage: Decimal,
    #[serde(rename = "gainIntegral")]
    pub gain_integral: Option<i32>,
    #[serde(rename = "useIntegral")]
    pub use_integral: Option<i32>,
    #[serde(rename = "backIntegral")]
    pub back_integral: Option<i32>,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    #[serde(rename = "isSystemDel")]
    pub is_system_del: bool,
    pub mark: String,
    pub remark: Option<String>,
    #[serde(rename = "refundReasonWapImg")]
    pub refund_reason_wap_img: Option<String>,
    #[serde(rename = "refundReasonWapExplain")]
    pub refund_reason_wap_explain: Option<String>,
    #[serde(rename = "refundReasonTime")]
    pub refund_reason_time: Option<String>,
    #[serde(rename = "refundReasonWap")]
    pub refund_reason_wap: Option<String>,
    #[serde(rename = "refundReason")]
    pub refund_reason: Option<String>,
    #[serde(rename = "refundPrice")]
    pub refund_price: Decimal,
    #[serde(rename = "refundStatus")]
    pub refund_status: i32,
    #[serde(rename = "totalNum")]
    pub total_num: i32,
    #[serde(rename = "shippingType")]
    pub shipping_type: i32,
    #[serde(rename = "verifyCode")]
    pub verify_code: String,
    #[serde(rename = "spreadInfo")]
    pub spread_info: StoreOrderSpreadInfoResponse,
    #[serde(rename = "orderType")]
    pub order_type: String,
}
