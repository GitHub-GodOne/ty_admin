/// 核销订单 -- 控制器
///
/// Java参考: SystemWriteOffOrderController
/// 路径前缀: /api/admin/system/store/order
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::store_order::SystemWriteOffOrderSearchRequest;
use crate::services::store_order_service::StoreOrderService;

/// 核销订单分页列表
/// POST /api/admin/system/store/order/list
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Query(search): Query<SystemWriteOffOrderSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreOrderService::get_write_off_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/store/order")
        .add("/list", post(list))
}
