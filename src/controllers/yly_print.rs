/// 易联云打印订单 -- 控制器
///
/// Java参考: YlyPrintController
/// 路径前缀: /api/admin/yly
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::yly_print::YlyPrintQueryRequest;
use crate::services::yly_print_service::YlyPrintService;

/// 打印小票
/// GET /api/admin/yly/print?ordid=xxx
/// Java: GET /api/admin/yly/print/{ordid}
#[debug_handler]
async fn print_order(
    State(ctx): State<AppContext>,
    Query(params): Query<YlyPrintQueryRequest>,
) -> Result<Response> {
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| loco_rs::Error::string(&e.to_string()))?;

    YlyPrintService::yly_print(&ctx.db, &redis, &params.ordid, false).await?;
    format::json(ApiResponse::<String>::success_empty())
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/yly")
        .add("/print", get(print_order))
}
