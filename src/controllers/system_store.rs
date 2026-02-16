/// 门店自提 -- 控制器
///
/// Java参考: SystemStoreController
/// 路径前缀: /api/admin/system/store
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::system_store::*;
use crate::services::system_store_service::SystemStoreService;

/// 分页列表
/// GET /api/admin/system/store/list
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    Query(search): Query<SystemStoreSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = SystemStoreService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 数量统计
/// GET /api/admin/system/store/getCount
#[debug_handler]
async fn get_count(
    State(ctx): State<AppContext>,
    Query(search): Query<SystemStoreSearchRequest>,
) -> Result<Response> {
    let response = SystemStoreService::get_count(&ctx.db, &search.keywords).await?;
    format::json(ApiResponse::success(response))
}

/// 新增
/// POST /api/admin/system/store/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    Json(request): Json<SystemStoreRequest>,
) -> Result<Response> {
    if SystemStoreService::create(&ctx.db, &request).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增失败"))
    }
}

/// 删除（软删除）
/// GET /api/admin/system/store/delete?id=xxx
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if SystemStoreService::delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改
/// POST /api/admin/system/store/update?id=xxx
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
    Json(request): Json<SystemStoreRequest>,
) -> Result<Response> {
    if SystemStoreService::update(&ctx.db, params.id, &request).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 修改门店显示状态
/// GET /api/admin/system/store/update/status?id=xxx&status=true
#[debug_handler]
async fn update_status(
    State(ctx): State<AppContext>,
    Query(params): Query<StoreUpdateStatusQuery>,
) -> Result<Response> {
    if SystemStoreService::update_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 详情
/// GET /api/admin/system/store/info?id=xxx
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let response = SystemStoreService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

/// 彻底删除
/// GET /api/admin/system/store/completely/delete?id=xxx
#[debug_handler]
async fn completely_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if SystemStoreService::completely_delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("彻底删除失败"))
    }
}

/// 恢复
/// GET /api/admin/system/store/recovery?id=xxx
#[debug_handler]
async fn recovery(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if SystemStoreService::recovery(&ctx.db, params.id).await? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("恢复失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/store")
        .add("/list", get(list))
        .add("/getCount", get(get_count))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/update/status", get(update_status))
        .add("/info", get(info))
        .add("/completely/delete", get(completely_delete))
        .add("/recovery", get(recovery))
}
