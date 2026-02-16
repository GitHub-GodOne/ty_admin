/// 订单操作记录 -- 控制器
///
/// 实现与Java版本一致的订单操作记录管理接口
/// Java代码参考: com.zbkj.admin.controller.StoreOrderStatusController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::store_order_status::*;
use crate::services::store_order_status_service::StoreOrderStatusService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 订单操作记录分页列表
///
/// 权限: admin:order:status:list
/// 路径: GET /api/admin/store/order/status/list
/// Java: StoreOrderStatusController.getList()
/// 参数: orderNo (订单编号), page, limit
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<StoreOrderStatusSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:order:status:list").await?;

    let response = StoreOrderStatusService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/order/status")
        .add("/list", get(get_list))
}
