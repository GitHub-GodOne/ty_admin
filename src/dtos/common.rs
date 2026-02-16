use serde::{Deserialize, Serialize};

/// ID 查询参数
#[derive(Debug, Deserialize)]
pub struct IdQuery {
    pub id: i32,
}

/// IDs 查询参数（逗号分隔）
#[derive(Debug, Deserialize)]
pub struct IdsQuery {
    pub ids: String,
}

/// 删除操作参数
#[derive(Debug, Deserialize)]
pub struct DeleteQuery {
    pub id: i32,
    #[serde(rename = "type")]
    pub delete_type: Option<String>,
}

/// 分页参数
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PageParamRequest {
    /// 页码
    pub page: Option<i32>,

    /// 每页数量
    pub limit: Option<i32>,
}

impl Default for PageParamRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            limit: Some(10),
        }
    }
}

impl PageParamRequest {
    /// 获取页码（默认为1）
    pub fn get_page(&self) -> i32 {
        self.page.unwrap_or(1)
    }

    /// 获取每页数量（默认为10）
    pub fn get_limit(&self) -> i32 {
        self.limit.unwrap_or(10)
    }
}

/// 通用分页响应（兼容旧代码）
#[derive(Debug, Serialize)]
pub struct CommonPage<T> {
    pub list: Vec<T>,
    pub total: i64,
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

impl<T> CommonPage<T> {
    pub fn new(list: Vec<T>, total: i64, page: i32, limit: i32) -> Self {
        Self {
            list,
            total,
            page_number: page,
            page_size: limit,
        }
    }
}
