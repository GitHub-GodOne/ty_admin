/// 权限验证宏
///
/// 简化权限验证的使用，类似Java的@PreAuthorize注解
///
/// 使用示例：
/// ```rust
/// #[require_permission("admin:statistics:home:index")]
/// async fn index_date(State(ctx): State<AppContext>) -> Result<Response> {
///     // 函数体
/// }
/// ```

/// 检查请求是否有指定权限
///
/// 这是一个辅助函数，用于在handler中手动检查权限
pub async fn check_permission(
    redis: &crate::utils::redis_client::RedisClient,
    req: &axum::extract::Request,
    required_permission: &str,
) -> Result<crate::utils::auth::LoginUserVo, axum::http::StatusCode> {
    use crate::utils::auth;

    // 获取Token
    let token = auth::get_token_from_request(req)
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    // 获取登录用户信息
    let login_user = auth::get_login_user(redis, &token)
        .await
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    // 验证Token是否有效
    if !auth::verify_token(&login_user) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    // 检查权限
    if !auth::has_authority(&login_user, required_permission) {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    Ok(login_user)
}

/// 从请求中获取当前登录用户（不检查权限）
pub async fn get_current_user(
    redis: &crate::utils::redis_client::RedisClient,
    req: &axum::extract::Request,
) -> Result<crate::utils::auth::LoginUserVo, axum::http::StatusCode> {
    use crate::utils::auth;

    let token = auth::get_token_from_request(req)
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let login_user = auth::get_login_user(redis, &token)
        .await
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    if !auth::verify_token(&login_user) {
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    Ok(login_user)
}
