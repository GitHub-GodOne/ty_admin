/// 一号通服务 -- 控制器
///
/// 实现与Java版本一致的一号通服务管理接口
/// Java代码参考: com.zbkj.admin.controller.OnePassController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::one_pass::*;
use crate::services::one_pass_service::OnePassService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 一号通 应用保存
///
/// 权限: admin:pass:appsave
/// 路径: POST /api/admin/pass/appsave
/// Java: OnePassController.saveOnePassApplication()
/// 保存一号通应用的 accessKey 和 secretKey 到系统配置
#[debug_handler]
async fn app_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<OnePassLoginRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pass:appsave").await?;

    let result = OnePassService::save_application_info(&ctx.db, &request).await?;
    format::json(ApiResponse::success(result))
}

/// 2. 一号通 应用详情获取
///
/// 权限: admin:pass:appget
/// 路径: GET /api/admin/pass/appget
/// Java: OnePassController.getOnePassApplication()
/// 获取一号通应用的 accessKey 和 secretKey
#[debug_handler]
async fn app_get(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pass:appget").await?;

    let result = OnePassService::get_application_info(&ctx.db).await?;
    format::json(ApiResponse::success(result))
}

/// 3. 一号通 取消商家寄件
///
/// 权限: admin:pass:shipment:cancel
/// 路径: POST /api/admin/pass/shipment/cancel
/// Java: OnePassController.onePassShipmentCancel()
/// 调用一号通API取消商家寄件订单
#[debug_handler]
async fn shipment_cancel(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<OnePassShipmentCancelOrderRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pass:shipment:cancel").await?;

    let result = OnePassService::shipment_cancel_order(&ctx.db, &request).await?;
    format::json(ApiResponse::success(result))
}

/// 4. 一号通 商家寄件 快递列表
///
/// 权限: admin:pass:shipment:express
/// 路径: GET /api/admin/pass/shipment/express
/// Java: OnePassController.onePassShipmentExpress()
/// 获取商家寄件可用的快递公司列表
#[debug_handler]
async fn shipment_express(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pass:shipment:express").await?;

    let result = OnePassService::shipment_coms(&ctx.db).await?;
    format::json(ApiResponse::success(result))
}

/// 5. 一号通 商家寄件 回调地址
///
/// 权限: admin:pass:shipment:callback
/// 路径: POST /api/admin/pass/shipment/callback
/// Java: OnePassController.onePassShipmentCallBack()
/// 处理一号通商家寄件的回调通知
#[debug_handler]
async fn shipment_callback(
    State(_ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<OnePassShipmentCallBackRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:pass:shipment:callback").await?;

    let callback_type = request.callback_type.as_deref().unwrap_or("");
    let data = request.data.as_deref().unwrap_or("");

    let result = OnePassService::shipment_callback(callback_type, data).await?;
    format::json(ApiResponse::success(result))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/pass")
        .add("/appsave", post(app_save))
        .add("/appget", get(app_get))
        .add("/shipment/cancel", post(shipment_cancel))
        .add("/shipment/express", get(shipment_express))
        .add("/shipment/callback", post(shipment_callback))
}
