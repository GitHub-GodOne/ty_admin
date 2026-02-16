use serde::{Deserialize, Serialize};

/// 分页请求参数
#[derive(Debug, Deserialize, Clone)]
pub struct PageRequest {
    /// 页码（从 1 开始）
    #[serde(default = "default_page")]
    pub page: u64,

    /// 每页数量
    #[serde(default = "default_limit")]
    pub limit: u64,
}

fn default_page() -> u64 {
    1
}

fn default_limit() -> u64 {
    10
}

impl Default for PageRequest {
    fn default() -> Self {
        Self { page: 1, limit: 10 }
    }
}

/// 分页响应
#[derive(Debug, Serialize)]
pub struct PageResponse<T> {
    /// 数据列表
    pub list: Vec<T>,

    /// 总记录数
    pub total: u64,

    /// 当前页码
    #[serde(rename = "pageNumber")]
    pub page_number: u64,

    /// 每页数量
    #[serde(rename = "pageSize")]
    pub page_size: u64,

    /// 总页数
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
}

impl<T> PageResponse<T> {
    pub fn new(list: Vec<T>, total: u64, page: u64, limit: u64) -> Self {
        let total_pages = if limit > 0 {
            (total + limit - 1) / limit
        } else {
            0
        };
        Self {
            list,
            total,
            page_number: page,
            page_size: limit,
            total_pages,
        }
    }
}
