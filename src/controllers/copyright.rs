/// 版权控制器
///
/// 实现与Java版本完全一致的版权接口
/// Java代码参考: com.zbkj.admin.controller.CopyrightController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::services::copyright_service::CopyrightService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 获取版权信息
///
/// 权限: admin:copyright:get:info
/// 路径: GET /api/admin/copyright/get/info
/// Java: @PreAuthorize("hasAuthority('admin:copyright:get:info')")
#[debug_handler]
async fn get_info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:copyright:get:info").await?;

    let response = CopyrightService::get_info(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/copyright")
        .add("/get/info", get(get_info))
}
