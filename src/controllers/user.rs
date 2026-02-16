/// 用户管理控制器
///
/// 对应Java的UserController
/// 路由前缀: /api/admin/user
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::user::{UserSearchRequest, UserResponse};
use crate::services::user_service::UserService;
use crate::dtos::common::CommonPage;
use crate::utils::auth;

/// 分页列表
///
/// 对应Java: @RequestMapping(value = "/list", method = RequestMethod.GET)
/// 权限: admin:user:list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<UserSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:list").await?;

    tracing::info!(
        "用户列表查询: keywords={:?}, tagId={:?}, groupId={:?}, page={:?}, limit={:?}",
        search.keywords,
        search.tag_id,
        search.group_id,
        page.page,
        page.limit
    );

    let response: CommonPage<UserResponse> = UserService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 路由定义
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/user")
        .add("/list", get(get_list))
}
