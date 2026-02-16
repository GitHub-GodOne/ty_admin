/// 统计 -- 主页控制器
///
/// 实现与Java版本完全一致的8个接口，包含权限验证
/// Java代码参考: com.zbkj.admin.controller.HomeController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::services::home_service::HomeService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 首页数据 - 今日/昨日对比
///
/// 权限: admin:statistics:home:index
/// 路径: GET /api/admin/statistics/home/index
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:index')")
#[debug_handler]
async fn index_date(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:index").await?;

    let response = HomeService::index_date(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 用户曲线图 - 最近30天新增用户趋势
///
/// 权限: admin:statistics:home:chart:user
/// 路径: GET /api/admin/statistics/home/chart/user
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:chart:user')")
#[debug_handler]
async fn chart_user(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:chart:user").await?;

    let response = HomeService::chart_user(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 3. 用户购买统计
///
/// 权限: admin:statistics:home:chart:user:buy
/// 路径: GET /api/admin/statistics/home/chart/user/buy
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:chart:user:buy')")
#[debug_handler]
async fn chart_user_buy(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:chart:user:buy").await?;

    let response = HomeService::chart_user_buy(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 4. 30天订单量趋势
///
/// 权限: admin:statistics:home:chart:order
/// 路径: GET /api/admin/statistics/home/chart/order
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:chart:order')")
#[debug_handler]
async fn chart_order(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:chart:order").await?;

    let response = HomeService::chart_order(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 5. 周订单量趋势（本周 vs 上周）
///
/// 权限: admin:statistics:home:chart:order:week
/// 路径: GET /api/admin/statistics/home/chart/order/week
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:chart:order:week')")
#[debug_handler]
async fn chart_order_in_week(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:chart:order:week").await?;

    let response = HomeService::chart_order_in_week(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 6. 月订单量趋势（本月 vs 上月）
///
/// 权限: admin:statistics:home:chart:order:month
/// 路径: GET /api/admin/statistics/home/chart/order/month
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:chart:order:month')")
#[debug_handler]
async fn chart_order_in_month(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:chart:order:month").await?;

    let response = HomeService::chart_order_in_month(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 7. 年订单量趋势（今年 vs 去年，按月）
///
/// 权限: admin:statistics:home:chart:order:year
/// 路径: GET /api/admin/statistics/home/chart/order/year
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:chart:order:year')")
#[debug_handler]
async fn chart_order_in_year(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:chart:order:year").await?;

    let response = HomeService::chart_order_in_year(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 8. 首页经营数据
///
/// 权限: admin:statistics:home:operating:data
/// 路径: GET /api/admin/statistics/home/operating/data
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:operating:data')")
#[debug_handler]
async fn operating_data(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:statistics:home:operating:data").await?;

    let response = HomeService::operating_data(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/statistics/home")
        .add("/index", get(index_date))
        .add("/chart/user", get(chart_user))
        .add("/chart/user/buy", get(chart_user_buy))
        .add("/chart/order", get(chart_order))
        .add("/chart/order/week", get(chart_order_in_week))
        .add("/chart/order/month", get(chart_order_in_month))
        .add("/chart/order/year", get(chart_order_in_year))
        .add("/operating/data", get(operating_data))
}
