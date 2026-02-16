/// 用户标签控制器
///
/// 对应Java的UserTagController
/// 路由前缀: /api/admin/user/tag
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{PageParamRequest, IdQuery, CommonPage};
use crate::dtos::user_tag::{UserTagRequest, UserTagResponse};
use crate::services::user_tag_service::UserTagService;
use crate::utils::auth;

/// 分页列表
///
/// 对应Java: @RequestMapping(value = "/list", method = RequestMethod.GET)
/// 权限: admin:user:tag:list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:tag:list").await?;

    tracing::info!("用户标签列表查询: page={:?}, limit={:?}", page.page, page.limit);

    let response: CommonPage<UserTagResponse> = UserTagService::get_list(&ctx.db, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增用户标签
///
/// 对应Java: @RequestMapping(value = "/save", method = RequestMethod.POST)
/// 权限: admin:user:tag:save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<UserTagRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:tag:save").await?;

    tracing::info!("新增用户标签: name={}", request.name);

    let result = UserTagService::create(&ctx.db, &request).await?;
    if result {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("创建失败"))
    }
}

/// 删除用户标签
///
/// 对应Java: @RequestMapping(value = "/delete", method = RequestMethod.GET)
/// 权限: admin:user:tag:delete
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:tag:delete").await?;

    tracing::info!("删除用户标签: id={}", params.id);

    let result = UserTagService::delete(&ctx.db, params.id).await?;
    if result {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改用户标签
///
/// 对应Java: @RequestMapping(value = "/update", method = RequestMethod.POST)
/// 权限: admin:user:tag:update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    Json(request): Json<UserTagRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:tag:update").await?;

    tracing::info!("修改用户标签: id={}, name={}", params.id, request.name);

    let result = UserTagService::update(&ctx.db, params.id, &request).await?;
    if result {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("更新失败"))
    }
}

/// 查询用户标签详情
///
/// 对应Java: @RequestMapping(value = "/info", method = RequestMethod.GET)
/// 权限: admin:user:tag:info
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:tag:info").await?;

    tracing::info!("查询用户标签详情: id={}", params.id);

    let tag = UserTagService::get_by_id(&ctx.db, params.id).await?;
    match tag {
        Some(t) => format::json(ApiResponse::success(t)),
        None => format::json(ApiResponse::<()>::failed("标签不存在")),
    }
}

/// 路由定义
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/user/tag")
        .add("/list", get(get_list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
}
