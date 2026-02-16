/// 系统通知 -- 控制器
///
/// 实现与Java版本一致的通知管理接口
/// Java代码参考: com.zbkj.admin.controller.SystemNotificationController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::IdQuery;
use crate::dtos::system_notification::*;
use crate::services::system_notification_service::SystemNotificationService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 系统通知列表
///
/// 权限: admin:system:notification:list
/// 路径: GET /api/admin/system/notification/list
/// Java: SystemNotificationController.getList()
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<NotificationSearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:notification:list").await?;

    let response = SystemNotificationService::get_list(&ctx.db, &search).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 公众号模板开关
///
/// 权限: admin:system:notification:wechat:switch
/// 路径: POST /api/admin/system/notification/wechat/switch
/// Java: SystemNotificationController.wechatSwitch()
#[debug_handler]
async fn wechat_switch(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:notification:wechat:switch").await?;

    if SystemNotificationService::wechat_switch(&ctx.db, params.id).await? {
        format::json(ApiResponse::success("更改成功".to_string()))
    } else {
        format::json(ApiResponse::<()>::failed("更改失败"))
    }
}

/// 3. 小程序订阅模板开关
///
/// 权限: admin:system:notification:routine:switch
/// 路径: POST /api/admin/system/notification/routine/switch
/// Java: SystemNotificationController.routineSwitch()
#[debug_handler]
async fn routine_switch(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:notification:routine:switch").await?;

    if SystemNotificationService::routine_switch(&ctx.db, params.id).await? {
        format::json(ApiResponse::success("更改成功".to_string()))
    } else {
        format::json(ApiResponse::<()>::failed("更改失败"))
    }
}

/// 4. 发送短信开关
///
/// 权限: admin:system:notification:sms:switch
/// 路径: POST /api/admin/system/notification/sms/switch
/// Java: SystemNotificationController.smsSwitch()
#[debug_handler]
async fn sms_switch(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:notification:sms:switch").await?;

    if SystemNotificationService::sms_switch(&ctx.db, params.id).await? {
        format::json(ApiResponse::success("更改成功".to_string()))
    } else {
        format::json(ApiResponse::<()>::failed("更改失败"))
    }
}

/// 5. 通知详情
///
/// 权限: admin:system:notification:detail
/// 路径: GET /api/admin/system/notification/detail
/// Java: SystemNotificationController.info()
#[debug_handler]
async fn detail(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<NotificationInfoRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:notification:detail").await?;

    let response = SystemNotificationService::get_detail(&ctx.db, &request).await?;
    format::json(ApiResponse::success(response))
}

/// 6. 修改通知
///
/// 权限: admin:system:notification:update
/// 路径: POST /api/admin/system/notification/update
/// Java: SystemNotificationController.update()
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<NotificationUpdateRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:notification:update").await?;

    if SystemNotificationService::modify(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改通知失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/notification")
        .add("/list", get(list))
        .add("/wechat/switch", post(wechat_switch))
        .add("/routine/switch", post(routine_switch))
        .add("/sms/switch", post(sms_switch))
        .add("/detail", get(detail))
        .add("/update", post(update))
}
