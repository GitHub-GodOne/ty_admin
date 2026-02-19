/// 用户等级控制器
///
/// 对应Java的SystemUserLevelController
/// 路由前缀: /api/admin/system/user/level
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{PageParamRequest, CommonPage};
use crate::dtos::user_level::{UserLevelResponse, UserLevelSaveRequest, UserLevelUseRequest, LevelIdRequest};
use crate::services::user_level_service::UserLevelService;
use crate::utils::auth;

/// 分页列表
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(_page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:user:level:list").await?;

    // 返回全部列表，不分页
    let page_param = PageParamRequest { page: Some(1), limit: Some(9999) };
    let response: CommonPage<UserLevelResponse> = UserLevelService::get_list(&ctx.db, &page_param).await?;
    // 直接返回列表数组，与Java保持一致
    format::json(ApiResponse::success(response.list))
}

/// 保存用户等级
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<UserLevelSaveRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:user:level:save").await?;

    tracing::info!("保存用户等级: name={}", request.name);
    UserLevelService::save(&ctx.db, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 更新用户等级
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<LevelIdRequest>,
    Json(request): Json<UserLevelSaveRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:user:level:update").await?;

    let id = params.id.ok_or_else(|| Error::string("ID不能为空"))?;
    tracing::info!("更新用户等级: id={}, name={}", id, request.name);
    UserLevelService::update(&ctx.db, id, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 删除用户等级
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<LevelIdRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:user:level:delete").await?;

    let id = params.id.ok_or_else(|| Error::string("ID不能为空"))?;
    tracing::info!("删除用户等级: id={}", id);
    UserLevelService::delete(&ctx.db, id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 更新等级状态（显示/隐藏）
#[debug_handler]
async fn update_use(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<UserLevelUseRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:user:level:use").await?;

    tracing::info!("更新用户等级状态: id={}, isShow={}", request.id, request.is_show);
    UserLevelService::update_show(&ctx.db, request.id, request.is_show).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 路由定义
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/user/level")
        .add("/list", get(get_list))
        .add("/save", post(save))
        .add("/update", post(update))
        .add("/delete", post(delete))
        .add("/use", post(update_use))
}
