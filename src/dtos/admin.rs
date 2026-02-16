use serde::{Deserialize, Serialize};

/// 管理员登录请求
#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    /// 账号
    pub account: String,

    /// 密码
    pub pwd: String,
}

/// 管理员登录响应
#[derive(Debug, Serialize)]
pub struct AdminLoginResponse {
    /// 管理员ID
    pub id: i32,

    /// 账号
    pub account: String,

    /// 真实姓名
    #[serde(rename = "realName")]
    pub real_name: String,

    /// Token
    pub token: String,

    /// 是否开启短信提醒
    #[serde(rename = "isSms")]
    pub is_sms: bool,
}

/// 管理员信息响应（对应Java SystemAdminResponse）
#[derive(Debug, Serialize)]
pub struct AdminInfoResponse {
    /// 管理员ID
    pub id: i32,

    /// 账号
    pub account: String,

    /// 真实姓名
    #[serde(rename = "realName")]
    pub real_name: String,

    /// 角色ID列表（逗号分隔）
    pub roles: String,

    /// 角色名称列表
    #[serde(rename = "roleNames")]
    pub role_names: Option<String>,

    /// 最后登录IP
    #[serde(rename = "lastIp")]
    pub last_ip: Option<String>,

    /// 最后登录时间
    #[serde(rename = "lastTime")]
    pub last_time: Option<String>,

    /// 添加时间
    #[serde(rename = "addTime")]
    pub add_time: Option<String>,

    /// 登录次数
    #[serde(rename = "loginCount")]
    pub login_count: i32,

    /// 管理员级别
    pub level: i32,

    /// 状态
    pub status: bool,

    /// 手机号
    pub phone: Option<String>,

    /// 是否开启短信提醒
    #[serde(rename = "isSms")]
    pub is_sms: bool,

    /// 权限标识数组
    #[serde(rename = "permissionsList")]
    pub permissions_list: Vec<String>,

    /// Token（getInfoByToken时不返回）
    #[serde(rename = "Token")]
    pub token: Option<String>,
}

/// 菜单响应
#[derive(Debug, Serialize, Clone)]
pub struct MenuResponse {
    /// 菜单ID
    pub id: i32,

    /// 菜单名称
    pub name: String,

    /// 父级ID
    pub pid: i32,

    /// 路由路径
    pub path: String,

    /// 图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 菜单类型：M=目录，C=菜单，A=按钮
    #[serde(rename = "menuType")]
    pub menu_type: String,

    /// 组件路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,

    /// 权限标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perms: Option<String>,

    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: bool,

    /// 排序
    pub sort: i32,

    /// 子菜单列表
    #[serde(rename = "childList")]
    pub child_list: Vec<MenuResponse>,
}

/// 管理员列表查询请求
#[derive(Debug, Deserialize)]
pub struct AdminListRequest {
    /// 关键字搜索（账号或姓名）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<String>,

    /// 状态：0=禁用，1=启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 管理员新增/修改请求
#[derive(Debug, Deserialize)]
pub struct AdminSaveRequest {
    /// 管理员ID（修改时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /// 账号
    pub account: String,

    /// 密码（新增时必填，修改时可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,

    /// 确认密码
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conf_pwd")]
    pub conf_pwd: Option<String>,

    /// 真实姓名
    #[serde(rename = "realName")]
    pub real_name: String,

    /// 角色ID列表（逗号分隔）
    pub roles: String,

    /// 状态：0=禁用，1=启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,

    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// 修改密码请求
#[derive(Debug, Deserialize)]
pub struct UpdatePasswordRequest {
    /// 旧密码
    #[serde(rename = "oldPassword")]
    pub old_password: String,

    /// 新密码
    #[serde(rename = "newPassword")]
    pub new_password: String,

    /// 确认新密码
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

/// 账号检测请求
#[derive(Debug, Deserialize)]
pub struct AccountDetectionRequest {
    /// 账号
    pub account: String,

    /// 管理员ID（修改时传入，用于排除自己）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}
