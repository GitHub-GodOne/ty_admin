/// 表单模板 -- 控制器
///
/// 实现与Java版本一致的表单模板管理接口
/// Java代码参考: com.zbkj.admin.controller.SystemFormTempController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::system_form_temp::*;
use crate::services::system_form_temp_service::SystemFormTempService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分页列表
///
/// 权限: admin:system:form:list
/// 路径: GET /api/admin/system/form/temp/list
/// Java: SystemFormTempController.getList()
/// 参数: keywords (可选), page, limit
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<SystemFormTempSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:form:list").await?;

    let response = SystemFormTempService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增表单模板
///
/// 权限: admin:system:form:save
/// 路径: POST /api/admin/system/form/temp/save
/// Java: SystemFormTempController.save()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemFormTempRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:form:save").await?;

    if SystemFormTempService::add(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增表单模板失败"))
    }
}

/// 3. 修改表单模板
///
/// 权限: admin:system:form:update
/// 路径: POST /api/admin/system/form/temp/update
/// Java: SystemFormTempController.update()
/// 参数: id (查询参数), body (SystemFormTempRequest)
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    Json(request): Json<SystemFormTempRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:form:update").await?;

    if SystemFormTempService::edit(&ctx.db, params.id, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改表单模板失败"))
    }
}

/// 4. 查询表单模板详情
///
/// 权限: admin:system:form:info
/// 路径: GET /api/admin/system/form/temp/info
/// Java: SystemFormTempController.info()
/// 参数: id (查询参数)
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:form:info").await?;

    let response = SystemFormTempService::get_by_id(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/form/temp")
        .add("/list", get(get_list))
        .add("/save", post(save))
        .add("/update", post(update))
        .add("/info", get(info))
}
