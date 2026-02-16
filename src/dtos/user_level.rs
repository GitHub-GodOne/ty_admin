use serde::Serialize;

/// 用户等级响应
#[derive(Debug, Serialize)]
pub struct UserLevelResponse {
    /// ID
    pub id: i32,

    /// 用户ID
    pub uid: i32,

    /// 等级ID
    #[serde(rename = "levelId")]
    pub level_id: i32,

    /// 等级
    pub grade: i32,

    /// 状态
    pub status: i32,

    /// 备注
    pub mark: String,

    /// 提醒
    pub remind: i32,

    /// 是否删除
    #[serde(rename = "isDel")]
    pub is_del: bool,

    /// 折扣
    pub discount: i32,

    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,

    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,

    /// 过期时间
    #[serde(rename = "expiredTime")]
    pub expired_time: Option<String>,
}
