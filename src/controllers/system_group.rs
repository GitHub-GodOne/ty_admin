/// 组合数据 -- 控制器
///
/// Java参考: SystemGroupController
/// 路径前缀: /api/admin/system/group
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::system_group::*;
use crate::services::system_group_service::SystemGroupService;
use crate::utils::auth;

/// 分页列表
/// GET /api/admin/system/group/list
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<SystemGroupSearchRequest>,
    Query(page_param): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:list").await?;

    let response = SystemGroupService::get_list(&ctx.db, &search, &page_param)
        .await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 新增组合数据
/// POST /api/admin/system/group/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<SystemGroupRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:save").await?;

    if SystemGroupService::add(&ctx.db, &request)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("新增失败"))
    }
}

/// 删除组合数据
/// GET /api/admin/system/group/delete
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:delete").await?;

    if SystemGroupService::delete(&ctx.db, params.id)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("删除失败"))
    }
}

/// 修改组合数据
/// POST /api/admin/system/group/update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    axum::Json(request): axum::Json<SystemGroupRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:update").await?;

    if SystemGroupService::edit(&ctx.db, params.id, &request)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("修改失败"))
    }
}

/// 查询组合数据详情
/// GET /api/admin/system/group/info
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:info").await?;

    let response = SystemGroupService::get_by_id(&ctx.db, params.id)
        .await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/group")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
}
