/// 运费模板包邮区域 -- 控制器
///
/// Java参考: ShippingTemplatesFreeController
/// 路径前缀: /api/admin/express/shipping/free
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::shipping_templates::TempIdQuery;
use crate::services::shipping_templates_service::ShippingTemplatesService;

/// 根据模板id查询包邮数据(分组)
/// GET /api/admin/express/shipping/free/list?tempId=
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    Query(params): Query<TempIdQuery>,
) -> Result<Response> {
    let response = ShippingTemplatesService::get_free_list_group(&ctx.db, params.temp_id).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/express/shipping/free")
        .add("/list", get(get_list))
}
