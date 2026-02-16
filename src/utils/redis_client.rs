/// Redis客户端管理
///
/// 提供Redis连接池和基本操作
use redis::{aio::ConnectionManager, AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};

/// Redis客户端
#[derive(Clone)]
pub struct RedisClient {
    manager: ConnectionManager,
}

impl RedisClient {
    /// 创建Redis客户端
    pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = redis::Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;
        Ok(Self { manager })
    }

    /// 设置字符串值
    pub async fn set(&self, key: &str, value: &str, expire_seconds: usize) -> Result<(), RedisError> {
        let mut conn = self.manager.clone();
        conn.set_ex(key, value, expire_seconds as u64).await
    }

    /// 获取字符串值
    pub async fn get(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut conn = self.manager.clone();
        conn.get(key).await
    }

    /// 删除键
    pub async fn del(&self, key: &str) -> Result<(), RedisError> {
        let mut conn = self.manager.clone();
        conn.del(key).await
    }

    /// 设置JSON对象
    pub async fn set_json<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        expire_seconds: usize,
    ) -> Result<(), RedisError> {
        let json = serde_json::to_string(value)
            .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "JSON序列化失败", e.to_string())))?;
        self.set(key, &json, expire_seconds).await
    }

    /// 获取JSON对象
    pub async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> Result<Option<T>, RedisError> {
        let value: Option<String> = self.get(key).await?;
        match value {
            Some(json) => {
                let obj = serde_json::from_str(&json)
                    .map_err(|e| RedisError::from((redis::ErrorKind::TypeError, "JSON反序列化失败", e.to_string())))?;
                Ok(Some(obj))
            }
            None => Ok(None),
        }
    }

    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> Result<bool, RedisError> {
        let mut conn = self.manager.clone();
        conn.exists(key).await
    }

    /// 设置过期时间
    pub async fn expire(&self, key: &str, seconds: usize) -> Result<(), RedisError> {
        let mut conn = self.manager.clone();
        conn.expire(key, seconds as i64).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // 需要Redis服务器运行
    async fn test_redis_operations() {
        let client = RedisClient::new("redis://127.0.0.1:6379").await.unwrap();

        // 测试字符串操作
        client.set("test_key", "test_value", 60).await.unwrap();
        let value: Option<String> = client.get("test_key").await.unwrap();
        assert_eq!(value, Some("test_value".to_string()));

        // 测试删除
        client.del("test_key").await.unwrap();
        let value: Option<String> = client.get("test_key").await.unwrap();
        assert_eq!(value, None);
    }
}
