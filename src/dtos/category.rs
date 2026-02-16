use serde::{Deserialize, Deserializer, Serialize};

/// 反序列化空字符串为 None
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

/// 分类搜索请求
/// Java: CategorySearchRequest
#[derive(Debug, Deserialize)]
pub struct CategorySearchRequest {
    /// 分类名称（模糊搜索）
    pub name: Option<String>,

    /// 父级ID
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub pid: Option<i32>,

    /// 分类类型
    #[serde(rename = "type", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub category_type: Option<i16>,

    /// 状态
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i16>,
}

/// 分类新增/编辑请求
/// Java: CategoryRequest
#[derive(Debug, Deserialize)]
pub struct CategoryRequest {
    /// 分类ID（编辑时必填）
    pub id: Option<i32>,

    /// 父级ID
    pub pid: Option<i32>,

    /// 分类名称
    pub name: Option<String>,

    /// 分类类型
    #[serde(rename = "type")]
    pub category_type: Option<i16>,

    /// 链接地址
    pub url: Option<String>,

    /// 扩展字段（JSON字符串）
    pub extra: Option<String>,

    /// 排序
    pub sort: Option<i32>,

    /// 状态：1=正常，0=关闭
    pub status: Option<i16>,
}

/// 分类响应
/// Java: CategoryResponse / Category
#[derive(Debug, Serialize, Clone)]
pub struct CategoryResponse {
    pub id: i32,
    pub pid: i32,
    pub path: String,
    pub name: String,
    #[serde(rename = "type")]
    pub category_type: Option<i16>,
    pub url: Option<String>,
    pub extra: Option<String>,
    pub status: i16,
    pub sort: i32,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
}

/// 分类树形结构
/// Java: CategoryTreeVo
#[derive(Debug, Serialize, Clone)]
pub struct CategoryTreeVo {
    /// 分类ID
    pub id: i32,

    /// 父级ID
    pub pid: i32,

    /// 分类名称
    pub name: String,

    /// 分类类型
    #[serde(rename = "type")]
    pub category_type: Option<i16>,

    /// 链接地址
    pub url: Option<String>,

    /// 扩展字段
    pub extra: Option<String>,

    /// 状态
    pub status: i16,

    /// 排序
    pub sort: i32,

    /// 路径
    pub path: String,

    /// 子分类列表
    #[serde(rename = "child")]
    pub child: Vec<CategoryTreeVo>,
}

/// 更新状态请求
#[derive(Debug, Deserialize)]
pub struct UpdateStatusQuery {
    pub id: i32,
}
