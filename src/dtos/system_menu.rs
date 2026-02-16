use serde::{Deserialize, Serialize};

/// 菜单搜索请求
///
/// Java: SystemMenuSearchRequest
#[derive(Debug, Deserialize)]
pub struct SystemMenuSearchRequest {
    /// 菜单名称
    #[serde(default)]
    pub name: Option<String>,

    /// 菜单类型: M=目录, C=菜单, A=按钮
    #[serde(rename = "menuType", default)]
    pub menu_type: Option<String>,
}

/// 菜单新增/修改请求
///
/// Java: SystemMenuRequest
#[derive(Debug, Deserialize)]
pub struct SystemMenuRequest {
    /// 菜单ID（修改时必填）
    #[serde(default)]
    pub id: Option<i32>,

    /// 父级ID
    pub pid: i32,

    /// 菜单名称
    pub name: String,

    /// 图标
    #[serde(default)]
    pub icon: Option<String>,

    /// 权限标识
    #[serde(default)]
    pub perms: Option<String>,

    /// 组件路径
    #[serde(default)]
    pub component: Option<String>,

    /// 菜单类型: M=目录, C=菜单, A=按钮
    #[serde(rename = "menuType")]
    pub menu_type: String,

    /// 排序
    #[serde(default)]
    pub sort: i32,

    /// 是否显示
    #[serde(rename = "isShow", default)]
    pub is_show: Option<bool>,
}

/// 菜单响应（不含isDelte/createTime/updateTime）
///
/// Java: SystemMenu (getInfo返回时去掉部分字段)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMenuResponse {
    pub id: i32,
    pub pid: i32,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub perms: Option<String>,
    pub component: Option<String>,
    pub menu_type: Option<String>,
    pub sort: i32,
    pub is_show: bool,
}

/// 菜单列表响应（含全部字段）
///
/// Java: SystemMenu (list返回)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMenuListResponse {
    pub id: i32,
    pub pid: i32,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub perms: Option<String>,
    pub component: Option<String>,
    pub menu_type: Option<String>,
    pub sort: i32,
    pub is_show: bool,
    pub is_delte: bool,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

/// 菜单树节点
///
/// Java: MenuCheckVo
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuCheckVo {
    pub id: i32,
    pub pid: i32,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub checked: bool,
    pub sort: i32,
    pub child_list: Vec<MenuCheckVo>,
}
