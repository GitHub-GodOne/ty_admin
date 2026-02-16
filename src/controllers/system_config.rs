/// 系统配置控制器
///
/// 实现与Java版本完全一致的系统配置接口
/// Java代码参考: com.zbkj.admin.controller.SystemConfigController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::system_config::*;
use crate::services::system_config_service::SystemConfigService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 根据formId获取表单详情
///
/// 权限: admin:system:config:info
/// 路径: GET /api/admin/system/config/info?formId=X
/// Java: @PreAuthorize("hasAuthority('admin:system:config:info')")
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<FormIdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:info").await?;

    let response = SystemConfigService::info(&ctx.db, params.form_id)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::success(response))
}

/// 2. 保存表单配置
///
/// 权限: admin:system:config:save:form
/// 路径: POST /api/admin/system/config/save/form
/// Java: @PreAuthorize("hasAuthority('admin:system:config:save:form')")
#[debug_handler]
async fn save_form(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(params): Json<SystemFormCheckRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:save:form").await?;

    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    SystemConfigService::save_form(&ctx.db, &redis, &params)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::<String>::success_empty())
}

/// 3. 获取文件上传类型
///
/// 权限: admin:system:config:upload:type
/// 路径: GET /api/admin/system/config/get/upload/type
/// Java: @PreAuthorize("hasAuthority('admin:system:config:upload:type')")
#[debug_handler]
async fn get_upload_type(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:upload:type").await?;

    let config = SystemConfigService::get_file_upload_type(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    match config {
        Some(c) => {
            let response: SystemConfigResponse = c.into();
            format::json(ApiResponse::success(response))
        }
        None => format::json(ApiResponse::<SystemConfigResponse>::success_empty()),
    }
}

/// 4. 获取管理端站点Logo
///
/// 权限: 无需权限（Java中注释掉了权限检查）
/// 路径: GET /api/admin/system/config/get/site/logo
/// Java: 无 @PreAuthorize 注解
#[debug_handler]
async fn get_site_logo(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let response = SystemConfigService::get_site_logo(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::success(response))
}

/// 5. 获取腾讯地图Key
///
/// 权限: admin:system:config:tx:map:key
/// 路径: GET /api/admin/system/config/get/tx/map/key
/// Java: @PreAuthorize("hasAuthority('admin:system:config:tx:map:key')")
#[debug_handler]
async fn get_tx_map_key(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:tx:map:key").await?;

    let config = SystemConfigService::get_tx_map_key(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    match config {
        Some(c) => {
            let response: SystemConfigResponse = c.into();
            format::json(ApiResponse::success(response))
        }
        None => format::json(ApiResponse::<SystemConfigResponse>::success_empty()),
    }
}

/// 6. 获取移动端首页列表样式
///
/// 权限: admin:system:config:home:page:list:style
/// 路径: GET /api/admin/system/config/get/home/page/list/style
/// Java: @PreAuthorize("hasAuthority('admin:system:config:home:page:list:style')")
#[debug_handler]
async fn get_home_page_list_style(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:home:page:list:style").await?;

    let config = SystemConfigService::get_home_page_sale_list_style(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    match config {
        Some(c) => {
            let response: SystemConfigResponse = c.into();
            format::json(ApiResponse::success(response))
        }
        None => format::json(ApiResponse::<SystemConfigResponse>::success_empty()),
    }
}

// /// 查询参数（空，用于无参数的POST请求）
// #[derive(Debug, Deserialize)]
// struct EmptyQuery {}

/// 7. 保存移动端首页列表样式
///
/// 权限: admin:system:config:home:page:list:style:save
/// 路径: POST /api/admin/system/config/save/home/page/list/style
/// Java: @PreAuthorize("hasAuthority('admin:system:config:home:page:list:style:save')")
#[debug_handler]
async fn save_home_page_list_style(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(params): Json<SaveConfigRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:home:page:list:style:save").await?;

    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    SystemConfigService::save_home_page_sale_list_style(&ctx.db, &redis, &params)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::<String>::success_empty())
}

/// 8. 获取授权Host
///
/// 权限: admin:system:config:auth:host
/// 路径: GET /api/admin/system/config/get/auth/host
/// Java: @PreAuthorize("hasAuthority('admin:system:config:auth:host')")
#[debug_handler]
async fn get_auth_host(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:auth:host").await?;

    let config = SystemConfigService::get_auth_host(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    match config {
        Some(c) => {
            let response: SystemConfigResponse = c.into();
            format::json(ApiResponse::success(response))
        }
        None => format::json(ApiResponse::<SystemConfigResponse>::success_empty()),
    }
}

/// 9. 获取主题颜色
///
/// 权限: admin:system:config:change:color
/// 路径: GET /api/admin/system/config/get/change/color
/// Java: @PreAuthorize("hasAuthority('admin:system:config:change:color')")
#[debug_handler]
async fn get_change_color(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:change:color").await?;

    let config = SystemConfigService::get_change_color(&ctx.db)
        .await
        .map_err(|e| Error::from(e))?;

    match config {
        Some(c) => {
            let response: SystemConfigResponse = c.into();
            format::json(ApiResponse::success(response))
        }
        None => format::json(ApiResponse::<SystemConfigResponse>::success_empty()),
    }
}

/// 10. 保存主题颜色
///
/// 权限: admin:system:config:change:color:save
/// 路径: POST /api/admin/system/config/save/change/color
/// Java: @PreAuthorize("hasAuthority('admin:system:config:change:color:save')")
#[debug_handler]
async fn save_change_color(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(params): Json<SaveConfigRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:change:color:save").await?;

    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    SystemConfigService::save_change_color(&ctx.db, &redis, &params)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::<String>::success_empty())
}

/// 11. 清除配置缓存
///
/// 权限: admin:system:config:clear:cache
/// 路径: POST /api/admin/system/config/clear/cache
/// Java: @PreAuthorize("hasAuthority('admin:system:config:clear:cache')")
#[debug_handler]
async fn clear_cache(
    State(_ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:config:clear:cache").await?;

    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    SystemConfigService::clear_cache(&redis)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::<String>::success_empty())
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/config")
        .add("/info", get(info))
        .add("/save/form", post(save_form))
        .add("/get/upload/type", get(get_upload_type))
        .add("/get/site/logo", get(get_site_logo))
        .add("/get/tx/map/key", get(get_tx_map_key))
        .add("/get/home/page/list/style", get(get_home_page_list_style))
        .add("/save/home/page/list/style", post(save_home_page_list_style))
        .add("/get/auth/host", get(get_auth_host))
        .add("/get/change/color", get(get_change_color))
        .add("/save/change/color", post(save_change_color))
        .add("/clear/cache", post(clear_cache))
}
