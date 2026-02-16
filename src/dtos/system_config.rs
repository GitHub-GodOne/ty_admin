use serde::{Deserialize, Serialize};

/// 表单项检查请求（对应Java SystemFormItemCheckRequest）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SystemFormItemCheckRequest {
    /// 字段名
    pub name: String,

    /// 字段标题
    #[serde(default)]
    pub title: String,

    /// 字段值
    pub value: Option<String>,
}

/// 表单检查请求（对应Java SystemFormCheckRequest）
///
/// 用于保存表单配置
#[derive(Debug, Deserialize)]
pub struct SystemFormCheckRequest {
    /// 表单ID
    #[serde(rename = "id")]
    pub id: i32,

    /// 表单字段列表
    pub fields: Vec<SystemFormItemCheckRequest>,
}

/// 保存配置请求（对应Java SaveConfigRequest）
///
/// 用于保存单个配置项
#[derive(Debug, Deserialize)]
pub struct SaveConfigRequest {
    /// 配置名称
    pub name: String,

    /// 配置值
    pub value: String,
}

/// 管理端站点Logo响应（对应Java AdminSiteLogoResponse）
#[derive(Debug, Serialize)]
pub struct AdminSiteLogoResponse {
    /// 方形Logo
    #[serde(rename = "siteLogoSquare")]
    pub site_logo_square: Option<String>,

    /// 左上角Logo
    #[serde(rename = "siteLogoLeftTop")]
    pub site_logo_left_top: Option<String>,
}

/// 系统配置响应（对应Java SystemConfig返回给前端的格式）
#[derive(Debug, Serialize)]
pub struct SystemConfigResponse {
    /// 配置ID
    pub id: i32,

    /// 配置名称
    pub name: String,

    /// 配置标题
    pub title: String,

    /// 表单ID
    #[serde(rename = "formId")]
    pub form_id: Option<i32>,

    /// 配置值
    pub value: Option<String>,

    /// 状态
    pub status: Option<i32>,

    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,

    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

impl From<crate::models::_entities::system_config::Model> for SystemConfigResponse {
    fn from(model: crate::models::_entities::system_config::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            title: model.title,
            form_id: model.form_id,
            value: model.value,
            status: model.status.map(|s| s as i32),
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// 表单ID查询参数
#[derive(Debug, Deserialize)]
pub struct FormIdQuery {
    /// 表单ID
    #[serde(rename = "formId")]
    pub form_id: i32,
}
