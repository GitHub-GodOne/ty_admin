use serde::{Deserialize, Serialize};

/// 附件列表查询参数（包含分页）
/// Java: SystemAttachmentController.getList() 的参数
#[derive(Debug, Deserialize)]
pub struct SystemAttachmentListQuery {
    /// 分类ID
    #[serde(default)]
    pub pid: i32,

    /// 附件类型（逗号分隔，如 "png,jpeg,jpg,audio/mpeg,text/plain,video/mp4,gif"）
    #[serde(rename = "attType")]
    pub att_type: Option<String>,

    /// 页码
    pub page: Option<i32>,

    /// 每页数量
    pub limit: Option<i32>,
}

/// 附件新增/编辑请求
/// Java: SystemAttachmentRequest
#[derive(Debug, Deserialize)]
pub struct SystemAttachmentRequest {
    /// 附件ID（编辑时必填）
    #[serde(rename = "attId")]
    pub att_id: Option<i32>,

    /// 附件名称
    pub name: Option<String>,

    /// 附件路径
    #[serde(rename = "attDir")]
    pub att_dir: Option<String>,

    /// 压缩图片路径
    #[serde(rename = "sattDir")]
    pub satt_dir: Option<String>,

    /// 附件大小
    #[serde(rename = "attSize")]
    pub att_size: Option<String>,

    /// 附件类型
    #[serde(rename = "attType")]
    pub att_type: Option<String>,

    /// 图片上传类型 1本地 2七牛云 3OSS 4COS
    #[serde(rename = "imageType")]
    pub image_type: Option<i16>,
}

/// 附件移动请求
/// Java: SystemAttachmentMoveRequest
#[derive(Debug, Deserialize)]
pub struct SystemAttachmentMoveRequest {
    /// 目标分类ID
    pub pid: i32,

    /// 附件ID列表（逗号分隔）
    #[serde(rename = "attrId")]
    pub attr_id: String,
}

/// 附件响应
/// Java: SystemAttachment entity
#[derive(Debug, Serialize)]
pub struct SystemAttachmentResponse {
    #[serde(rename = "attId")]
    pub att_id: i32,
    pub name: String,
    #[serde(rename = "attDir")]
    pub att_dir: String,
    #[serde(rename = "sattDir")]
    pub satt_dir: Option<String>,
    #[serde(rename = "attSize")]
    pub att_size: String,
    #[serde(rename = "attType")]
    pub att_type: String,
    pub pid: i32,
    #[serde(rename = "imageType")]
    pub image_type: i16,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}
