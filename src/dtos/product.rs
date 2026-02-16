use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 商品搜索请求
#[derive(Debug, Deserialize, Serialize)]
pub struct StoreProductSearchRequest {
    /// 类型（1：出售中，2：仓库中，3：已售罄，4：警戒库存，5：回收站）
    #[serde(rename = "type")]
    pub product_type: i32,

    /// 分类ID，多个逗号分隔
    #[serde(rename = "cateId")]
    pub cate_id: Option<String>,

    /// 关键字搜索
    pub keywords: Option<String>,

    /// 价格排序 asc/desc
    #[serde(rename = "priceOrder")]
    pub price_order: Option<String>,

    /// 销量排序 asc/desc
    #[serde(rename = "salesOrder")]
    pub sales_order: Option<String>,
}

/// 商品响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreProductResponse {
    /// 商品ID
    pub id: i32,

    /// 商品图片
    pub image: Option<String>,

    /// 轮播图
    #[serde(rename = "sliderImage")]
    pub slider_image: Option<String>,

    /// 商品名称
    #[serde(rename = "storeName")]
    pub store_name: String,

    /// 关键字
    pub keyword: Option<String>,

    /// 商品条码
    #[serde(rename = "barCode")]
    pub bar_code: Option<String>,

    /// 分类ID
    #[serde(rename = "cateId")]
    pub cate_id: Option<String>,

    /// 商品价格
    pub price: Decimal,

    /// 会员价格
    #[serde(rename = "vipPrice")]
    pub vip_price: Option<Decimal>,

    /// 市场价
    #[serde(rename = "otPrice")]
    pub ot_price: Option<Decimal>,

    /// 邮费
    pub postage: Decimal,

    /// 单位名
    #[serde(rename = "unitName")]
    pub unit_name: Option<String>,

    /// 排序
    pub sort: i32,

    /// 销量
    pub sales: i32,

    /// 库存
    pub stock: i32,

    /// 状态（0：未上架，1：上架）
    #[serde(rename = "isShow")]
    pub is_show: bool,

    /// 是否热卖
    #[serde(rename = "isHot")]
    pub is_hot: bool,

    /// 是否优惠
    #[serde(rename = "isBenefit")]
    pub is_benefit: bool,

    /// 是否精品
    #[serde(rename = "isBest")]
    pub is_best: bool,

    /// 是否新品
    #[serde(rename = "isNew")]
    pub is_new: bool,

    /// 是否删除
    #[serde(rename = "isDel")]
    pub is_del: bool,

    /// 虚拟销量
    pub ficti: Option<i32>,

    /// 浏览量
    pub browse: Option<i32>,
}

/// 商品表头统计
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreProductTabsHeader {
    /// 数量
    pub count: i32,

    /// 名称
    pub name: String,

    /// 类型
    #[serde(rename = "type")]
    pub tab_type: i32,
}

/// 商品新增/修改请求
#[derive(Debug, Deserialize, Serialize)]
pub struct StoreProductAddRequest {
    /// 商品ID（修改时必填）
    pub id: Option<i32>,

    /// 商品名称
    #[serde(rename = "storeName")]
    pub store_name: Option<String>,

    /// 商品图片
    pub image: Option<String>,

    /// 轮播图
    #[serde(rename = "sliderImage")]
    pub slider_image: Option<String>,

    /// 商品价格
    pub price: Option<Decimal>,

    /// 库存
    pub stock: Option<i32>,

    /// 分类ID
    #[serde(rename = "cateId")]
    pub cate_id: Option<String>,

    /// 其他字段...
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

/// 商品详情响应
#[derive(Debug, Serialize)]
pub struct StoreProductInfoResponse {
    /// 商品基本信息
    pub product: StoreProductResponse,

    /// 商品详情内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// 商品属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<serde_json::Value>,
}
