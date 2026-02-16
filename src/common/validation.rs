// Placeholder for validation middleware
// This will be implemented when we add validator crate support

use axum::{extract::Request, middleware::Next, response::Response};
use loco_rs::prelude::*;

/// 自动验证中间件
/// TODO: Implement validation logic when validator crate is added
pub async fn validate_request(req: Request, next: Next) -> Result<Response> {
    // For now, just pass through
    Ok(next.run(req).await)
}
