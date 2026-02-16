/// 微信小程序公共控制器
///
/// Java参考: com.zbkj.admin.pub.WechatMiniCommonController
/// 路径前缀: /api/public/wechat/mini
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::services::wechat_mini_service::WechatMiniService;

/// 获取微信小程序码
/// POST /api/public/wechat/mini/get/qrcode
/// Java: WechatMiniCommonController.getWecahtQrCode()
/// 参数: path 和 scene 不能为空
/// 参考: https://developers.weixin.qq.com/miniprogram/dev/OpenApiDoc/qrcode-link/qr-code/getUnlimitedQRCode.html
#[debug_handler]
async fn get_wechat_qr_code(
    State(ctx): State<AppContext>,
    axum::Json(data): axum::Json<serde_json::Value>,
) -> Result<Response> {
    let response = WechatMiniService::get_wechat_qr_code(&ctx.db, &data).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/public/wechat/mini")
        .add("/get/qrcode", post(get_wechat_qr_code))
}
