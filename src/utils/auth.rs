/// JWT认证和权限验证模块
///
/// 实现与Java版本一致的Token验证和权限检查
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

/// 登录用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUserVo {
    /// Token
    pub token: String,

    /// 用户ID
    pub user_id: i32,

    /// 用户账号
    pub account: String,

    /// 用户角色（逗号分隔）
    pub roles: String,

    /// 权限列表
    pub permissions: Vec<String>,

    /// 登录时间（毫秒时间戳）
    pub login_time: i64,

    /// 过期时间（毫秒时间戳）
    pub expire_time: i64,
}

/// 从请求头获取Token
///
/// Java: Constants.HEADER_AUTHORIZATION_KEY = "Authori-zation"
/// 注意：这里的拼写是故意的，与Java保持一致
pub fn get_token_from_request(req: &Request) -> Option<String> {
    // 从请求头获取 Authori-zation
    let auth_header = req.headers().get("Authori-zation")?;
    let auth_str = auth_header.to_str().ok()?;

    // 如果以 "TOKEN:ADMIN:" 开头，去掉前缀
    if auth_str.starts_with("TOKEN:ADMIN:") {
        Some(auth_str.replace("TOKEN:ADMIN:", ""))
    } else {
        Some(auth_str.to_string())
    }
}

/// 生成UUID Token
///
/// Java: UUID.randomUUID().toString().replace("-", "")
pub fn generate_token() -> String {
    uuid::Uuid::new_v4().to_string().replace("-", "")
}

/// 创建Token并存储到Redis
///
/// Java: TokenComponent.createToken(loginUser)
pub async fn create_token(
    redis: &crate::utils::redis_client::RedisClient,
    user_id: i32,
    account: String,
    roles: String,
    permissions: Vec<String>,
) -> Result<String, String> {
    let token = generate_token();
    let now = chrono::Utc::now().timestamp_millis();

    // Token有效期：5小时（与Java一致）
    let expire_minutes = 5 * 60;
    let expire_time = now + (expire_minutes * 60 * 1000);

    let login_user = LoginUserVo {
        token: token.clone(),
        user_id,
        account,
        roles,
        permissions,
        login_time: now,
        expire_time,
    };

    // 存储到Redis
    let redis_key = format!("TOKEN:ADMIN:{}", token);
    redis
        .set_json(&redis_key, &login_user, (expire_minutes * 60) as usize)
        .await
        .map_err(|e| format!("Redis存储失败: {}", e))?;

    Ok(token)
}

/// 从Redis获取登录用户信息
///
/// Java: TokenComponent.getLoginUser(request)
pub async fn get_login_user(
    redis: &crate::utils::redis_client::RedisClient,
    token: &str,
) -> Option<LoginUserVo> {
    // 从Redis获取
    let redis_key = format!("TOKEN:ADMIN:{}", token);
    redis.get_json(&redis_key).await.ok().flatten()
}

/// 验证Token是否有效
///
/// Java: TokenComponent.verifyToken(loginUser)
pub fn verify_token(login_user: &LoginUserVo) -> bool {
    let now = chrono::Utc::now().timestamp_millis();

    // 检查是否过期
    if now > login_user.expire_time {
        return false;
    }

    // TODO: 如果距离过期时间不足20分钟，自动刷新
    // let millis_minute_ten = 20 * 60 * 1000;
    // if login_user.expire_time - now <= millis_minute_ten {
    //     refresh_token(login_user).await;
    // }

    true
}

/// 检查用户是否有指定权限
///
/// Java: @PreAuthorize("hasAuthority('admin:statistics:home:index')")
pub fn has_authority(login_user: &LoginUserVo, authority: &str) -> bool {
    // 如果角色包含1（超级管理员），拥有所有权限
    if login_user.roles.split(',').any(|r| r == "1") {
        return true;
    }

    // 检查权限列表
    login_user.permissions.iter().any(|p| p == authority || p == "*:*:*")
}

/// 从请求头或Cookie中提取Token（去掉 TOKEN:ADMIN: 前缀）
///
/// 统一的Token提取函数，供所有Controller使用
/// 支持从 Authori-zation 请求头和 Cookie 中提取
pub fn extract_token_from_headers(headers: &axum::http::HeaderMap) -> Option<String> {
    // 1. 尝试从 Authori-zation 请求头获取
    if let Some(auth_header) = headers.get("Authori-zation") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("TOKEN:ADMIN:") {
                return Some(auth_str.replace("TOKEN:ADMIN:", ""));
            }
            return Some(auth_str.to_string());
        }
    }

    // 2. 尝试从 Cookie 获取
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some(token) = cookie.strip_prefix("Authori-zation=") {
                    if token.starts_with("TOKEN:ADMIN:") {
                        return Some(token.replace("TOKEN:ADMIN:", ""));
                    }
                    return Some(token.to_string());
                }
            }
        }
    }

    None
}

/// 从请求头中提取Token并验证权限
///
/// 统一的权限检查函数，供所有Controller使用
/// 对应Java的 @PreAuthorize("hasAuthority('xxx')")
/// 流程：提取Token -> 从Redis获取用户 -> 验证Token有效性 -> 检查权限
pub async fn check_permission(
    headers: &axum::http::HeaderMap,
    required_permission: &str,
) -> Result<LoginUserVo, loco_rs::Error> {
    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| loco_rs::Error::string(&format!("获取Redis失败: {}", e)))?;

    let token = extract_token_from_headers(headers)
        .ok_or_else(|| loco_rs::Error::Unauthorized("未提供Token".to_string()))?;

    let login_user = get_login_user(&redis, &token)
        .await
        .ok_or_else(|| loco_rs::Error::Unauthorized("未登录或Token已过期".to_string()))?;

    if !verify_token(&login_user) {
        return Err(loco_rs::Error::Unauthorized("Token已过期".to_string()));
    }

    if !has_authority(&login_user, required_permission) {
        return Err(loco_rs::Error::Unauthorized(format!(
            "没有权限: {}",
            required_permission
        )));
    }

    Ok(login_user)
}

/// 权限验证中间件
///
/// 使用方式：
/// ```rust
/// .route_layer(middleware::from_fn_with_state(
///     ctx.clone(),
///     |State(ctx): State<AppContext>, req, next| {
///         require_auth(ctx, req, next, "admin:statistics:home:index")
///     }
/// ))
/// ```
pub async fn require_auth(
    _ctx: AppContext,
    req: Request,
    next: Next,
    required_permission: &str,
) -> Result<Response, StatusCode> {
    // 获取Redis客户端
    let redis = match crate::initializers::redis::get_redis().await {
        Ok(r) => r,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // 获取Token
    let token = match get_token_from_request(&req) {
        Some(t) => t,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 获取登录用户信息
    let login_user = match get_login_user(&redis, &token).await {
        Some(user) => user,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 验证Token是否有效
    if !verify_token(&login_user) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 检查权限
    if !has_authority(&login_user, required_permission) {
        return Err(StatusCode::FORBIDDEN);
    }

    // 权限验证通过，继续处理请求
    Ok(next.run(req).await)
}

/// 从请求中获取当前登录用户
///
/// Java: SecurityUtil.getLoginUserVo()
pub async fn get_current_user(req: &Request) -> Option<LoginUserVo> {
    // 获取Redis客户端
    let redis = crate::initializers::redis::get_redis().await.ok()?;
    let token = get_token_from_request(req)?;
    get_login_user(&redis, &token).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let token = generate_token();
        assert_eq!(token.len(), 32); // UUID without dashes
        assert!(!token.contains('-'));
    }

    #[test]
    fn test_has_authority_super_admin() {
        let user = LoginUserVo {
            token: "test".to_string(),
            user_id: 1,
            account: "admin".to_string(),
            roles: "1".to_string(), // 超级管理员
            permissions: vec![],
            login_time: 0,
            expire_time: 0,
        };

        assert!(has_authority(&user, "any:permission"));
    }

    #[test]
    fn test_has_authority_specific() {
        let user = LoginUserVo {
            token: "test".to_string(),
            user_id: 2,
            account: "user".to_string(),
            roles: "2".to_string(),
            permissions: vec![
                "admin:statistics:home:index".to_string(),
                "admin:logout".to_string(),
            ],
            login_time: 0,
            expire_time: 0,
        };

        assert!(has_authority(&user, "admin:statistics:home:index"));
        assert!(has_authority(&user, "admin:logout"));
        assert!(!has_authority(&user, "admin:user:delete"));
    }

    #[test]
    fn test_verify_token_expired() {
        let now = chrono::Utc::now().timestamp_millis();
        let user = LoginUserVo {
            token: "test".to_string(),
            user_id: 1,
            account: "admin".to_string(),
            roles: "1".to_string(),
            permissions: vec![],
            login_time: now - 10000,
            expire_time: now - 1000, // 已过期
        };

        assert!(!verify_token(&user));
    }

    #[test]
    fn test_verify_token_valid() {
        let now = chrono::Utc::now().timestamp_millis();
        let user = LoginUserVo {
            token: "test".to_string(),
            user_id: 1,
            account: "admin".to_string(),
            roles: "1".to_string(),
            permissions: vec![],
            login_time: now,
            expire_time: now + 1000000, // 未过期
        };

        assert!(verify_token(&user));
    }
}
