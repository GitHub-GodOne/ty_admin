/// Excel导出 -- 控制器
///
/// Java参考: ExcelController
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::product::StoreProductSearchRequest;
use crate::dtos::store_order::StoreOrderSearchRequest;
use crate::dtos::store_bargain::StoreBargainSearchRequest;
use crate::dtos::store_combination::StoreCombinationSearchRequest;
use crate::dtos::excel::ExcelFileNameResponse;
use crate::services::excel_service::ExcelService;

/// 商品导出
/// GET /api/admin/export/excel/product
#[debug_handler]
async fn export_product(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreProductSearchRequest>,
) -> Result<Response> {
    let file_name = ExcelService::export_product(&ctx.db, &search).await?;
    format::json(ApiResponse::success(ExcelFileNameResponse { file_name }))
}

/// 砍价商品导出
/// GET /api/admin/export/excel/bargain/product
#[debug_handler]
async fn export_bargain_product(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreBargainSearchRequest>,
) -> Result<Response> {
    let file_name = ExcelService::export_bargain_product(&ctx.db, &search).await?;
    format::json(ApiResponse::success(ExcelFileNameResponse { file_name }))
}

/// 拼团商品导出 (注意: Java中URL有拼写错误 combiantion)
/// GET /api/admin/export/excel/combiantion/product
#[debug_handler]
async fn export_combination_product(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreCombinationSearchRequest>,
) -> Result<Response> {
    let file_name = ExcelService::export_combination_product(&ctx.db, &search).await?;
    format::json(ApiResponse::success(ExcelFileNameResponse { file_name }))
}

/// 订单导出
/// GET /api/admin/export/excel/order
#[debug_handler]
async fn export_order(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreOrderSearchRequest>,
) -> Result<Response> {
    let file_name = ExcelService::export_order(&ctx.db, &search).await?;
    format::json(ApiResponse::success(ExcelFileNameResponse { file_name }))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/export/excel")
        .add("/product", get(export_product))
        .add("/bargain/product", get(export_bargain_product))
        .add("/combiantion/product", get(export_combination_product))
        .add("/order", get(export_order))
}
