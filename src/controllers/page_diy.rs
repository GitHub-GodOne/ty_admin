/// DIY页面管理 -- 控制器
///
/// 实现与Java版本一致的DIY页面管理接口
/// Java代码参考: com.zbkj.admin.controller.PageDiyController
/// 路由前缀: /api/admin/pagediy
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::page_diy::*;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::services::page_diy_service::PageDiyService;
use crate::utils::auth;

/// 1. 分页列表
///
/// 权限: admin:pagediy:list
/// 路径: GET /api/admin/pagediy/list
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page_param): Query<PageParamRequest>,
    Query(search): Query<PageDiySearchParam>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:list").await?;

    let name = search.name.as_deref().unwrap_or("");
    let response = PageDiyService::get_list(
        &ctx.db, name, &page_param,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 2. 设置商城首页
///
/// 权限: admin:pagediy:setdefault
/// 路径: GET /api/admin/pagediy/setdefault
#[debug_handler]
async fn set_default(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:setdefault").await?;

    let result = PageDiyService::set_diy_page_home(
        &ctx.db, params.id,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    if result {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("设置失败"))
    }
}

/// 3. 获取商城首页模版ID
///
/// 权限: admin:pagediy:getdefault
/// 路径: GET /api/admin/pagediy/getdefault
#[debug_handler]
async fn get_default(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:getdefault").await?;

    let id = PageDiyService::get_diy_page_home_id(
        &ctx.db,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(id))
}

/// 4. 新增DIY页面
///
/// 权限: admin:pagediy:save
/// 路径: POST /api/admin/pagediy/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<PageDiyRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:save").await?;

    let result = PageDiyService::save_page_diy(
        &ctx.db, &request,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(result))
}

/// 5. 删除DIY页面
///
/// 权限: admin:pagediy:delete
/// 路径: GET /api/admin/pagediy/delete
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:delete").await?;

    let result = PageDiyService::delete(
        &ctx.db, params.id,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    if result {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("删除失败"))
    }
}

/// 6. 修改DIY页面
///
/// 权限: admin:pagediy:update
/// 路径: POST /api/admin/pagediy/update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<PageDiyRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:update").await?;

    let result = PageDiyService::edit_page_diy(
        &ctx.db, &request,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    if result {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("修改失败"))
    }
}

/// 7. 修改DIY模版名称
///
/// 权限: admin:pagediy:updatename
/// 路径: POST /api/admin/pagediy/updatename
#[debug_handler]
async fn update_name(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<PageDiyEditNameRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:updatename").await?;

    let result = PageDiyService::edit_page_diy_name(
        &ctx.db, &request,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    if result {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("修改失败"))
    }
}

/// 8. DIY详情
///
/// 权限: admin:pagediy:info
/// 路径: GET /api/admin/pagediy/info
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pagediy:info").await?;

    let response = PageDiyService::get_info(
        &ctx.db, params.id,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/pagediy")
        .add("/list", get(list))
        .add("/setdefault", get(set_default))
        .add("/getdefault", get(get_default))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/updatename", post(update_name))
        .add("/info", get(info))
}
