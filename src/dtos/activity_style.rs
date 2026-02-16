/// 活动样式 DTO
///
/// Java参考:
/// - ActivityStyleSearchRequest (列表搜索)
/// - ActivityStyleRequest (新增/修改)
/// - ActivityStyleUpdateStatusRequest (更新状态)
/// - ActivityStyleResponse (列表响应)
use serde::{Deserialize, Deserializer, Serialize};

/// 空字符串反序列化为 None
fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => s.trim().parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
}

/// 将 "0"/"1"/""/"true"/"false" 反序列化为 Option<bool>
/// 前端query string中布尔值通常以 0/1 形式传递
fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => match s.trim() {
            "0" | "false" => Ok(Some(false)),
            "1" | "true" => Ok(Some(true)),
            other => Err(serde::de::Error::custom(format!("invalid bool value: {}", other))),
        },
    }
}

// ==================== 搜索请求 ====================

/// 活动样式列表搜索请求
#[derive(Debug, Deserialize)]
pub struct ActivityStyleSearchRequest {
    pub name: Option<String>,
    /// 类型: false(0)=边框, true(1)=背景
    #[serde(rename = "type", default, deserialize_with = "deserialize_optional_bool")]
    pub style_type: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub status: Option<bool>,
    /// 运行状态: -1=已结束, 0=未开始, 1=进行中
    #[serde(rename = "runningStatus", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub running_status: Option<i32>,
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub method: Option<i32>,
}

// ==================== 新增/修改请求 ====================

/// 活动样式新增/修改请求
#[derive(Debug, Deserialize)]
pub struct ActivityStyleRequest {
    pub id: Option<i32>,
    pub name: String,
    #[serde(rename = "type")]
    pub style_type: bool,
    pub starttime: String,
    pub endtime: String,
    pub style: String,
    pub status: bool,
    pub method: Option<i32>,
    pub products: Option<String>,
}

// ==================== 更新状态请求 ====================

/// 活动样式更新状态请求
#[derive(Debug, Deserialize)]
pub struct ActivityStyleUpdateStatusRequest {
    pub id: i32,
    pub status: bool,
}

// ==================== 响应 ====================

/// 活动样式列表响应
#[derive(Debug, Serialize, Clone)]
pub struct ActivityStyleResponse {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub style_type: bool,
    pub starttime: String,
    pub endtime: String,
    pub style: String,
    /// 运行状态: -1=已结束, 0=未开始, 1=进行中
    #[serde(rename = "runningStatus")]
    pub running_status: i32,
    pub status: bool,
    pub method: Option<i32>,
    pub products: Option<String>,
    pub createtime: String,
    pub updatetime: String,
}
