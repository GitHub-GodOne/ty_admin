/// 秒杀管理 DTO
///
/// 对应Java:
/// - StoreSeckillSearchRequest
/// - StoreSeckillAddRequest
/// - StoreSeckillResponse
/// - StoreSeckillMangerSearchRequest
/// - StoreSeckillMangerRequest
/// - StoreSeckillManagerResponse
use serde::{Deserialize, Deserializer, Serialize};
use rust_decimal::Decimal;

/// 空字符串反序列化为 None
fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.trim().is_empty() => Ok(None),
        Some(s) => s.trim().parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
}

// ==================== 秒杀商品 ====================

/// 秒杀商品搜索请求
///
/// Java: StoreSeckillSearchRequest
#[derive(Debug, Deserialize)]
pub struct StoreSeckillSearchRequest {
    /// 搜索关键字
    pub keywords: Option<String>,

    /// 时间段id
    #[serde(rename = "timeId", default, deserialize_with = "deserialize_empty_string_as_none")]
    pub time_id: Option<i32>,

    /// 状态: 0=关闭, 1=开启
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub status: Option<i16>,
}

/// 秒杀商品新增/修改请求
///
/// Java: StoreSeckillAddRequest
#[derive(Debug, Deserialize)]
pub struct StoreSeckillAddRequest {
    /// 秒杀商品id（修改时必填）
    pub id: Option<i32>,

    /// 商品id
    #[serde(rename = "productId")]
    pub product_id: i32,

    /// 推荐图
    pub image: String,

    /// 轮播图（逗号分隔）
    pub images: String,

    /// 活动标题
    pub title: String,

    /// 单位名
    #[serde(rename = "unitName")]
    pub unit_name: String,

    /// 活动开始时间
    #[serde(rename = "startTime")]
    pub start_time: String,

    /// 活动结束时间
    #[serde(rename = "stopTime")]
    pub stop_time: String,

    /// 状态
    pub status: i16,

    /// 限购总数
    pub num: i32,

    /// 秒杀时间段id
    #[serde(rename = "timeId")]
    pub time_id: i32,

    /// 运费模板id
    #[serde(rename = "tempId")]
    pub temp_id: i32,

    /// 商品属性
    pub attr: Option<serde_json::Value>,

    /// 商品属性值
    #[serde(rename = "attrValue")]
    pub attr_value: Option<serde_json::Value>,

    /// 商品详情
    pub content: Option<String>,

    /// 排序
    pub sort: Option<i32>,

    /// 规格类型: 0=单规格, 1=多规格
    #[serde(rename = "specType")]
    pub spec_type: Option<i16>,
}

/// 秒杀商品列表响应
///
/// Java: StoreSeckillResponse
#[derive(Debug, Serialize, Clone)]
pub struct StoreSeckillResponse {
    pub id: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
    pub image: String,
    pub images: String,
    pub title: String,
    pub info: String,
    pub price: Decimal,
    pub cost: Decimal,
    #[serde(rename = "otPrice")]
    pub ot_price: Decimal,
    #[serde(rename = "giveIntegral")]
    pub give_integral: Decimal,
    pub sort: i32,
    pub stock: i32,
    pub sales: i32,
    #[serde(rename = "unitName")]
    pub unit_name: String,
    pub postage: Decimal,
    pub description: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "stopTime")]
    pub stop_time: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    pub status: i16,
    #[serde(rename = "statusName")]
    pub status_name: String,
    #[serde(rename = "isPostage")]
    pub is_postage: bool,
    pub num: i32,
    #[serde(rename = "isShow")]
    pub is_show: bool,
    #[serde(rename = "timeId")]
    pub time_id: Option<i32>,
    #[serde(rename = "tempId")]
    pub temp_id: i32,
    pub weight: Decimal,
    pub volume: Decimal,
    pub quota: i32,
    #[serde(rename = "quotaShow")]
    pub quota_show: i32,
    #[serde(rename = "specType")]
    pub spec_type: i16,
    /// 秒杀状态: -1=已结束, 0=已关闭, 1=未开始, 2=进行中
    #[serde(rename = "killStatus")]
    pub kill_status: i32,
}

/// 秒杀商品详情响应（管理端）
#[derive(Debug, Serialize, Clone)]
pub struct StoreSeckillDetailResponse {
    #[serde(rename = "storeSeckill")]
    pub store_seckill: StoreSeckillResponse,
    /// 商品属性
    #[serde(rename = "productAttr")]
    pub product_attr: serde_json::Value,
    /// 商品属性值
    #[serde(rename = "productValue")]
    pub product_value: serde_json::Value,
}

/// 更新状态请求
#[derive(Debug, Deserialize)]
pub struct SeckillStatusRequest {
    pub id: i32,
    pub status: i16,
}

// ==================== 秒杀时间段 ====================

/// 秒杀时间段搜索请求
///
/// Java: StoreSeckillMangerSearchRequest
#[derive(Debug, Deserialize)]
pub struct StoreSeckillMangerSearchRequest {
    /// 时间段名称
    pub name: Option<String>,

    /// 状态
    pub status: Option<String>,
}

/// 秒杀时间段新增/修改请求
///
/// Java: StoreSeckillMangerRequest
#[derive(Debug, Deserialize)]
pub struct StoreSeckillMangerRequest {
    /// id（修改时必填）
    pub id: Option<i32>,

    /// 时间段名称
    pub name: String,

    /// 时间范围 "HH:00,HH:00"
    pub time: String,

    /// 图片
    pub img: Option<String>,

    /// 轮播图
    #[serde(rename = "silderImgs")]
    pub silder_imgs: Option<String>,

    /// 排序
    pub sort: Option<i32>,

    /// 状态: 0=关闭, 1=开启
    pub status: Option<String>,
}

/// 秒杀时间段响应
///
/// Java: StoreSeckillManagerResponse
#[derive(Debug, Serialize, Clone)]
pub struct StoreSeckillMangerResponse {
    pub id: i32,
    pub name: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<i32>,
    #[serde(rename = "endTime")]
    pub end_time: Option<i32>,
    pub img: Option<String>,
    #[serde(rename = "silderImgs")]
    pub silder_imgs: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<String>,
    #[serde(rename = "isDel")]
    pub is_del: i32,
    /// 时间范围 "HH:00,HH:00"
    pub time: Option<String>,
    /// 状态名称
    #[serde(rename = "statusName")]
    pub status_name: String,
    /// 秒杀状态: -1=已结束, 0=已关闭, 1=未开始, 2=进行中
    #[serde(rename = "killStatus")]
    pub kill_status: i32,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<String>,
}
