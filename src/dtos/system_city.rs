/// 城市管理 DTO
///
/// 对应Java:
/// - SystemCitySearchRequest
/// - SystemCityRequest
/// - SystemCityTreeVo
use serde::{Deserialize, Serialize};

/// 城市列表搜索请求
///
/// Java: SystemCitySearchRequest
/// parentId 必填，最小值0
#[derive(Debug, Deserialize)]
pub struct SystemCitySearchRequest {
    /// 父级id
    #[serde(rename = "parentId")]
    pub parent_id: i32,
}

/// 城市修改请求
///
/// Java: SystemCityRequest
/// parentId 必填，最小值0
/// name 必填，最大100字符
#[derive(Debug, Deserialize)]
pub struct SystemCityRequest {
    /// 父级id
    #[serde(rename = "parentId")]
    pub parent_id: i32,

    /// 城市名称
    pub name: String,
}

/// 城市修改状态请求
#[derive(Debug, Deserialize)]
pub struct SystemCityUpdateStatusRequest {
    /// 城市id
    pub id: i32,

    /// 显示状态
    pub status: bool,
}

/// 城市响应（用于列表和详情）
///
/// Java: SystemCity model 直接返回
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemCityResponse {
    pub id: i32,
    #[serde(rename = "cityId")]
    pub city_id: i32,
    pub level: i32,
    #[serde(rename = "parentId")]
    pub parent_id: i32,
    #[serde(rename = "areaCode")]
    pub area_code: String,
    pub name: String,
    #[serde(rename = "mergerName")]
    pub merger_name: String,
    pub lng: String,
    pub lat: String,
    #[serde(rename = "isShow")]
    pub is_show: bool,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

/// 城市树形结构响应
///
/// Java: SystemCityTreeVo
/// child 为空时不序列化（@JsonInclude NON_EMPTY）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemCityTreeVo {
    pub id: i32,
    #[serde(rename = "cityId")]
    pub city_id: i32,
    pub level: i32,
    #[serde(rename = "parentId")]
    pub parent_id: i32,
    #[serde(rename = "areaCode")]
    pub area_code: String,
    pub name: String,
    #[serde(rename = "mergerName")]
    pub merger_name: String,
    pub lng: String,
    pub lat: String,
    #[serde(rename = "isShow")]
    pub is_show: bool,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    /// 子节点列表，为空时不序列化
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub child: Vec<SystemCityTreeVo>,
}
