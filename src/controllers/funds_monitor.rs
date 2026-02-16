/// 资金监控 -- 控制器
///
/// Java参考: FundsMonitorController
/// 路由前缀: /api/admin/finance/founds/monitor
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::funds_monitor::*;
use crate::dtos::common::PageParamRequest;
use crate::services::funds_monitor_service::FundsMonitorService;
use crate::utils::auth;

/// 1. 资金监控列表
///
/// 权限: admin:finance:monitor:list
/// 路径: GET /api/admin/finance/founds/monitor/list
/// Java: FundsMonitorController.getList()
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<FundsMonitorRequest>,
    Query(page_param): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:finance:monitor:list").await?;

    let response = FundsMonitorService::fund_monitoring(
        &ctx.db, &request, &page_param,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 2. 佣金记录列表
///
/// 权限: admin:finance:monitor:brokerage:record
/// 路径: GET /api/admin/finance/founds/monitor/brokerage/record
/// Java: FundsMonitorController.brokerageRecord()
#[debug_handler]
async fn brokerage_record(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<BrokerageRecordRequest>,
    Query(page_param): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:finance:monitor:brokerage:record").await?;

    let response = FundsMonitorService::get_brokerage_record(
        &ctx.db, &request, &page_param,
    ).await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/finance/founds/monitor")
        .add("/list", get(list))
        .add("/brokerage/record", get(brokerage_record))
}
