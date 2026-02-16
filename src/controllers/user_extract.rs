/// 用户提现管理 -- 控制器
///
/// 实现与Java版本一致的用户提现管理接口
/// Java代码参考: com.zbkj.admin.controller.UserExtractController
/// 路由前缀: /api/admin/finance/apply
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::user_extract::*;
use crate::dtos::common::PageParamRequest;
use crate::services::user_extract_service::UserExtractService;
use crate::utils::auth;

/// 1. 提现申请列表
///
/// 权限: admin:finance:apply:list
/// 路径: GET /api/admin/finance/apply/list
/// Java: UserExtractController.getList()
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page_param): Query<PageParamRequest>,
    Query(request): Query<UserExtractSearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:finance:apply:list").await?;

    let response = UserExtractService::get_list(
        &ctx.db, &request, &page_param,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 2. 修改提现申请
///
/// 权限: admin:finance:apply:update
/// 路径: POST /api/admin/finance/apply/update
/// Java: UserExtractController.update()
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<UserExtractUpdateRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:finance:apply:update").await?;

    let result = UserExtractService::update_extract(
        &ctx.db, &request,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    if result {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("修改失败"))
    }
}

/// 3. 提现统计
///
/// 权限: admin:finance:apply:balance
/// 路径: POST /api/admin/finance/apply/balance
/// Java: UserExtractController.balance()
#[debug_handler]
async fn balance(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<BalanceDateLimitParam>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:finance:apply:balance").await?;

    let date_limit = params.date_limit.as_deref().unwrap_or("");
    let response = UserExtractService::get_balance(
        &ctx.db, date_limit,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 4. 提现审核
///
/// 权限: admin:finance:apply:apply
/// 路径: POST /api/admin/finance/apply/apply
/// Java: UserExtractController.updateStatus()
#[debug_handler]
async fn apply(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<ExtractApplyRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:finance:apply:apply").await?;

    let result = UserExtractService::update_status(
        &ctx.db, request.id, request.status, request.back_message,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    if result {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("操作失败"))
    }
}

/// balance接口的dateLimit参数
#[derive(Debug, serde::Deserialize)]
struct BalanceDateLimitParam {
    #[serde(rename = "dateLimit")]
    date_limit: Option<String>,
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/finance/apply")
        .add("/list", get(list))
        .add("/update", post(update))
        .add("/balance", post(balance))
        .add("/apply", post(apply))
}
