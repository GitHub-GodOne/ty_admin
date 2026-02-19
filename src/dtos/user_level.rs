use serde::{Deserialize, Serialize};

/// 系统用户等级响应（等级配置列表）
#[derive(Debug, Serialize)]
pub struct UserLevelResponse {
    /// ID
    pub id: i32,

    /// 等级名称
    pub name: String,

    /// 所需经验值
    pub experience: i32,

    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: bool,

    /// 等级
    pub grade: i32,

    /// 折扣
    pub discount: i32,

    /// 图标
    pub icon: String,

    /// 是否删除
    #[serde(rename = "isDel")]
    pub is_del: bool,

    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,

    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

/// 用户等级保存/更新请求
#[derive(Debug, Deserialize)]
pub struct UserLevelSaveRequest {
    /// 等级名称
    pub name: String,

    /// 等级
    pub grade: i32,

    /// 折扣
    pub discount: Option<i32>,

    /// 经验值
    pub experience: i32,

    /// 图标
    pub icon: Option<String>,

    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: Option<bool>,
}

/// 用户等级状态更新请求
#[derive(Debug, Deserialize)]
pub struct UserLevelUseRequest {
    /// ID
    pub id: i32,

    /// 是否显示
    #[serde(rename = "isShow")]
    pub is_show: bool,
}

/// ID 请求
#[derive(Debug, Deserialize)]
pub struct LevelIdRequest {
    pub id: Option<i32>,
}
