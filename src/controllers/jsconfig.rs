/// 公共JS配置 -- 控制器
///
/// Java参考: com.zbkj.admin.pub.GetJSConfig
/// 路径前缀: /api/public/jsconfig
use loco_rs::prelude::*;

use crate::common::constants;
use crate::common::response::ApiResponse;
use crate::services::system_config_service::SystemConfigService;
use crate::services::system_attachment_service::SystemAttachmentService;

/// 获取移动端域名
/// GET /api/public/jsconfig/get/front/domain
/// Java: systemConfigService.getFrontDomain() → getValueByKey("site_url")
#[debug_handler]
async fn get_front_domain(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let domain = SystemConfigService::get_value_by_key(&ctx.db, constants::CONFIG_KEY_SITE_URL)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(domain))
}

/// 获取平台当前的素材地址
/// GET /api/public/jsconfig/get/admin/mediadomain
/// Java: systemConfigService.getMediaDomain() → systemAttachmentService.getCdnUrl()
#[debug_handler]
async fn get_media_domain(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let domain = SystemAttachmentService::get_cdn_url(&ctx.db)
        .await
        .map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(domain))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/public/jsconfig")
        .add("/get/front/domain", get(get_front_domain))
        .add("/get/admin/mediadomain", get(get_media_domain))
}
