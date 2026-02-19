/// 用户管理控制器
///
/// 对应Java的UserController
/// 路由前缀: /api/admin/user
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::user::{UserSearchRequest, UserResponse, UserIdRequest, UserUpdateRequest, UserInfoByConditionRequest, UserConditionResponse, UpdateUserLevelRequest};
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

/// 会员详情页Top数据
///
/// 对应Java: @RequestMapping(value = "topdetail", method = RequestMethod.GET)
/// 权限: admin:user:topdetail
#[debug_handler]
async fn top_detail(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<UserIdRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:user:topdetail").await?;

    let uid = params.get_uid().ok_or_else(|| Error::string("用户ID不能为空"))?;
    let response = UserService::get_top_detail(&ctx.db, uid).await?;
    format::json(ApiResponse::success(response))
}

/// 根据参数类型查询会员对应的信息（编辑用）
///
/// 对应Java: @RequestMapping(value = "/infobycondition", method = RequestMethod.GET)
/// 权限: admin:user:infobycondition
/// type: 0=消费记录，1=积分明细，2=签到记录，3=持有优惠券，4=余额变动，5=好友关系
#[debug_handler]
async fn info_by_condition(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<UserInfoByConditionRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:user:infobycondition").await?;

    // 验证type参数范围
    if params.condition_type < 0 || params.condition_type > 5 {
        return Err(Error::string("type参数必须在0-5之间"));
    }

    tracing::info!(
        "会员详情查询: userId={}, type={}",
        params.user_id,
        params.condition_type
    );

    let response: UserConditionResponse = UserService::get_info_by_condition(
        &ctx.db,
        params.user_id,
        params.condition_type,
        &page,
    ).await?;

    format::json(ApiResponse::success(response))
}

/// 修改用户
///
/// 对应Java: @RequestMapping(value = "/update", method = RequestMethod.POST)
/// 权限: admin:user:update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(id_query): Query<UserIdRequest>,
    Json(request): Json<UserUpdateRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:user:update").await?;

    let uid = id_query.uid.or(id_query.user_id).ok_or_else(|| Error::string("用户ID不能为空"))?;
    UserService::update_user(&ctx.db, uid, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 更新用户会员等级
///
/// 对应Java: @RequestMapping(value = "/update/level", method = RequestMethod.POST)
/// 权限: admin:user:update:level
#[debug_handler]
async fn update_level(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<UpdateUserLevelRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:user:update:level").await?;

    tracing::info!("更新用户等级: uid={}, levelId={}", request.uid, request.level_id);

    UserService::update_user_level(&ctx.db, request.uid, request.level_id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 路由定义
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/user")
        .add("/list", get(get_list))
        .add("/topdetail", get(top_detail))
        .add("/infobycondition", get(info_by_condition))
        .add("/update", post(update))
        .add("/update/level", post(update_level))
}
