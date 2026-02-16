use serde::{Deserialize, Serialize};
use crate::dtos::system_menu::MenuCheckVo;

/// 角色搜索请求
///
/// Java: SystemRoleSearchRequest
#[derive(Debug, Deserialize)]
pub struct SystemRoleSearchRequest {
    /// 角色名称
    #[serde(rename = "roleName", default)]
    pub role_name: Option<String>,

    /// 状态
    #[serde(default)]
    pub status: Option<bool>,
}

/// 角色新增/修改请求
///
/// Java: SystemRoleRequest
#[derive(Debug, Deserialize)]
pub struct SystemRoleRequest {
    /// 角色id（添加时不填，修改时必填）
    #[serde(default)]
    pub id: Option<i32>,

    /// 角色名称
    #[serde(rename = "roleName")]
    pub role_name: String,

    /// 权限字符串（英文逗号拼接的菜单id）
    pub rules: String,

    /// 状态：false=关闭，true=正常
    pub status: bool,
}

/// 角色列表响应
///
/// Java: SystemRole (list返回，只选了部分字段)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemRoleListResponse {
    pub id: i32,
    pub role_name: String,
    pub status: bool,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

/// 角色详情响应
///
/// Java: RoleInfoResponse
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleInfoResponse {
    pub id: i32,
    pub role_name: String,
    pub status: bool,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub menu_list: Vec<MenuCheckVo>,
}

/// 修改状态请求
#[derive(Debug, Deserialize)]
pub struct UpdateStatusQuery {
    pub id: i32,
    pub status: bool,
}
