/// 用户积分管理 -- 控制器
///
/// 实现与Java版本一致的用户积分管理接口
/// Java代码参考: com.zbkj.admin.controller.UserIntegralController
/// 路由前缀: /api/admin/user/integral
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::user_integral::*;
use crate::dtos::common::PageParamRequest;
use crate::services::user_integral_service::UserIntegralRecordService;
use crate::utils::auth;

/// 1. 积分分页列表
///
/// 权限: admin:user:integral:list
/// 路径: POST /api/admin/user/integral/list
/// Java: UserIntegralController.getList(AdminIntegralSearchRequest, PageParamRequest)
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(page_param): Query<PageParamRequest>,
    axum::Json(request): axum::Json<AdminIntegralSearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:user:integral:list").await?;

    let response = UserIntegralRecordService::find_admin_list(
        &ctx.db, &request, &page_param,
    ).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/user/integral")
        .add("/list", post(list))
}
