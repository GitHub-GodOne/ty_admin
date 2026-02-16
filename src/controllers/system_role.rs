/// 系统角色 -- 控制器
///
/// 实现与Java版本一致的角色管理接口
/// Java代码参考: com.zbkj.admin.controller.SystemRoleController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::system_role::*;
use crate::services::system_role_service::SystemRoleService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分页列表
///
/// 权限: admin:system:role:list
/// 路径: GET /api/admin/system/role/list
/// Java: SystemRoleController.getList()
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<SystemRoleSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:role:list").await?;

    let response = SystemRoleService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增角色
///
/// 权限: admin:system:role:save
/// 路径: POST /api/admin/system/role/save
/// Java: SystemRoleController.save()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemRoleRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:role:save").await?;

    if SystemRoleService::add(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增角色失败"))
    }
}

/// 3. 删除角色
///
/// 权限: admin:system:role:delete
/// 路径: GET /api/admin/system/role/delete
/// Java: SystemRoleController.delete()
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:role:delete").await?;

    if SystemRoleService::delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除角色失败"))
    }
}

/// 4. 修改角色
///
/// 权限: admin:system:role:update
/// 路径: POST /api/admin/system/role/update
/// Java: SystemRoleController.update()
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemRoleRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:role:update").await?;

    if SystemRoleService::edit(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改角色失败"))
    }
}

/// 5. 角色详情
///
/// 权限: admin:system:role:info
/// 路径: GET /api/admin/system/role/info
/// Java: SystemRoleController.info()
/// 注意: Java用 @PathVariable，Rust改为查询参数
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:role:info").await?;

    let response = SystemRoleService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

/// 6. 修改角色状态
///
/// 权限: admin:system:role:update:status
/// 路径: GET /api/admin/system/role/updateStatus
/// Java: SystemRoleController.updateStatus()
#[debug_handler]
async fn update_status(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<UpdateStatusQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:role:update:status").await?;

    if SystemRoleService::update_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::success("修改成功".to_string()))
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/role")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
        .add("/updateStatus", get(update_status))
}
