/// 砍价管理 -- 控制器
///
/// Java参考: StoreBargainController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::store_bargain::*;
use crate::services::store_bargain_service::StoreBargainService;

/// 分页列表
/// GET /api/admin/store/bargain/list
#[debug_handler]
async fn bargain_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreBargainSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreBargainService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增砍价商品
/// POST /api/admin/store/bargain/save
#[debug_handler]
async fn bargain_save(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreBargainRequest>,
) -> Result<Response> {
    if StoreBargainService::save_bargain(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

/// 删除砍价商品
/// GET /api/admin/store/bargain/delete?id=xxx
#[debug_handler]
async fn bargain_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if StoreBargainService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改砍价商品
/// POST /api/admin/store/bargain/update
#[debug_handler]
async fn bargain_update(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreBargainRequest>,
) -> Result<Response> {
    if StoreBargainService::update_bargain(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 修改砍价商品状态
/// POST /api/admin/store/bargain/update/status
#[debug_handler]
async fn bargain_update_status(
    State(ctx): State<AppContext>,
    Json(params): Json<BargainStatusRequest>,
) -> Result<Response> {
    if StoreBargainService::update_bargain_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 砍价商品详情（管理端）
/// GET /api/admin/store/bargain/info?id=xxx
#[debug_handler]
async fn bargain_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let info = StoreBargainService::get_admin_detail(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(info))
}

/// 砍价用户列表
/// GET /api/admin/store/bargain/bargain_list
#[debug_handler]
async fn bargain_user_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreBargainUserSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreBargainService::get_bargain_user_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 砍价帮助记录列表
/// GET /api/admin/store/bargain/bargain_list_info?id=xxx
/// (Java原为 bargain_list/{id}，因Loco不支持路径参数改为query参数)
#[debug_handler]
async fn bargain_user_help_list(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreBargainService::get_bargain_user_help_list(&ctx.db, params.id, &page).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/bargain")
        .add("/list", get(bargain_list))
        .add("/info", get(bargain_info))
        .add("/delete", get(bargain_delete))
        .add("/save", post(bargain_save))
        .add("/update", post(bargain_update))
        .add("/update/status", post(bargain_update_status))
        .add("/bargain_list", get(bargain_user_list))
        .add("/bargain_list_info", get(bargain_user_help_list))
}
