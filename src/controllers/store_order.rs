/// 订单管理 -- 控制器
///
/// 实现与Java版本一致的订单管理接口
/// Java代码参考: com.zbkj.admin.controller.StoreOrderController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::store_order::*;
use crate::services::store_order_service::StoreOrderService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 分页列表
///
/// 权限: admin:order:list
/// 路径: GET /api/admin/store/order/list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<StoreOrderSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:list").await?;
    let response = StoreOrderService::get_admin_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 获取订单各状态数量
///
/// 权限: admin:order:status:num
/// 路径: GET /api/admin/store/order/status/num
#[debug_handler]
async fn get_status_num(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<OrderStatusNumQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:status:num").await?;
    let response = StoreOrderService::get_order_status_num(
        &ctx.db,
        &params.date_limit,
        params.order_type,
        &params.order_no,
    ).await?;
    format::json(ApiResponse::success(response))
}

/// 获取订单统计数据（九宫格）
///
/// 权限: admin:order:data
/// 路径: GET /api/admin/store/order/data
#[debug_handler]
async fn get_order_data(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<OrderDataQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:data").await?;
    let response = StoreOrderService::get_order_data(&ctx.db, &params.date_limit).await?;
    format::json(ApiResponse::success(response))
}

/// 订单删除
///
/// 权限: admin:order:delete
/// 路径: GET /api/admin/store/order/delete
#[debug_handler]
async fn delete_order(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<OrderNoQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:delete").await?;
    StoreOrderService::delete(&ctx.db, &params.order_no).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 订单备注
///
/// 权限: admin:order:mark
/// 路径: POST /api/admin/store/order/mark
#[debug_handler]
async fn mark_order(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(params): axum::Json<OrderMarkRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:mark").await?;
    StoreOrderService::mark(&ctx.db, &params.order_no, &params.mark).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 订单改价
///
/// 权限: admin:order:update:price
/// 路径: POST /api/admin/store/order/update/price
#[debug_handler]
async fn update_price(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(params): axum::Json<StoreOrderUpdatePriceRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:update:price").await?;
    StoreOrderService::update_price(&ctx.db, &params).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 订单详情
///
/// 权限: admin:order:info
/// 路径: GET /api/admin/store/order/info
#[debug_handler]
async fn order_info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<OrderNoQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:info").await?;
    let response = StoreOrderService::info(&ctx.db, &params.order_no).await?;
    format::json(ApiResponse::success(response))
}

/// 订单发货
///
/// 权限: admin:order:send
/// 路径: POST /api/admin/store/order/send
#[debug_handler]
async fn send_order(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(params): axum::Json<StoreOrderSendRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:send").await?;
    StoreOrderService::send(&ctx.db, &params).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 订单退款
///
/// 权限: admin:order:refund
/// 路径: GET /api/admin/store/order/refund
#[debug_handler]
async fn refund_order(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<StoreOrderRefundRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:refund").await?;
    StoreOrderService::refund(&ctx.db, &params).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 拒绝退款
///
/// 权限: admin:order:refund:refuse
/// 路径: GET /api/admin/store/order/refund/refuse
#[debug_handler]
async fn refund_refuse(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<RefundRefuseQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:refund:refuse").await?;
    StoreOrderService::refund_refuse(&ctx.db, &params.order_no, &params.reason).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 快递查询
///
/// 权限: admin:order:logistics:info
/// 路径: GET /api/admin/store/order/getLogisticsInfo
#[debug_handler]
async fn get_logistics_info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<OrderNoQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:logistics:info").await?;
    let response = StoreOrderService::get_logistics_info(&ctx.db, &params.order_no).await?;
    format::json(ApiResponse::success(response))
}

/// 核销订单头部数据
///
/// 权限: admin:order:verification:data
/// 路径: GET /api/admin/store/order/statisticsData
#[debug_handler]
async fn get_verification_data(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:verification:data").await?;
    let response = StoreOrderService::get_verification_data(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 核销订单月列表数据
///
/// 权限: admin:order:verification:detail
/// 路径: GET /api/admin/store/order/statisticsDetail
#[debug_handler]
async fn get_verification_detail(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<StoreOrderStatisticsRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:verification:detail").await?;
    let response = StoreOrderService::get_verification_detail(&ctx.db, &params).await?;
    format::json(ApiResponse::success(response))
}

/// 核销码核销订单
///
/// 权限: admin:order:verification
/// 路径: GET /api/admin/store/order/writeOff
#[debug_handler]
async fn write_off_order(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<VerifyCodeQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:verification").await?;
    let response = StoreOrderService::verification_order_by_code(&ctx.db, &params.v_code).await?;
    format::json(ApiResponse::success(response))
}

/// 核销码查询待核销订单
///
/// 权限: admin:order:verification:info
/// 路径: GET /api/admin/store/order/writeConfirm
#[debug_handler]
async fn write_confirm(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<VerifyCodeQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:verification:info").await?;
    let response = StoreOrderService::get_verification_order_by_code(&ctx.db, &params.v_code).await?;
    format::json(ApiResponse::success(response))
}

/// 订单统计详情
///
/// 权限: admin:order:statistics
/// 路径: GET /api/admin/store/order/time
#[debug_handler]
async fn order_time(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<OrderTimeQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:statistics").await?;
    let response = StoreOrderService::order_statistics_by_time(
        &ctx.db,
        &params.date_limit,
        params.stat_type,
    ).await?;
    format::json(ApiResponse::success(response))
}

/// 获取面单默认配置信息
///
/// 权限: admin:order:delivery:info
/// 路径: GET /api/admin/store/order/getDeliveryInfo
#[debug_handler]
async fn get_delivery_info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:delivery:info").await?;
    let response = StoreOrderService::get_delivery_info(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 更改订单运单号
///
/// 权限: admin:order:update:tracking
/// 路径: POST /api/admin/store/order/updateTrackingNumber
#[debug_handler]
async fn update_tracking_number(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(params): axum::Json<StoreOrderSendRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:update:tracking").await?;
    StoreOrderService::update_tracking_number(&ctx.db, &params).await?;
    format::json(ApiResponse::<String>::success_empty())
}

/// 路由注册
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/order")
        .add("/list", get(get_list))
        .add("/status/num", get(get_status_num))
        .add("/data", get(get_order_data))
        .add("/delete", get(delete_order))
        .add("/mark", post(mark_order))
        .add("/update/price", post(update_price))
        .add("/info", get(order_info))
        .add("/send", post(send_order))
        .add("/refund", get(refund_order))
        .add("/refund/refuse", get(refund_refuse))
        .add("/getLogisticsInfo", get(get_logistics_info))
        .add("/statisticsData", get(get_verification_data))
        .add("/statisticsDetail", get(get_verification_detail))
        .add("/writeOff", get(write_off_order))
        .add("/writeConfirm", get(write_confirm))
        .add("/time", get(order_time))
        .add("/getDeliveryInfo", get(get_delivery_info))
        .add("/updateTrackingNumber", post(update_tracking_number))
}
