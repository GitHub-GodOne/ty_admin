/// 运费模板区域运费 -- 控制器
///
/// Java参考: ShippingTemplatesRegionController
/// 路径前缀: /api/admin/express/shipping/region
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::shipping_templates::TempIdQuery;
use crate::services::shipping_templates_service::ShippingTemplatesService;

/// 根据模板id查询区域运费数据(分组)
/// GET /api/admin/express/shipping/region/list?tempId=
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    Query(params): Query<TempIdQuery>,
) -> Result<Response> {
    let response = ShippingTemplatesService::get_region_list_group(&ctx.db, params.temp_id).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/express/shipping/region")
        .add("/list", get(get_list))
}
