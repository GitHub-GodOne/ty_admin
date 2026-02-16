/// 秒杀商品服务
///
/// Java参考: StoreSeckillServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Local};
use rust_decimal::Decimal;

use crate::models::_entities::{
    store_seckill, store_product,
    store_product_attr, store_product_attr_value, store_product_description,
};
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_seckill::{
    StoreSeckillSearchRequest, StoreSeckillAddRequest, StoreSeckillResponse,
};
use crate::services::store_seckill_manger_service::StoreSeckillMangerService;

/// 秒杀商品类型常量 (Java: Constants.PRODUCT_TYPE_SECKILL = 1)
const PRODUCT_TYPE_SECKILL: i16 = 1;
/// 普通商品类型常量
const PRODUCT_TYPE_NORMAL: i16 = 0;

pub struct StoreSeckillService;

impl StoreSeckillService {
    /// 分页列表
    ///
    /// Java参考: StoreSeckillServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreSeckillSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreSeckillResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_seckill::Entity::find();

        // 状态筛选
        if let Some(status) = request.status {
            query = query.filter(store_seckill::Column::Status.eq(status));
        }

        // 时间段筛选
        if let Some(time_id) = request.time_id {
            query = query.filter(store_seckill::Column::TimeId.eq(time_id));
        }

        // 未删除
        query = query.filter(store_seckill::Column::IsDel.eq(0i16));

        // 关键字搜索
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(store_seckill::Column::Title.contains(keywords))
                        .add(store_seckill::Column::Id.eq(keywords.parse::<i32>().unwrap_or(-1)))
                );
            }
        }

        // 排序: sort desc, id desc
        query = query
            .order_by_desc(store_seckill::Column::Sort)
            .order_by_desc(store_seckill::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let products = paginator.fetch_page((page - 1) as u64).await?;

        // 获取当前正在秒杀的timeId
        let current_managers = StoreSeckillMangerService::get_current_seckill_manager(db).await?;
        let current_skill_time_id = current_managers.first().map(|m| m.id).unwrap_or(0);

        // 获取所有秒杀配置
        let all_manger_list = StoreSeckillMangerService::get_all_list(db).await?;

        let mut list = Vec::new();
        for product in products {
            // 查询商品属性
            let _attrs = store_product_attr::Entity::find()
                .filter(store_product_attr::Column::ProductId.eq(product.id as i32))
                .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_SECKILL))
                .all(db)
                .await?;

            // 查询富文本
            let description = store_product_description::Entity::find()
                .filter(store_product_description::Column::ProductId.eq(product.id))
                .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_SECKILL))
                .one(db)
                .await?;

            let content = description.map(|d| d.description).unwrap_or_default();

            // 查找对应的秒杀配置
            let manger_response = all_manger_list.iter()
                .find(|m| Some(m.id) == product.time_id);

            let _current_time = manger_response.and_then(|m| m.time.clone());

            let status_name = Self::get_status_name(&product, current_skill_time_id);

            let mut resp = Self::model_to_response(&product);
            resp.status_name = status_name;
            resp.description = Some(content);

            list.push(resp);
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 逻辑删除
    ///
    /// Java参考: StoreSeckillServiceImpl.deleteById()
    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool> {
        let model = store_seckill::ActiveModel {
            id: Set(id),
            is_del: Set(1),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }

    /// 新增秒杀商品
    ///
    /// Java参考: StoreSeckillServiceImpl.saveSeckill()
    pub async fn save_seckill(
        db: &DatabaseConnection,
        request: &StoreSeckillAddRequest,
    ) -> Result<bool> {
        // 验证attrValue中的quota
        if let Some(attr_value) = &request.attr_value {
            if let Some(arr) = attr_value.as_array() {
                for item in arr {
                    let quota = item.get("quota").and_then(|v| v.as_i64()).unwrap_or(0);
                    if quota <= 0 {
                        return Err(Error::string("请正确输入限量"));
                    }
                }
            }
        }

        // 检查标题是否已存在
        if Self::is_exist_title(db, &request.title).await? {
            return Err(Error::string("活动标题已经存在"));
        }

        // 解析开始和结束时间
        let start_time = NaiveDate::parse_from_str(&request.start_time, "%Y-%m-%d")
            .map_err(|_| Error::string("开始时间格式不正确"))?
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let stop_time = NaiveDate::parse_from_str(&request.stop_time, "%Y-%m-%d")
            .map_err(|_| Error::string("结束时间格式不正确"))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        // 计算价格（从attrValue中取最小价格）
        let (price, ot_price, cost, total_quota) = Self::calc_price_and_quota(&request.attr_value);

        let sort = request.sort.unwrap_or(0);
        let spec_type = request.spec_type.unwrap_or(0);
        let now = Local::now().naive_local();

        // 插入秒杀商品
        let seckill = store_seckill::ActiveModel {
            product_id: Set(request.product_id),
            image: Set(request.image.clone()),
            images: Set(request.images.clone()),
            title: Set(request.title.clone()),
            info: Set(String::new()),
            price: Set(price),
            cost: Set(cost),
            ot_price: Set(ot_price),
            give_integral: Set(Decimal::ZERO),
            sort: Set(sort),
            stock: Set(total_quota),
            sales: Set(0),
            unit_name: Set(request.unit_name.clone()),
            postage: Set(Decimal::ZERO),
            description: Set(None),
            start_time: Set(start_time),
            stop_time: Set(stop_time),
            create_time: Set(now),
            status: Set(request.status),
            is_postage: Set(0),
            is_del: Set(0),
            num: Set(request.num),
            is_show: Set(1),
            time_id: Set(Some(request.time_id)),
            temp_id: Set(request.temp_id),
            weight: Set(Decimal::ZERO),
            volume: Set(Decimal::ZERO),
            quota: Set(total_quota),
            quota_show: Set(total_quota),
            spec_type: Set(spec_type),
            ..Default::default()
        };

        let inserted = seckill.insert(db).await?;
        let seckill_id = inserted.id;

        // 保存商品属性
        if let Some(attr) = &request.attr {
            if let Some(arr) = attr.as_array() {
                for item in arr {
                    let attr_name = item.get("attrName")
                        .or_else(|| item.get("attr_name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let attr_values = item.get("attrValues")
                        .or_else(|| item.get("attr_values"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let attr_model = store_product_attr::ActiveModel {
                        product_id: Set(seckill_id as i32),
                        attr_name: Set(attr_name.to_string()),
                        attr_values: Set(attr_values.to_string()),
                        r#type: Set(Some(PRODUCT_TYPE_SECKILL)),
                        is_del: Set(0),
                        ..Default::default()
                    };
                    attr_model.insert(db).await?;
                }
            }
        }

        // 保存商品属性值
        if let Some(attr_value) = &request.attr_value {
            if let Some(arr) = attr_value.as_array() {
                for item in arr {
                    let suk = item.get("suk").and_then(|v| v.as_str()).unwrap_or("");
                    let item_price = Self::get_decimal_from_json(item, "price");
                    let item_cost = Self::get_decimal_from_json(item, "cost");
                    let item_ot_price = Self::get_decimal_from_json(item, "otPrice");
                    let item_stock = item.get("stock").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let item_quota = item.get("quota").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let item_image = item.get("image").and_then(|v| v.as_str()).unwrap_or("");
                    let item_weight = Self::get_decimal_from_json(item, "weight");
                    let item_volume = Self::get_decimal_from_json(item, "volume");
                    let item_bar_code = item.get("barCode")
                        .or_else(|| item.get("bar_code"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let attr_value_model = store_product_attr_value::ActiveModel {
                        product_id: Set(seckill_id as i32),
                        suk: Set(suk.to_string()),
                        stock: Set(item_stock),
                        sales: Set(0),
                        price: Set(item_price),
                        image: Set(Some(item_image.to_string())),
                        unique: Set(String::new()),
                        cost: Set(item_cost),
                        bar_code: Set(item_bar_code.to_string()),
                        ot_price: Set(item_ot_price),
                        weight: Set(item_weight),
                        volume: Set(item_volume),
                        brokerage: Set(Decimal::ZERO),
                        brokerage_two: Set(Decimal::ZERO),
                        r#type: Set(Some(PRODUCT_TYPE_SECKILL)),
                        quota: Set(Some(item_quota)),
                        quota_show: Set(Some(item_quota)),
                        attr_value: Set(None),
                        is_del: Set(0),
                        version: Set(0),
                        ..Default::default()
                    };
                    attr_value_model.insert(db).await?;
                }
            }
        }

        // 保存富文本描述
        let content = request.content.as_deref().unwrap_or("");
        // 先删除旧的
        store_product_description::Entity::delete_many()
            .filter(store_product_description::Column::ProductId.eq(seckill_id))
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .exec(db)
            .await?;

        let desc_model = store_product_description::ActiveModel {
            product_id: Set(seckill_id),
            description: Set(content.to_string()),
            r#type: Set(PRODUCT_TYPE_SECKILL),
            ..Default::default()
        };
        desc_model.insert(db).await?;

        Ok(true)
    }

    /// 更新秒杀商品
    ///
    /// Java参考: StoreSeckillServiceImpl.updateSeckill()
    pub async fn update_seckill(
        db: &DatabaseConnection,
        request: &StoreSeckillAddRequest,
    ) -> Result<bool> {
        let seckill_id = request.id.ok_or_else(|| Error::string("秒杀商品ID不能为空"))?;

        let existing = store_seckill::Entity::find_by_id(seckill_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("秒杀商品不存在"))?;

        if existing.is_del != 0 {
            return Err(Error::string("秒杀商品不存在"));
        }

        if existing.status == 1 {
            return Err(Error::string("请先关闭秒杀商品，再修改商品信息"));
        }

        // 解析时间
        let start_time = NaiveDate::parse_from_str(&request.start_time, "%Y-%m-%d")
            .map_err(|_| Error::string("开始时间格式不正确"))?
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let stop_time = NaiveDate::parse_from_str(&request.stop_time, "%Y-%m-%d")
            .map_err(|_| Error::string("结束时间格式不正确"))?
            .and_hms_opt(0, 0, 0)
            .unwrap();

        // 计算价格
        let (price, ot_price, cost, total_quota) = Self::calc_price_and_quota(&request.attr_value);

        let sort = request.sort.unwrap_or(existing.sort);
        let spec_type = request.spec_type.unwrap_or(existing.spec_type);

        // 更新秒杀商品
        let seckill = store_seckill::ActiveModel {
            id: Set(seckill_id),
            product_id: Set(request.product_id),
            image: Set(request.image.clone()),
            images: Set(request.images.clone()),
            title: Set(request.title.clone()),
            price: Set(price),
            cost: Set(cost),
            ot_price: Set(ot_price),
            sort: Set(sort),
            stock: Set(total_quota),
            unit_name: Set(request.unit_name.clone()),
            start_time: Set(start_time),
            stop_time: Set(stop_time),
            status: Set(request.status),
            num: Set(request.num),
            time_id: Set(Some(request.time_id)),
            temp_id: Set(request.temp_id),
            quota: Set(total_quota),
            quota_show: Set(total_quota),
            spec_type: Set(spec_type),
            ..Default::default()
        };
        seckill.update(db).await?;

        // 删除旧的attr和attrValue
        store_product_attr::Entity::delete_many()
            .filter(store_product_attr::Column::ProductId.eq(seckill_id as i32))
            .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .exec(db)
            .await?;

        store_product_attr_value::Entity::delete_many()
            .filter(store_product_attr_value::Column::ProductId.eq(seckill_id as i32))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .exec(db)
            .await?;

        // 重新保存属性
        if let Some(attr) = &request.attr {
            if let Some(arr) = attr.as_array() {
                for item in arr {
                    let attr_name = item.get("attrName")
                        .or_else(|| item.get("attr_name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let attr_values = item.get("attrValues")
                        .or_else(|| item.get("attr_values"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let attr_model = store_product_attr::ActiveModel {
                        product_id: Set(seckill_id as i32),
                        attr_name: Set(attr_name.to_string()),
                        attr_values: Set(attr_values.to_string()),
                        r#type: Set(Some(PRODUCT_TYPE_SECKILL)),
                        is_del: Set(0),
                        ..Default::default()
                    };
                    attr_model.insert(db).await?;
                }
            }
        }

        // 重新保存属性值
        if let Some(attr_value) = &request.attr_value {
            if let Some(arr) = attr_value.as_array() {
                for item in arr {
                    let suk = item.get("suk").and_then(|v| v.as_str()).unwrap_or("");
                    let item_price = Self::get_decimal_from_json(item, "price");
                    let item_cost = Self::get_decimal_from_json(item, "cost");
                    let item_ot_price = Self::get_decimal_from_json(item, "otPrice");
                    let item_stock = item.get("stock").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let item_quota = item.get("quota").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let item_image = item.get("image").and_then(|v| v.as_str()).unwrap_or("");
                    let item_weight = Self::get_decimal_from_json(item, "weight");
                    let item_volume = Self::get_decimal_from_json(item, "volume");
                    let item_bar_code = item.get("barCode")
                        .or_else(|| item.get("bar_code"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let attr_value_model = store_product_attr_value::ActiveModel {
                        product_id: Set(seckill_id as i32),
                        suk: Set(suk.to_string()),
                        stock: Set(item_stock),
                        sales: Set(0),
                        price: Set(item_price),
                        image: Set(Some(item_image.to_string())),
                        unique: Set(String::new()),
                        cost: Set(item_cost),
                        bar_code: Set(item_bar_code.to_string()),
                        ot_price: Set(item_ot_price),
                        weight: Set(item_weight),
                        volume: Set(item_volume),
                        brokerage: Set(Decimal::ZERO),
                        brokerage_two: Set(Decimal::ZERO),
                        r#type: Set(Some(PRODUCT_TYPE_SECKILL)),
                        quota: Set(Some(item_quota)),
                        quota_show: Set(Some(item_quota)),
                        attr_value: Set(None),
                        is_del: Set(0),
                        version: Set(0),
                        ..Default::default()
                    };
                    attr_value_model.insert(db).await?;
                }
            }
        }

        // 更新富文本
        store_product_description::Entity::delete_many()
            .filter(store_product_description::Column::ProductId.eq(seckill_id))
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .exec(db)
            .await?;

        let content = request.content.as_deref().unwrap_or("");
        let desc_model = store_product_description::ActiveModel {
            product_id: Set(seckill_id),
            description: Set(content.to_string()),
            r#type: Set(PRODUCT_TYPE_SECKILL),
            ..Default::default()
        };
        desc_model.insert(db).await?;

        Ok(true)
    }

    /// 更新秒杀状态
    ///
    /// Java参考: StoreSeckillServiceImpl.updateSecKillStatus()
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let seckill = store_seckill::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("秒杀商品不存在"))?;

        if seckill.is_del != 0 {
            return Err(Error::string("秒杀商品不存在"));
        }

        if status {
            // 判断关联商品是否存在
            let product = store_product::Entity::find_by_id(seckill.product_id)
                .one(db)
                .await?;
            if product.is_none() {
                return Err(Error::string("关联的商品已删除，无法开启活动"));
            }
        }

        let status_val: i16 = if status { 1 } else { 0 };
        let model = store_seckill::ActiveModel {
            id: Set(id),
            status: Set(status_val),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }

    /// 秒杀商品详情（管理端）
    ///
    /// Java参考: StoreSeckillServiceImpl.getDetailAdmin()
    pub async fn get_detail_admin(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<serde_json::Value> {
        let seckill = store_seckill::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("未找到对应商品信息"))?;

        if seckill.is_del != 0 {
            return Err(Error::string("未找到对应商品信息"));
        }

        // 查询秒杀商品属性
        let attr_list = store_product_attr::Entity::find()
            .filter(store_product_attr::Column::ProductId.eq(id as i32))
            .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .all(db)
            .await?;

        // 查询秒杀商品sku
        let seckill_attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(id as i32))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .all(db)
            .await?;

        // 查询主商品sku
        let product_attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(seckill.product_id as i32))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_NORMAL))
            .all(db)
            .await?;

        // 组装attrValue: 对比主商品和秒杀商品的suk
        let attr_value_list: Vec<serde_json::Value> = product_attr_values.iter().map(|pav| {
            let matched = seckill_attr_values.iter().find(|sav| sav.suk == pav.suk);
            match matched {
                Some(sav) => serde_json::json!({
                    "id": sav.id,
                    "productId": sav.product_id,
                    "suk": sav.suk,
                    "stock": sav.stock,
                    "sales": sav.sales,
                    "price": sav.price,
                    "image": sav.image,
                    "cost": sav.cost,
                    "barCode": sav.bar_code,
                    "otPrice": sav.ot_price,
                    "weight": sav.weight,
                    "volume": sav.volume,
                    "quota": sav.quota,
                    "quotaShow": sav.quota_show,
                }),
                None => serde_json::json!({
                    "id": null,
                    "productId": pav.product_id,
                    "suk": pav.suk,
                    "stock": pav.stock,
                    "sales": pav.sales,
                    "price": pav.price,
                    "image": pav.image,
                    "cost": pav.cost,
                    "barCode": pav.bar_code,
                    "otPrice": pav.ot_price,
                    "weight": pav.weight,
                    "volume": pav.volume,
                    "quota": pav.quota,
                    "quotaShow": pav.quota_show,
                }),
            }
        }).collect();

        // 查询富文本
        let description = store_product_description::Entity::find()
            .filter(store_product_description::Column::ProductId.eq(id))
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_SECKILL))
            .one(db)
            .await?;

        let content = description.map(|d| d.description).unwrap_or_default();

        // 组装attr
        let attr_json: Vec<serde_json::Value> = attr_list.iter().map(|a| {
            serde_json::json!({
                "id": a.id,
                "productId": a.product_id,
                "attrName": a.attr_name,
                "attrValues": a.attr_values,
                "type": a.r#type,
            })
        }).collect();

        let start_time_str = seckill.start_time.format("%Y-%m-%d").to_string();
        let stop_time_str = seckill.stop_time.format("%Y-%m-%d").to_string();

        Ok(serde_json::json!({
            "id": seckill.id,
            "productId": seckill.product_id,
            "storeName": seckill.title,
            "image": seckill.image,
            "sliderImage": seckill.images,
            "price": seckill.price,
            "cost": seckill.cost,
            "otPrice": seckill.ot_price,
            "stock": seckill.stock,
            "sales": seckill.sales,
            "unitName": seckill.unit_name,
            "sort": seckill.sort,
            "postage": seckill.postage,
            "status": seckill.status,
            "num": seckill.num,
            "timeId": seckill.time_id,
            "tempId": seckill.temp_id,
            "specType": seckill.spec_type,
            "startTimeStr": start_time_str,
            "stopTimeStr": stop_time_str,
            "attr": attr_json,
            "attrValue": attr_value_list,
            "content": content,
        }))
    }

    // ==================== 私有方法 ====================

    /// 获取秒杀状态描述
    fn get_status_name(seckill: &store_seckill::Model, current_skill_time_id: i32) -> String {
        if seckill.status == 0 {
            return "已关闭".to_string();
        }

        let now = Local::now().naive_local();
        let start_date = seckill.start_time.date();
        let stop_date = seckill.stop_time.date();
        let start_datetime = NaiveDateTime::new(start_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let stop_datetime = NaiveDateTime::new(stop_date, NaiveTime::from_hms_opt(23, 59, 59).unwrap());

        if now < start_datetime {
            return "未开始".to_string();
        }

        if now >= start_datetime && now <= stop_datetime {
            if seckill.time_id == Some(current_skill_time_id) {
                return "进行中".to_string();
            }
            return "未开始".to_string();
        }

        "已结束".to_string()
    }

    /// 检查标题是否已存在
    async fn is_exist_title(db: &DatabaseConnection, title: &str) -> Result<bool> {
        let count = store_seckill::Entity::find()
            .filter(store_seckill::Column::Title.eq(title))
            .filter(store_seckill::Column::IsDel.eq(0i16))
            .count(db)
            .await?;
        Ok(count > 0)
    }

    /// 从attrValue计算价格和总限量
    fn calc_price_and_quota(attr_value: &Option<serde_json::Value>) -> (Decimal, Decimal, Decimal, i32) {
        let mut min_price = Decimal::MAX;
        let mut min_ot_price = Decimal::ZERO;
        let mut min_cost = Decimal::ZERO;
        let mut total_quota = 0i32;

        if let Some(value) = attr_value {
            if let Some(arr) = value.as_array() {
                for item in arr {
                    let price = Self::get_decimal_from_json(item, "price");
                    let ot_price = Self::get_decimal_from_json(item, "otPrice");
                    let cost = Self::get_decimal_from_json(item, "cost");
                    let quota = item.get("quota").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

                    if price < min_price {
                        min_price = price;
                        min_ot_price = ot_price;
                        min_cost = cost;
                    }
                    total_quota += quota;
                }
            }
        }

        if min_price == Decimal::MAX {
            min_price = Decimal::ZERO;
        }

        (min_price, min_ot_price, min_cost, total_quota)
    }

    /// 从JSON中获取Decimal值
    fn get_decimal_from_json(json: &serde_json::Value, key: &str) -> Decimal {
        json.get(key)
            .and_then(|v| {
                if let Some(s) = v.as_str() {
                    s.parse::<Decimal>().ok()
                } else if let Some(f) = v.as_f64() {
                    Decimal::try_from(f).ok()
                } else if let Some(i) = v.as_i64() {
                    Some(Decimal::from(i))
                } else {
                    None
                }
            })
            .unwrap_or(Decimal::ZERO)
    }

    /// Model -> Response 转换
    fn model_to_response(model: &store_seckill::Model) -> StoreSeckillResponse {
        StoreSeckillResponse {
            id: model.id,
            product_id: model.product_id,
            image: model.image.clone(),
            images: model.images.clone(),
            title: model.title.clone(),
            info: model.info.clone(),
            price: model.price,
            cost: model.cost,
            ot_price: model.ot_price,
            give_integral: model.give_integral,
            sort: model.sort,
            stock: model.stock,
            sales: model.sales,
            unit_name: model.unit_name.clone(),
            postage: model.postage,
            description: model.description.clone(),
            start_time: model.start_time.format("%Y-%m-%d").to_string(),
            stop_time: model.stop_time.format("%Y-%m-%d").to_string(),
            create_time: model.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            status: model.status,
            status_name: String::new(),
            is_postage: model.is_postage != 0,
            num: model.num,
            is_show: model.is_show != 0,
            time_id: model.time_id,
            temp_id: model.temp_id,
            weight: model.weight,
            volume: model.volume,
            quota: model.quota,
            quota_show: model.quota_show,
            spec_type: model.spec_type,
            kill_status: 0,
        }
    }
}
