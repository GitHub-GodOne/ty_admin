/// Redis初始化器
///
/// 在应用启动时初始化Redis连接池
use async_trait::async_trait;
use loco_rs::{app::Initializer, prelude::*, Result};
use std::sync::Arc;
use tokio::sync::OnceCell;

use crate::utils::redis_client::RedisClient;

/// 全局Redis客户端
static REDIS_CLIENT: OnceCell<Arc<RedisClient>> = OnceCell::const_new();

/// 获取全局Redis客户端
pub async fn get_redis() -> Result<Arc<RedisClient>> {
    REDIS_CLIENT
        .get()
        .cloned()
        .ok_or_else(|| Error::string("Redis未初始化"))
}

/// Redis初始化器
pub struct RedisInitializer;

#[async_trait]
impl Initializer for RedisInitializer {
    fn name(&self) -> String {
        "redis".to_string()
    }

    async fn after_routes(&self, router: axum::Router, ctx: &AppContext) -> Result<axum::Router> {
        // 从配置中读取Redis URL
        let redis_url = ctx
            .config
            .settings
            .as_ref()
            .and_then(|s| s.get("redis"))
            .and_then(|r| r.get("uri"))
            .and_then(|u| u.as_str())
            .unwrap_or("redis://127.0.0.1:6379");

        tracing::info!("正在连接Redis: {}", redis_url);

        // 创建Redis客户端
        let client = RedisClient::new(redis_url)
            .await
            .map_err(|e| Error::string(&format!("Redis连接失败: {}", e)))?;

        // 设置全局客户端
        REDIS_CLIENT
            .set(Arc::new(client))
            .map_err(|_| Error::string("Redis客户端已初始化"))?;

        tracing::info!("Redis连接成功");

        Ok(router)
    }
}
