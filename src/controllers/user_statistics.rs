/// 统计 -- 用户统计控制器
///
/// 实现与Java版本完全一致的用户统计接口
/// Java代码参考: com.zbkj.admin.controller.UserStatisticsController
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::common::response::ApiResponse;
use crate::services::user_statistics_service::UserStatisticsService;

// ==================== 接口实现 ====================

/// 用户渠道数据
///
/// 权限: admin:statistics:user:channel
/// 路径: GET /api/admin/statistics/user/channel
///
/// 返回不同渠道(h5, ios, routine, wechat)的用户数量统计
#[debug_handler]
async fn get_channel_data(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let response = UserStatisticsService::get_channel_data(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 用户概览
///
/// 权限: admin:statistics:user:overview
/// 路径: GET /api/admin/statistics/user/overview
///
/// 参数:
/// - dateLimit: 时间范围参数
///   - day: 今天
///   - yesterday: 昨天
///   - lately7: 最近7天
///   - lately30: 最近30天
///   - week: 本周
///   - month: 本月
///   - year: 本年
///   - 自定义: "2024-01-01,2024-01-31"
#[debug_handler]
async fn get_overview(
    State(ctx): State<AppContext>,
    Query(params): Query<OverviewQuery>,
) -> Result<Response> {
    let date_limit = params.date_limit.unwrap_or_else(|| "lately7".to_string());
    let response = UserStatisticsService::get_overview(&ctx.db, &date_limit).await?;
    format::json(ApiResponse::success(response))
}

/// 查询参数
#[derive(Debug, Deserialize)]
struct OverviewQuery {
    #[serde(rename = "dateLimit")]
    date_limit: Option<String>,
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/statistics/user")
        .add("/channel", get(get_channel_data))
        .add("/overview", get(get_overview))
}
