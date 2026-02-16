/// 用户充值管理 -- 控制器
///
/// 实现与Java版本一致的用户充值管理接口
/// Java代码参考: com.zbkj.admin.controller.UserRechargeController
/// 路由前缀: /api/admin/user/topUpLog
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::user_recharge::*;
use crate::dtos::common::PageParamRequest;
use crate::services::user_recharge_service::UserRechargeService;
use crate::utils::auth;

/// 1. 充值记录列表
///
/// 权限: admin:recharge:list
/// 路径: GET /api/admin/user/topUpLog/list
/// Java: UserRechargeController.getList(UserRechargeSearchRequest, PageParamRequest)
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page_param): Query<PageParamRequest>,
    Query(request): Query<UserRechargeSearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:recharge:list").await?;

    let response = UserRechargeService::get_list(
        &ctx.db, &request, &page_param,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 2. 充值余额统计
///
/// 权限: admin:recharge:balance
/// 路径: POST /api/admin/user/topUpLog/balance
/// Java: UserRechargeController.getBalance()
#[debug_handler]
async fn balance(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:recharge:balance").await?;

    let response = UserRechargeService::get_balance_list(
        &ctx.db,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/user/topUpLog")
        .add("/list", get(list))
        .add("/balance", post(balance))
}
