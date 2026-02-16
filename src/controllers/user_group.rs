/// 用户分组控制器
///
/// 对应Java的UserGroupController
/// 路由前缀: /api/admin/user/group
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{PageParamRequest, IdQuery, CommonPage};
use crate::dtos::user_group::{UserGroupRequest, UserGroupResponse};
use crate::services::user_group_service::UserGroupService;
use crate::utils::auth;

/// 分页列表
///
/// 对应Java: @RequestMapping(value = "/list", method = RequestMethod.GET)
/// 权限: admin:user:group:list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:group:list").await?;

    tracing::info!("用户分组列表查询: page={:?}, limit={:?}", page.page, page.limit);

    let response: CommonPage<UserGroupResponse> = UserGroupService::get_list(&ctx.db, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增用户分组
///
/// 对应Java: @RequestMapping(value = "/save", method = RequestMethod.POST)
/// 权限: admin:user:group:save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<UserGroupRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:group:save").await?;

    tracing::info!("新增用户分组: groupName={}", request.group_name);

    let result = UserGroupService::create(&ctx.db, &request).await?;
    if result {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("创建失败"))
    }
}

/// 删除用户分组
///
/// 对应Java: @RequestMapping(value = "/delete", method = RequestMethod.GET)
/// 权限: admin:user:group:delete
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:group:delete").await?;

    tracing::info!("删除用户分组: id={}", params.id);

    let result = UserGroupService::delete(&ctx.db, params.id).await?;
    if result {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改用户分组
///
/// 对应Java: @RequestMapping(value = "/update", method = RequestMethod.POST)
/// 权限: admin:user:group:update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    Json(request): Json<UserGroupRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:group:update").await?;

    tracing::info!("修改用户分组: id={}, groupName={}", params.id, request.group_name);

    let result = UserGroupService::update(&ctx.db, params.id, &request).await?;
    if result {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("更新失败"))
    }
}

/// 查询用户分组详情
///
/// 对应Java: @RequestMapping(value = "/info", method = RequestMethod.GET)
/// 权限: admin:user:group:info
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:group:info").await?;

    tracing::info!("查询用户分组详情: id={}", params.id);

    let group = UserGroupService::get_by_id(&ctx.db, params.id).await?;
    match group {
        Some(g) => format::json(ApiResponse::success(g)),
        None => format::json(ApiResponse::<()>::failed("分组不存在")),
    }
}

/// 路由定义
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/user/group")
        .add("/list", get(get_list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
}
