/// 商品管理服务
///
/// 实现与Java版本一致的商品管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::store_product;
use crate::models::_entities::store_product_attr;
use crate::models::_entities::store_product_attr_value;
use crate::models::_entities::store_product_description;
use crate::dtos::product::{
    StoreProductResponse, StoreProductSearchRequest, StoreProductTabsHeader,
    StoreProductAddRequest,
};
use crate::dtos::common::{PageParamRequest, CommonPage};

/// 商品详情响应
#[derive(Debug, Serialize, Deserialize)]
pub struct StoreProductInfoResponse {
    /// 商品基本信息
    #[serde(flatten)]
    pub product: StoreProductResponse,

    /// 商品描述
    pub content: Option<String>,

    /// 商品属性
    pub attr: Option<serde_json::Value>,

    /// 商品规格值
    #[serde(rename = "attrValue")]
    pub attr_value: Option<serde_json::Value>,
}

/// 商品服务
pub struct StoreProductService;

impl StoreProductService {
    /// 获取商品列表（管理端）
    ///
    /// 根据搜索条件和分页参数获取商品列表
    /// Java参考: StoreProductServiceImpl.getAdminList()
    pub async fn get_admin_list(
        db: &DatabaseConnection,
        request: &StoreProductSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreProductResponse>> {
        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        // 构建查询条件
        let mut query = store_product::Entity::find();

        // 根据类型筛选
        match request.product_type {
            1 => {
                // 出售中（已上架）
                query = query
                    .filter(store_product::Column::IsShow.eq(1))
                    .filter(store_product::Column::IsDel.eq(0));
            }
            2 => {
                // 仓库中（未上架）
                query = query
                    .filter(store_product::Column::IsShow.eq(0))
                    .filter(store_product::Column::IsDel.eq(0));
            }
            3 => {
                // 已售罄
                query = query
                    .filter(store_product::Column::Stock.eq(0))
                    .filter(store_product::Column::IsDel.eq(0));
            }
            4 => {
                // 警戒库存（库存 < 10）
                query = query
                    .filter(store_product::Column::Stock.lt(10))
                    .filter(store_product::Column::IsDel.eq(0));
            }
            5 => {
                // 回收站
                query = query.filter(store_product::Column::IsDel.eq(1));
            }
            _ => {}
        }

        // 分类筛选
        if let Some(cate_id) = &request.cate_id {
            if !cate_id.is_empty() {
                query = query.filter(store_product::Column::CateId.contains(cate_id));
            }
        }

        // 关键字搜索
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(store_product::Column::StoreName.contains(keywords))
                        .add(store_product::Column::Keyword.contains(keywords))
                        .add(store_product::Column::BarCode.contains(keywords))
                );
            }
        }

        // 价格排序
        if let Some(price_order) = &request.price_order {
            if price_order == "asc" {
                query = query.order_by_asc(store_product::Column::Price);
            } else if price_order == "desc" {
                query = query.order_by_desc(store_product::Column::Price);
            }
        }

        // 销量排序
        if let Some(sales_order) = &request.sales_order {
            if sales_order == "asc" {
                query = query.order_by_asc(store_product::Column::Sales);
            } else if sales_order == "desc" {
                query = query.order_by_desc(store_product::Column::Sales);
            }
        }

        // 默认按ID降序
        query = query.order_by_desc(store_product::Column::Id);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let products = paginator.fetch_page((page - 1) as u64).await?;

        // 转换为响应格式
        let list: Vec<StoreProductResponse> = products
            .into_iter()
            .map(|p| Self::model_to_response(p))
            .collect();

        Ok(CommonPage {
            list,
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 根据ID集合获取商品列表
    pub async fn get_list_in_ids(
        db: &DatabaseConnection,
        ids: Vec<i32>,
    ) -> Result<Vec<StoreProductResponse>> {
        let products = store_product::Entity::find()
            .filter(store_product::Column::Id.is_in(ids))
            .all(db)
            .await?;

        Ok(products.into_iter().map(Self::model_to_response).collect())
    }

    /// 新增商品
    pub async fn save(
        _db: &DatabaseConnection,
        request: &StoreProductAddRequest,
    ) -> Result<bool> {
        // TODO: 实现完整的新增逻辑
        tracing::info!("新增商品: {:?}", request.store_name);
        Ok(true)
    }

    /// 更新商品
    pub async fn update(
        _db: &DatabaseConnection,
        request: &StoreProductAddRequest,
    ) -> Result<bool> {
        // TODO: 实现完整的更新逻辑
        tracing::info!("更新商品: id={:?}", request.id);
        Ok(true)
    }

    /// 获取商品详情
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<StoreProductInfoResponse> {
        let product = store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        // 查询商品规格属性
        let attrs = store_product_attr::Entity::find()
            .filter(store_product_attr::Column::ProductId.eq(id))
            .filter(store_product_attr::Column::IsDel.eq(0))
            .all(db)
            .await?;

        let attr_json: Vec<serde_json::Value> = attrs.iter().map(|a| {
            serde_json::json!({
                "attrName": a.attr_name,
                "attrValue": a.attr_values.split(',').collect::<Vec<&str>>(),
            })
        }).collect();

        // 查询商品规格值（SKU）
        let attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(id))
            .filter(store_product_attr_value::Column::IsDel.eq(0))
            .all(db)
            .await?;

        let attr_value_json: Vec<serde_json::Value> = attr_values.iter().map(|v| {
            serde_json::json!({
                "id": v.id,
                "suk": v.suk,
                "stock": v.stock,
                "sales": v.sales,
                "price": v.price,
                "image": v.image,
                "cost": v.cost,
                "barCode": v.bar_code,
                "otPrice": v.ot_price,
                "weight": v.weight,
                "volume": v.volume,
                "brokerage": v.brokerage,
                "brokerageTwo": v.brokerage_two,
            })
        }).collect();

        // 查询商品描述
        let content = store_product_description::Entity::find()
            .filter(store_product_description::Column::ProductId.eq(id))
            .one(db)
            .await?
            .map(|d| d.description);

        Ok(StoreProductInfoResponse {
            product: Self::model_to_response(product),
            content,
            attr: Some(serde_json::json!(attr_json)),
            attr_value: Some(serde_json::json!(attr_value_json)),
        })
    }

    /// 获取表头统计数据
    pub async fn get_tabs_header(
        db: &DatabaseConnection,
    ) -> Result<Vec<StoreProductTabsHeader>> {
        // 出售中
        let on_sale_count = store_product::Entity::find()
            .filter(store_product::Column::IsShow.eq(1))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 仓库中
        let in_stock_count = store_product::Entity::find()
            .filter(store_product::Column::IsShow.eq(0))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 已售罄
        let sold_out_count = store_product::Entity::find()
            .filter(store_product::Column::Stock.eq(0))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 警戒库存
        let low_stock_count = store_product::Entity::find()
            .filter(store_product::Column::Stock.lt(10))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 回收站
        let deleted_count = store_product::Entity::find()
            .filter(store_product::Column::IsDel.eq(1))
            .count(db)
            .await? as i32;

        Ok(vec![
            StoreProductTabsHeader {
                count: on_sale_count,
                name: "出售中".to_string(),
                tab_type: 1,
            },
            StoreProductTabsHeader {
                count: in_stock_count,
                name: "仓库中".to_string(),
                tab_type: 2,
            },
            StoreProductTabsHeader {
                count: sold_out_count,
                name: "已售罄".to_string(),
                tab_type: 3,
            },
            StoreProductTabsHeader {
                count: low_stock_count,
                name: "警戒库存".to_string(),
                tab_type: 4,
            },
            StoreProductTabsHeader {
                count: deleted_count,
                name: "回收站".to_string(),
                tab_type: 5,
            },
        ])
    }

    /// 删除商品
    pub async fn delete_product(
        db: &DatabaseConnection,
        id: i32,
        delete_type: &str,
    ) -> Result<bool> {
        let product = store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        if delete_type == "recycle" {
            // 移到回收站
            let mut product: store_product::ActiveModel = product.into();
            product.is_del = Set(1);
            product.update(db).await?;
        } else {
            // 彻底删除
            store_product::Entity::delete_by_id(id).exec(db).await?;
        }

        Ok(true)
    }

    /// 恢复商品
    pub async fn restore_product(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let product = store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        let mut product: store_product::ActiveModel = product.into();
        product.is_del = Set(0);
        product.update(db).await?;

        Ok(true)
    }

    /// 上架商品
    pub async fn put_on_shelf(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let product = store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        let mut product: store_product::ActiveModel = product.into();
        product.is_show = Set(1);
        product.update(db).await?;

        Ok(true)
    }

    /// 下架商品
    pub async fn off_shelf(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let product = store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        let mut product: store_product::ActiveModel = product.into();
        product.is_show = Set(0);
        product.update(db).await?;

        Ok(true)
    }

    /// 快捷添加库存
    ///
    /// Java参考: StoreProductServiceImpl.stockAdd()
    /// 支持按SKU单独更新库存，同时更新主商品总库存
    pub async fn quick_add_stock(
        db: &DatabaseConnection,
        id: i32,
        stock: i32,
        attr_stocks: Option<Vec<(i32, i32)>>,
    ) -> Result<bool> {
        let product = store_product::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("商品不存在"))?;

        if let Some(items) = attr_stocks {
            // 按SKU逐个更新库存
            for (attr_value_id, add_stock) in &items {
                if let Some(av) = store_product_attr_value::Entity::find_by_id(*attr_value_id)
                    .one(db)
                    .await?
                {
                    let new_stock = av.stock + add_stock;
                    let mut av_model: store_product_attr_value::ActiveModel = av.into();
                    av_model.stock = Set(if new_stock < 0 { 0 } else { new_stock });
                    av_model.update(db).await?;
                }
            }

            // 重新计算主商品总库存 = 所有SKU库存之和
            let all_attr_values = store_product_attr_value::Entity::find()
                .filter(store_product_attr_value::Column::ProductId.eq(id))
                .filter(store_product_attr_value::Column::IsDel.eq(0))
                .all(db)
                .await?;

            let total_stock: i32 = all_attr_values.iter().map(|v| v.stock).sum();
            let mut product_model: store_product::ActiveModel = product.into();
            product_model.stock = Set(total_stock);
            product_model.update(db).await?;
        } else {
            // 无SKU数据，直接加到主商品库存
            let mut product_model: store_product::ActiveModel = product.into();
            let current_stock = *product_model.stock.as_ref();
            let new_stock = current_stock + stock;
            product_model.stock = Set(if new_stock < 0 { 0 } else { new_stock });
            product_model.update(db).await?;
        }

        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 将Model转换为Response
    fn model_to_response(model: store_product::Model) -> StoreProductResponse {
        StoreProductResponse {
            id: model.id,
            image: Some(model.image),
            slider_image: Some(model.slider_image),
            store_name: model.store_name,
            keyword: Some(model.keyword),
            bar_code: Some(model.bar_code),
            cate_id: Some(model.cate_id),
            price: model.price,
            vip_price: Some(model.vip_price),
            ot_price: Some(model.ot_price),
            postage: model.postage,
            unit_name: Some(model.unit_name),
            sort: model.sort as i32,
            sales: model.sales,
            stock: model.stock,
            is_show: model.is_show != 0,
            is_hot: model.is_hot != 0,
            is_benefit: model.is_benefit != 0,
            is_best: model.is_best != 0,
            is_new: model.is_new != 0,
            is_del: model.is_del != 0,
            ficti: model.ficti,
            browse: model.browse,
        }
    }
}
