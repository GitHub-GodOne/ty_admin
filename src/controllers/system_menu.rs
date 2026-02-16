/// 系统菜单 -- 控制器
///
/// 实现与Java版本一致的菜单管理接口
/// Java代码参考: com.zbkj.admin.controller.SystemMenuController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::IdQuery;
use crate::dtos::system_menu::*;
use crate::services::system_menu_service::SystemMenuService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 菜单列表
///
/// 权限: admin:system:menu:list
/// 路径: GET /api/admin/system/menu/list
/// Java: SystemMenuController.getList()
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<SystemMenuSearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:list").await?;

    let response = SystemMenuService::get_admin_list(&ctx.db, &search).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增菜单
///
/// 权限: admin:system:menu:save
/// 路径: POST /api/admin/system/menu/save
/// Java: SystemMenuController.save()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemMenuRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:save").await?;

    if SystemMenuService::add(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增菜单失败"))
    }
}

/// 3. 删除菜单
///
/// 权限: admin:system:menu:delete
/// 路径: GET /api/admin/system/menu/delete
/// Java: SystemMenuController.delete()
/// 注意: Java用 @PathVariable("id")，Rust改为查询参数
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:delete").await?;

    if SystemMenuService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除菜单失败"))
    }
}

/// 4. 修改菜单
///
/// 权限: admin:system:menu:update
/// 路径: POST /api/admin/system/menu/update
/// Java: SystemMenuController.update()
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemMenuRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:update").await?;

    if SystemMenuService::edit(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改菜单失败"))
    }
}

/// 5. 菜单详情
///
/// 权限: admin:system:menu:info
/// 路径: GET /api/admin/system/menu/info
/// Java: SystemMenuController.info()
/// 注意: Java用 @PathVariable("id")，Rust改为查询参数
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:info").await?;

    let response = SystemMenuService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

/// 6. 切换显示状态
///
/// 权限: admin:system:menu:show:status
/// 路径: POST /api/admin/system/menu/updateShowStatus
/// Java: SystemMenuController.updateShowStatus()
/// 注意: Java用 @PathVariable("id")，Rust改为查询参数
#[debug_handler]
async fn update_show_status(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:show:status").await?;

    if SystemMenuService::update_show_status(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("切换显示状态失败"))
    }
}

/// 7. 获取菜单缓存树
///
/// 权限: admin:system:menu:cache:tree
/// 路径: GET /api/admin/system/menu/cache/tree
/// Java: SystemMenuController.getCacheTree()
#[debug_handler]
async fn cache_tree(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:menu:cache:tree").await?;

    let response = SystemMenuService::get_cache_tree(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/menu")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
        .add("/updateShowStatus", post(update_show_status))
        .add("/cache/tree", get(cache_tree))
}
