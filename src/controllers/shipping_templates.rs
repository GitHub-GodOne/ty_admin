/// 运费模板 -- 控制器
///
/// Java参考: ShippingTemplatesController
/// 路径前缀: /api/admin/express/shipping/templates
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::shipping_templates::*;
use crate::services::shipping_templates_service::ShippingTemplatesService;

/// 分页列表
/// GET /api/admin/express/shipping/templates/list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    Query(search): Query<ShippingTemplatesSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = ShippingTemplatesService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增运费模板
/// POST /api/admin/express/shipping/templates/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    Json(request): Json<ShippingTemplatesRequest>,
) -> Result<Response> {
    if ShippingTemplatesService::create(&ctx.db, &request).await? {
        format::json(ApiResponse::success("新增运费模板成功".to_string()))
    } else {
        format::json(ApiResponse::<String>::failed("新增运费模板失败"))
    }
}

/// 删除运费模板
/// GET /api/admin/express/shipping/templates/delete?id=
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if ShippingTemplatesService::remove(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改运费模板
/// POST /api/admin/express/shipping/templates/update?id=
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
    Json(request): Json<ShippingTemplatesRequest>,
) -> Result<Response> {
    if ShippingTemplatesService::update(&ctx.db, params.id, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 运费模板详情
/// GET /api/admin/express/shipping/templates/info?id=
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let response = ShippingTemplatesService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/express/shipping/templates")
        .add("/list", get(get_list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
}
