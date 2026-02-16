/// 城市管理 -- 控制器
///
/// 实现与Java版本一致的城市管理接口
/// Java代码参考: com.zbkj.admin.controller.SystemCityController
/// 路由前缀: /api/admin/system/city
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::IdQuery;
use crate::dtos::system_city::*;
use crate::services::system_city_service::SystemCityService;
use crate::utils::auth;

/// 1. 城市列表（按父级id查询）
///
/// 权限: admin:system:city:list
/// 路径: GET /api/admin/system/city/list
/// Java: SystemCityController.getList(SystemCitySearchRequest)
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<SystemCitySearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:city:list").await?;

    let response = SystemCityService::get_list(&ctx.db, &request).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 修改城市
///
/// 权限: admin:system:city:update
/// 路径: POST /api/admin/system/city/update
/// Java: SystemCityController.update(@RequestParam Integer id, @Validated SystemCityRequest)
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    axum::Json(request): axum::Json<SystemCityRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:city:update").await?;

    if SystemCityService::update(&ctx.db, params.id, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 3. 修改城市显示状态
///
/// 权限: admin:system:city:update:status
/// 路径: POST /api/admin/system/city/update/status
/// Java: SystemCityController.updateStatus(@RequestParam Integer id, @RequestParam Boolean status)
#[debug_handler]
async fn update_status(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<SystemCityUpdateStatusRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:city:update:status").await?;

    if SystemCityService::update_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 4. 城市详情
///
/// 权限: admin:system:city:info
/// 路径: GET /api/admin/system/city/info
/// Java: SystemCityController.info(@RequestParam Integer id)
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:city:info").await?;

    let response = SystemCityService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

/// 5. 获取城市树形结构
///
/// 权限: admin:system:city:list:tree
/// 路径: GET /api/admin/system/city/list/tree
/// Java: SystemCityController.getListTree()
#[debug_handler]
async fn get_list_tree(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:city:list:tree").await?;

    let response = SystemCityService::get_list_tree(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/city")
        .add("/list", get(get_list))
        .add("/update", post(update))
        .add("/update/status", post(update_status))
        .add("/info", get(info))
        .add("/list/tree", get(get_list_tree))
}
