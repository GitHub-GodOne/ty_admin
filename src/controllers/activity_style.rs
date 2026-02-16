/// 活动样式 -- 控制器
///
/// Java参考: ActivityStyleController
/// 路径前缀: /api/admin/activitystyle
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::activity_style::*;
use crate::services::activity_style_service::ActivityStyleService;

/// 分页列表
/// GET /api/admin/activitystyle/list
#[debug_handler]
async fn style_list(
    State(ctx): State<AppContext>,
    Query(search): Query<ActivityStyleSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = ActivityStyleService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增活动样式
/// POST /api/admin/activitystyle/save
#[debug_handler]
async fn style_save(
    State(ctx): State<AppContext>,
    Json(request): Json<ActivityStyleRequest>,
) -> Result<Response> {
    if ActivityStyleService::save(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

/// 删除活动样式
/// GET /api/admin/activitystyle/delete?id=xxx
#[debug_handler]
async fn style_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if ActivityStyleService::delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改活动样式
/// POST /api/admin/activitystyle/update
#[debug_handler]
async fn style_update(
    State(ctx): State<AppContext>,
    Json(request): Json<ActivityStyleRequest>,
) -> Result<Response> {
    if ActivityStyleService::update(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 更新状态
/// POST /api/admin/activitystyle/status
#[debug_handler]
async fn style_update_status(
    State(ctx): State<AppContext>,
    Json(request): Json<ActivityStyleUpdateStatusRequest>,
) -> Result<Response> {
    if ActivityStyleService::update_status(&ctx.db, request.id, request.status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/activitystyle")
        .add("/list", get(style_list))
        .add("/save", post(style_save))
        .add("/delete", get(style_delete))
        .add("/update", post(style_update))
        .add("/status", post(style_update_status))
}
