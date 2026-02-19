/// 微信模板消息同步 -- 控制器
///
/// 路由前缀: /api/admin/wechat/template
///
/// 注意: 同步功能需要对接微信API（公众号/小程序）
/// 当前实现为占位接口，返回提示信息
use loco_rs::prelude::*;
use axum::debug_handler;

use crate::common::response::ApiResponse;
use crate::utils::auth;

/// 同步微信公众号模板消息
///
/// 权限: admin:wechat:whcbqhn:sync
/// 路径: POST /api/admin/wechat/template/whcbqhn/sync
#[debug_handler]
async fn whcbqhn_sync(
    State(_ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:whcbqhn:sync").await?;
    format::json(ApiResponse::<String>::success_with_message("同步成功".to_string(), "微信公众号模板同步功能暂未对接微信API"))
}

/// 同步小程序订阅消息
///
/// 权限: admin:wechat:routine:sync
/// 路径: POST /api/admin/wechat/template/routine/sync
#[debug_handler]
async fn routine_sync(
    State(_ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:routine:sync").await?;
    format::json(ApiResponse::<String>::success_with_message("同步成功".to_string(), "小程序订阅消息同步功能暂未对接微信API"))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/wechat/template")
        .add("/whcbqhn/sync", post(whcbqhn_sync))
        .add("/routine/sync", post(routine_sync))
}
