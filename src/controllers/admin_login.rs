/// 管理端登录服务控制器（重构版）
///
/// 使用统一的 ApiResponse 和 AdminService
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::admin::*;
use crate::services::admin_service::AdminService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. PC端管理员登录
///
/// 权限: 无需权限
/// 路径: POST /api/admin/login
#[debug_handler]
async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<AdminLoginRequest>,
) -> Result<Response> {
    // 获取 Redis 客户端
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis客户端失败: {}", e)))?;

    // 调用 Service
    let response = AdminService::login(&ctx.db, &redis, &params)
        .await
        .map_err(|e| Error::from(e))?;

    // 返回JSON（与Java一致，前端自行将token写入cookie/header）
    format::json(ApiResponse::success(response))
}

/// 2. PC端管理员登出
///
/// 权限: admin:logout
/// 路径: GET /api/admin/logout
#[debug_handler]
async fn logout(
    State(_ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    // 获取Redis客户端
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    // 从请求头或Cookie获取Token
    if let Some(token) = auth::extract_token_from_headers(&headers) {
        // 从Redis删除token（key格式与auth.rs一致：TOKEN:ADMIN:{uuid}）
        let redis_key = format!("TOKEN:ADMIN:{}", token);
        let _ = redis.del(&redis_key).await;
    }

    format::json(ApiResponse::<()>::success_empty())
}

/// 3. 获取用户详情
///
/// 权限: admin:info
/// 路径: GET /api/admin/getAdminInfoByToken
#[debug_handler]
async fn get_admin_info_by_token(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    // 获取Redis客户端
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    // 从请求头或Cookie获取Token
    let token = auth::extract_token_from_headers(&headers)
        .ok_or_else(|| Error::Unauthorized("未提供Token".to_string()))?;

    // 调用 Service
    let response = AdminService::get_admin_info_by_token(&ctx.db, &redis, &token)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::success(response))
}

/// 4. 获取登录页图片
///
/// 权限: 无需权限
/// 路径: GET /api/admin/getLoginPic
#[debug_handler]
async fn get_login_pic(
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    // TODO: 从system_config表读取配置
    // TODO: 从system_group_data表读取轮播图

    let mut map = std::collections::HashMap::new();
    map.insert("backgroundImage", "");
    map.insert("logo", "");
    map.insert("loginLogo", "");
    map.insert("siteName", "CRMEB");

    format::json(ApiResponse::success(map))
}

/// 5. 获取管理员可访问目录
///
/// 权限: admin:login:menus
/// 路径: GET /api/admin/getMenus
#[debug_handler]
async fn get_menus(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    // 获取Redis客户端
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    // 从请求头或Cookie获取Token
    let token = auth::extract_token_from_headers(&headers)
        .ok_or_else(|| Error::Unauthorized("未提供Token".to_string()))?;

    // 调用 Service
    let menus = AdminService::get_menus(&ctx.db, &redis, &token)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::success(menus))
}

/// 6. 账号检测
///
/// 权限: admin:login:account:detection
/// 路径: POST /api/admin/login/account/detection
#[debug_handler]
async fn account_detection(
    State(ctx): State<AppContext>,
    Json(params): Json<AccountDetectionRequest>,
) -> Result<Response> {
    // 调用 Service
    let exists = AdminService::account_detection(&ctx.db, &params.account, params.id)
        .await
        .map_err(|e| Error::from(e))?;

    if exists {
        return format::json(ApiResponse::<()>::error(400, "账号已存在"));
    }

    format::json(ApiResponse::<()>::success_empty())
}

/// 7. 修改登录用户信息
///
/// 权限: admin:login:admin:update
/// 路径: POST /api/admin/login/admin/update
#[debug_handler]
async fn update_login_admin(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(params): Json<AdminSaveRequest>,
) -> Result<Response> {
    // 获取Redis客户端
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    // 从请求头或Cookie获取Token
    let token = auth::extract_token_from_headers(&headers)
        .ok_or_else(|| Error::Unauthorized("未提供Token".to_string()))?;

    // 从Redis获取登录用户信息
    let login_user = auth::get_login_user(&redis, &token)
        .await
        .ok_or_else(|| Error::Unauthorized("未登录或Token已过期".to_string()))?;

    // 调用 Service
    AdminService::update_login_admin(&ctx.db, login_user.user_id, &params.real_name)
        .await
        .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::<()>::success_empty())
}

/// 8. 修改登录用户密码
///
/// 权限: admin:login:update:password
/// 路径: POST /api/admin/login/update/password
#[debug_handler]
async fn update_password(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(params): Json<UpdatePasswordRequest>,
) -> Result<Response> {
    // 参数验证
    if params.new_password != params.confirm_password {
        return format::json(ApiResponse::<()>::error(400, "两次密码输入不一致"));
    }

    // 获取Redis客户端
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| Error::string(&format!("获取Redis失败: {}", e)))?;

    // 从请求头或Cookie获取Token
    let token = auth::extract_token_from_headers(&headers)
        .ok_or_else(|| Error::Unauthorized("未提供Token".to_string()))?;

    // 从Redis获取登录用户信息
    let login_user = auth::get_login_user(&redis, &token)
        .await
        .ok_or_else(|| Error::Unauthorized("未登录或Token已过期".to_string()))?;

    // 调用 Service
    AdminService::update_password(
        &ctx.db,
        login_user.user_id,
        &params.old_password,
        &params.new_password,
    )
    .await
    .map_err(|e| Error::from(e))?;

    format::json(ApiResponse::<()>::success_empty())
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin")
        .add("/login", post(login))
        .add("/logout", get(logout))
        .add("/getAdminInfoByToken", get(get_admin_info_by_token))
        .add("/getLoginPic", get(get_login_pic))
        .add("/getMenus", get(get_menus))
        .add("/login/account/detection", post(account_detection))
        .add("/login/admin/update", post(update_login_admin))
        .add("/login/update/password", post(update_password))
}
