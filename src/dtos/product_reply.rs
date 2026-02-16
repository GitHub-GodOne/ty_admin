use serde::{Deserialize, Deserializer, Serialize};

/// 将空字符串反序列化为None
fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

/// 将空字符串反序列化为None（bool类型）
fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        Some(ref v) if v == "true" => Ok(Some(true)),
        Some(ref v) if v == "false" => Ok(Some(false)),
        Some(ref v) if v == "1" => Ok(Some(true)),
        Some(ref v) if v == "0" => Ok(Some(false)),
        _ => Ok(None),
    }
}

/// 商品评论搜索请求
/// Java: StoreProductReplySearchRequest
#[derive(Debug, Deserialize)]
pub struct StoreProductReplySearchRequest {
    /// 商品搜索（商品名称）
    #[serde(rename = "productSearch", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub product_search: Option<String>,

    /// 是否已回复: true=已回复, false=未回复
    #[serde(rename = "isReply", default, deserialize_with = "deserialize_optional_bool")]
    pub is_reply: Option<bool>,

    /// 用户昵称
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub nickname: Option<String>,

    /// 时间范围（格式: "2024-01-01,2024-01-31"）
    #[serde(rename = "dateLimit", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,
}

/// 虚拟评论新增请求
/// Java: StoreProductReplyAddRequest
#[derive(Debug, Deserialize)]
pub struct StoreProductReplyAddRequest {
    /// 用户ID（虚拟评论可以为0）
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,

    /// 商品ID
    #[serde(rename = "productId")]
    pub product_id: i32,

    /// 唯一标识（虚拟评论自动生成）
    pub unique: Option<String>,

    /// 商品评分 1-5
    #[serde(rename = "productScore")]
    pub product_score: i16,

    /// 服务评分 1-5
    #[serde(rename = "serviceScore")]
    pub service_score: i16,

    /// 评论内容（最大512字符）
    pub comment: String,

    /// 评论图片（JSON数组字符串）
    pub pics: Option<String>,

    /// 用户头像
    pub avatar: Option<String>,

    /// 用户昵称
    pub nickname: Option<String>,

    /// SKU信息
    pub sku: Option<String>,
}

/// 管理员回复评论请求
/// Java: StoreProductReplyCommentRequest
#[derive(Debug, Deserialize)]
pub struct StoreProductReplyCommentRequest {
    /// 评论ID
    pub ids: i32,

    /// 商家回复内容
    #[serde(rename = "merchantReplyContent")]
    pub merchant_reply_content: String,
}

/// 商品评论响应
/// Java: StoreProductReplyResponse
#[derive(Debug, Serialize)]
pub struct StoreProductReplyResponse {
    pub id: i32,
    pub uid: i32,
    pub oid: i32,
    #[serde(rename = "unique")]
    pub unique: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "replyType")]
    pub reply_type: String,
    #[serde(rename = "productScore")]
    pub product_score: i16,
    #[serde(rename = "serviceScore")]
    pub service_score: i16,
    pub comment: String,
    /// 图片列表（从字符串转为数组）
    pub pics: Vec<String>,
    #[serde(rename = "merchantReplyContent")]
    pub merchant_reply_content: Option<String>,
    #[serde(rename = "merchantReplyTime")]
    pub merchant_reply_time: Option<i32>,
    #[serde(rename = "isDel")]
    pub is_del: bool,
    #[serde(rename = "isReply")]
    pub is_reply: bool,
    pub nickname: String,
    pub avatar: String,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
    pub sku: String,
    /// 关联的商品信息
    #[serde(rename = "storeProduct")]
    pub store_product: Option<StoreProductSimpleVo>,
}

/// 商品简要信息（用于评论列表中关联展示）
/// Java: StoreProduct（评论列表中附带的商品信息）
#[derive(Debug, Serialize)]
pub struct StoreProductSimpleVo {
    pub id: i32,
    pub image: String,
    #[serde(rename = "storeName")]
    pub store_name: String,
}
