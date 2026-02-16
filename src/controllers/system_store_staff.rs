/// 门店核销员 -- 控制器
///
/// Java参考: SystemStoreStaffController
/// 路径前缀: /api/admin/system/store/staff
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::system_store::*;
use crate::services::system_store_staff_service::SystemStoreStaffService;

/// 分页列表
/// GET /api/admin/system/store/staff/list
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreStaffSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let store_id = search.store_id.unwrap_or(0);
    let response = SystemStoreStaffService::get_list(&ctx.db, store_id, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增
/// POST /api/admin/system/store/staff/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    Json(request): Json<SystemStoreStaffRequest>,
) -> Result<Response> {
    if SystemStoreStaffService::save_unique(&ctx.db, &request).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增失败"))
    }
}

/// 删除
/// GET /api/admin/system/store/staff/delete?id=xxx
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if SystemStoreStaffService::delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改
/// POST /api/admin/system/store/staff/update?id=xxx
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
    Json(request): Json<SystemStoreStaffRequest>,
) -> Result<Response> {
    if SystemStoreStaffService::edit(&ctx.db, params.id, &request).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 修改状态
/// GET /api/admin/system/store/staff/update/status?id=xxx&status=1
#[debug_handler]
async fn update_status(
    State(ctx): State<AppContext>,
    Query(params): Query<StaffUpdateStatusQuery>,
) -> Result<Response> {
    if SystemStoreStaffService::update_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 详情
/// GET /api/admin/system/store/staff/info?id=xxx
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let response = SystemStoreStaffService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/store/staff")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/update/status", get(update_status))
        .add("/info", get(info))
}
