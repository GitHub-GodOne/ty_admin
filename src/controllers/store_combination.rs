/// 拼团管理 -- 控制器
///
/// Java参考: StoreCombinationController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::store_combination::*;
use crate::services::store_combination_service::StoreCombinationService;

/// 分页列表
/// GET /api/admin/store/combination/list
#[debug_handler]
async fn combination_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreCombinationSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreCombinationService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增拼团商品
/// POST /api/admin/store/combination/save
#[debug_handler]
async fn combination_save(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreCombinationRequest>,
) -> Result<Response> {
    if StoreCombinationService::save_combination(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

/// 删除拼团商品
/// GET /api/admin/store/combination/delete?id=xxx
#[debug_handler]
async fn combination_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if StoreCombinationService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改拼团商品
/// POST /api/admin/store/combination/update
#[debug_handler]
async fn combination_update(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreCombinationRequest>,
) -> Result<Response> {
    if StoreCombinationService::update_combination(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 修改拼团商品显示状态
/// POST /api/admin/store/combination/update/status
#[debug_handler]
async fn combination_update_status(
    State(ctx): State<AppContext>,
    Json(params): Json<CombinationStatusRequest>,
) -> Result<Response> {
    if StoreCombinationService::update_combination_show(&ctx.db, params.id, params.is_show).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 拼团商品详情（管理端）
/// GET /api/admin/store/combination/info?id=xxx
#[debug_handler]
async fn combination_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let info = StoreCombinationService::get_admin_detail(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(info))
}

/// 拼团统计
/// GET /api/admin/store/combination/statistics
#[debug_handler]
async fn combination_statistics(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let stats = StoreCombinationService::get_admin_statistics(&ctx.db).await?;
    format::json(ApiResponse::success(stats))
}

/// 拼团记录列表（团长列表）
/// GET /api/admin/store/combination/combine/list
#[debug_handler]
async fn combine_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StorePinkSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreCombinationService::get_pink_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 拼团订单详情
/// GET /api/admin/store/combination/order_pink?id=xxx
#[debug_handler]
async fn order_pink(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let detail = StoreCombinationService::get_pink_detail(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(detail))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/combination")
        .add("/list", get(combination_list))
        .add("/info", get(combination_info))
        .add("/delete", get(combination_delete))
        .add("/save", post(combination_save))
        .add("/update", post(combination_update))
        .add("/update/status", post(combination_update_status))
        .add("/statistics", get(combination_statistics))
        .add("/combine/list", get(combine_list))
        .add("/order_pink", get(order_pink))
}
