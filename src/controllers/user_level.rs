/// 用户等级控制器
///
/// 对应Java的UserLevelController
/// 路由前缀: /api/admin/user/level
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{PageParamRequest, CommonPage};
use crate::dtos::user_level::UserLevelResponse;
use crate::services::user_level_service::UserLevelService;
use crate::utils::auth;

/// 分页列表
///
/// 对应Java: @RequestMapping(value = "/list", method = RequestMethod.GET)
/// 权限: admin:user:level:list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    // 权限验证
    auth::check_permission(&headers, "admin:user:level:list").await?;

    tracing::info!("用户等级列表查询: page={:?}, limit={:?}", page.page, page.limit);

    let response: CommonPage<UserLevelResponse> = UserLevelService::get_list(&ctx.db, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 路由定义
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/user/level")
        .add("/list", get(get_list))
}
