use serde::{Deserialize, Deserializer, Serialize};

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

/// 提货点搜索请求
///
/// Java: SystemStoreSearchRequest
#[derive(Debug, Deserialize)]
pub struct SystemStoreSearchRequest {
    /// 搜索关键字
    pub keywords: Option<String>,

    /// 状态: 0=隐藏, 1=显示, 2=回收站
    #[serde(default = "default_status")]
    pub status: i32,
}

fn default_status() -> i32 {
    1
}

/// 提货点新增/修改请求
///
/// Java: SystemStoreRequest
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStoreRequest {
    /// 门店名称
    pub name: String,

    /// 简介
    pub introduction: Option<String>,

    /// 手机号码
    pub phone: Option<String>,

    /// 省市区
    pub address: String,

    /// 详细地址
    pub detailed_address: String,

    /// 每日营业开关时间
    pub day_time: Option<String>,

    /// 门店logo
    pub image: String,

    /// 纬度（前端传 "经度,纬度" 格式）
    pub latitude: Option<String>,

    /// 经度
    pub longitude: Option<String>,

    /// 核销有效日期
    pub valid_time: Option<String>,
}

/// 提货点响应
///
/// Java: SystemStore (直接返回实体)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStoreResponse {
    pub id: i32,
    pub name: String,
    pub introduction: String,
    pub phone: String,
    pub address: String,
    pub detailed_address: String,
    pub image: String,
    pub latitude: String,
    pub longitude: String,
    pub valid_time: String,
    pub day_time: String,
    pub is_show: bool,
    pub is_del: bool,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

/// 提货点数量统计响应
#[derive(Debug, Serialize)]
pub struct SystemStoreCountResponse {
    pub show: i64,
    pub hide: i64,
    pub recycle: i64,
}

/// 修改状态请求
#[derive(Debug, Deserialize)]
pub struct StoreUpdateStatusQuery {
    pub id: i32,
    pub status: bool,
}

/// 核销员搜索请求
///
/// Java: SystemStoreStaffController.getList params
#[derive(Debug, Deserialize)]
pub struct StoreStaffSearchRequest {
    /// 门店id
    #[serde(rename = "storeId", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub store_id: Option<i32>,
}

/// 核销员新增/修改请求
///
/// Java: SystemStoreStaffRequest
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStoreStaffRequest {
    /// 管理员id
    pub uid: i32,

    /// 管理员头像
    pub avatar: Option<String>,

    /// 提货点id
    pub store_id: i32,

    /// 核销员名称
    pub staff_name: String,

    /// 手机号码
    pub phone: Option<String>,

    /// 核销开关
    #[serde(default)]
    pub verify_status: bool,

    /// 状态
    #[serde(default)]
    pub status: bool,
}

/// 核销员修改状态请求
#[derive(Debug, Deserialize)]
pub struct StaffUpdateStatusQuery {
    pub id: i32,
    pub status: i32,
}

/// 核销员列表响应（含用户和门店信息）
///
/// Java: SystemStoreStaffResponse
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStoreStaffResponse {
    pub id: i32,
    pub uid: i32,
    pub avatar: String,
    pub store_id: i32,
    pub staff_name: Option<String>,
    pub phone: Option<String>,
    pub verify_status: i32,
    pub status: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    /// 用户信息
    pub user: Option<StaffUserInfo>,
    /// 门店信息
    pub system_store: Option<StaffStoreInfo>,
}

/// 核销员关联的用户信息
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffUserInfo {
    pub uid: i32,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub phone: Option<String>,
}

/// 核销员关联的门店信息
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StaffStoreInfo {
    pub id: i32,
    pub name: String,
    pub phone: String,
    pub address: String,
}

/// 核销员详情响应（直接返回实体字段）
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStoreStaffInfoResponse {
    pub id: i32,
    pub uid: i32,
    pub avatar: String,
    pub store_id: i32,
    pub staff_name: Option<String>,
    pub phone: Option<String>,
    pub verify_status: i32,
    pub status: Option<i32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
