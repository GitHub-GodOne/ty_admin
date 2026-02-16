/// 组合数据详情 -- 控制器
///
/// Java代码参考: com.zbkj.admin.controller.SystemGroupDataController
/// 路由前缀: /api/admin/system/group/data
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::system_group_data::*;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::services::system_group_data_service::SystemGroupDataService;
use crate::utils::auth;

/// 1. 分页组合数据详情
///
/// 权限: admin:system:group:data:list
/// 路径: GET /api/admin/system/group/data/list
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page_param): Query<PageParamRequest>,
    Query(search): Query<SystemGroupDataSearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:data:list").await?;

    let response = SystemGroupDataService::get_list(
        &ctx.db, &search, &page_param,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增组合数据
///
/// 权限: admin:system:group:data:save
/// 路径: POST /api/admin/system/group/data/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<SystemGroupDataRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:data:save").await?;

    if SystemGroupDataService::create(&ctx.db, &request)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("新增失败"))
    }
}
/// 3. 删除组合数据
///
/// 权限: admin:system:group:data:delete
/// 路径: GET /api/admin/system/group/data/delete
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:data:delete").await?;

    if SystemGroupDataService::delete(&ctx.db, params.id)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("删除失败"))
    }
}

/// 4. 修改组合数据
///
/// 权限: admin:system:group:data:update
/// 路径: POST /api/admin/system/group/data/update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    axum::Json(request): axum::Json<SystemGroupDataRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:data:update").await?;

    if SystemGroupDataService::update(&ctx.db, params.id, &request)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("修改失败"))
    }
}

/// 5. 组合数据详情信息
///
/// 权限: admin:system:group:data:info
/// 路径: GET /api/admin/system/group/data/info
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:group:data:info").await?;

    let response = SystemGroupDataService::get_info(&ctx.db, params.id)
        .await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/group/data")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
}
