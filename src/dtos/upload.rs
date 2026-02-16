use serde::{Deserialize, Serialize};

/// 文件上传结果
/// Java: FileResultVo
#[derive(Debug, Serialize)]
pub struct FileResultVo {
    /// 文件名
    #[serde(rename = "fileName")]
    pub file_name: String,

    /// 扩展名
    #[serde(rename = "extName")]
    pub ext_name: String,

    /// 文件大小（字节）
    #[serde(rename = "fileSize")]
    pub file_size: u64,

    /// 可供访问的url
    pub url: String,

    /// 类型
    #[serde(rename = "type")]
    pub file_type: String,
}

/// 上传请求参数（Query部分）
#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    /// 模块 用户user,商品product,微信wechat,news文章
    pub model: String,

    /// 分类ID 0编辑器,1商品图片,2拼团图片,3砍价图片,4秒杀图片,5文章图片,6组合数据图,7前台用户,8微信系列
    pub pid: i32,
}
