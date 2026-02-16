/// Excel导出 DTO
///
/// 对应Java:
/// - ProductExcelVo
/// - BargainProductExcelVo
/// - CombinationProductExcelVo
/// - OrderExcelVo
use serde::Serialize;

/// 商品导出Excel VO
///
/// Java: ProductExcelVo
pub struct ProductExcelVo {
    /// 商品名称
    pub store_name: String,
    /// 商品分类
    pub cate_name: String,
    /// 价格
    pub price: String,
    /// 库存
    pub stock: String,
    /// 销量
    pub sales: String,
    /// 浏览量
    pub browse: String,
}

/// 砍价商品导出Excel VO
///
/// Java: BargainProductExcelVo
pub struct BargainProductExcelVo {
    /// 砍价活动名称
    pub title: String,
    /// 砍价金额
    pub price: String,
    /// 用户每次砍价的次数
    pub bargain_num: String,
    /// 砍价状态
    pub status: String,
    /// 砍价开启时间
    pub start_time: String,
    /// 砍价结束时间
    pub stop_time: String,
    /// 销量
    pub sales: String,
    /// 库存
    pub quota_show: String,
    /// 返多少积分
    pub give_integral: String,
    /// 添加时间
    pub add_time: String,
}

/// 拼团商品导出Excel VO
///
/// Java: CombinationProductExcelVo
pub struct CombinationProductExcelVo {
    /// 编号
    pub id: String,
    /// 拼团名称
    pub title: String,
    /// 原价
    pub ot_price: String,
    /// 拼团价
    pub price: String,
    /// 库存
    pub quota_show: String,
    /// 拼团人数
    pub count_people: String,
    /// 参与人数
    pub count_people_all: String,
    /// 成团数量
    pub count_people_pink: String,
    /// 销量
    pub sales: String,
    /// 商品状态
    pub is_show: String,
    /// 拼团结束时间
    pub stop_time: String,
}

/// 订单导出Excel VO
///
/// Java: OrderExcelVo
pub struct OrderExcelVo {
    /// 订单号
    pub order_id: String,
    /// 实际支付金额
    pub pay_price: String,
    /// 创建时间
    pub create_time: String,
    /// 商品信息
    pub product_name: String,
    /// 订单状态
    pub status_str: String,
    /// 支付方式
    pub pay_type_str: String,
    /// 订单类型
    pub order_type: String,
    /// 用户姓名
    pub real_name: String,
}

/// Excel导出文件名响应
#[derive(Debug, Serialize)]
pub struct ExcelFileNameResponse {
    #[serde(rename = "fileName")]
    pub file_name: String,
}
