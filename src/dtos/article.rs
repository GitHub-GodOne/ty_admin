/// 文章管理 DTO
///
/// 对应Java:
/// - ArticleSearchRequest
/// - ArticleRequest
/// - ArticleVo
use serde::{Deserialize, Serialize};

/// 文章搜索请求
///
/// Java: ArticleSearchRequest
#[derive(Debug, Deserialize)]
pub struct ArticleSearchRequest {
    /// 分类id
    pub cid: Option<String>,

    /// 搜索关键字
    pub keywords: Option<String>,
}

/// 文章新增/修改请求
///
/// Java: ArticleRequest
#[derive(Debug, Deserialize)]
pub struct ArticleRequest {
    /// 分类id（必填）
    pub cid: String,

    /// 文章标题（必填，最多200字符）
    pub title: String,

    /// 文章作者（必填，最多50字符）
    pub author: String,

    /// 文章图片（必填）
    #[serde(rename = "imageInput")]
    pub image_input: String,

    /// 文章简介（必填，最多200字符）
    pub synopsis: String,

    /// 文章分享标题（必填，最多200字符）
    #[serde(rename = "shareTitle")]
    pub share_title: String,

    /// 文章分享简介（必填，最多200字符）
    #[serde(rename = "shareSynopsis")]
    pub share_synopsis: String,

    /// 是否热门(小程序)（必填）
    #[serde(rename = "isHot")]
    pub is_hot: bool,

    /// 是否轮播图(小程序)（必填）
    #[serde(rename = "isBanner")]
    pub is_banner: bool,

    /// 文章内容（必填）
    pub content: String,
}

/// 文章列表响应（管理端）
///
/// Java: ArticleVo
#[derive(Debug, Serialize, Clone)]
pub struct ArticleVo {
    pub id: i32,
    pub cid: String,
    pub title: String,
    pub author: Option<String>,
    #[serde(rename = "imageInput")]
    pub image_input: String,
    pub synopsis: Option<String>,
    pub visit: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}

/// 文章详情响应（管理端 info 接口直接返回 Article 全字段）
///
/// Java: Article model 直接返回
#[derive(Debug, Serialize, Clone)]
pub struct ArticleDetailResponse {
    pub id: i32,
    pub cid: String,
    pub title: String,
    pub author: Option<String>,
    #[serde(rename = "imageInput")]
    pub image_input: String,
    pub synopsis: Option<String>,
    #[serde(rename = "shareTitle")]
    pub share_title: Option<String>,
    #[serde(rename = "shareSynopsis")]
    pub share_synopsis: Option<String>,
    pub visit: Option<String>,
    pub sort: i32,
    pub url: Option<String>,
    #[serde(rename = "mediaId")]
    pub media_id: Option<String>,
    pub status: bool,
    pub hide: bool,
    #[serde(rename = "adminId")]
    pub admin_id: i32,
    #[serde(rename = "merId")]
    pub mer_id: Option<i32>,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "isHot")]
    pub is_hot: bool,
    #[serde(rename = "isBanner")]
    pub is_banner: bool,
    pub content: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}
