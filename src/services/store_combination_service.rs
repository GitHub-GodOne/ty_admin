/// 拼团商品服务
///
/// Java参考: StoreCombinationServiceImpl, StorePinkServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;
use rust_decimal::Decimal;

use crate::models::_entities::{
    store_combination, store_pink, store_product, store_order,
    store_product_attr, store_product_attr_value, store_product_description,
};
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_combination::*;

/// 拼团商品类型常量 (Java: Constants.PRODUCT_TYPE_PINGTUAN = 3)
const PRODUCT_TYPE_PINGTUAN: i16 = 3;
/// 普通商品类型常量
const PRODUCT_TYPE_NORMAL: i16 = 0;

pub struct StoreCombinationService;

impl StoreCombinationService {
    /// 分页列表
    ///
    /// Java参考: StoreCombinationServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreCombinationSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreCombinationResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_combination::Entity::find();

        // 未删除
        query = query.filter(store_combination::Column::IsDel.eq(0i16));

        // 关键字搜索 (productId / id / title)
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(store_combination::Column::Id.eq(keywords.parse::<i32>().unwrap_or(-1)))
                        .add(store_combination::Column::ProductId.eq(keywords.parse::<i32>().unwrap_or(-1)))
                        .add(store_combination::Column::Title.contains(keywords))
                );
            }
        }
        // isShow筛选
        if let Some(is_show) = request.is_show {
            query = query.filter(store_combination::Column::IsShow.eq(is_show));
        }

        // 排序: sort desc, id desc
        query = query
            .order_by_desc(store_combination::Column::Sort)
            .order_by_desc(store_combination::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let products = paginator.fetch_page((page - 1) as u64).await?;

        let mut list = Vec::new();
        for product in products {
            let cid = product.id;

            // 开团数量: kId=0 的团长记录数
            let count_people = store_pink::Entity::find()
                .filter(store_pink::Column::Cid.eq(cid))
                .filter(store_pink::Column::KId.eq(0))
                .count(db)
                .await? as i64;

            // 成团数量: kId=0 且 status=2 的团长记录数
            let count_people_pink = store_pink::Entity::find()
                .filter(store_pink::Column::Cid.eq(cid))
                .filter(store_pink::Column::KId.eq(0))
                .filter(store_pink::Column::Status.eq(2i16))
                .count(db)
                .await? as i64;

            // 参团人数: 所有参与者
            let count_people_all = store_pink::Entity::find()
                .filter(store_pink::Column::Cid.eq(cid))
                .count(db)
                .await? as i64;

            // 限量剩余
            let remaining_quota = product.quota;

            let mut resp = Self::model_to_response(&product);
            resp.count_people = count_people;
            resp.count_people_pink = count_people_pink;
            resp.count_people_all = count_people_all;
            resp.remaining_quota = remaining_quota;

            list.push(resp);
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
    /// 新增拼团商品
    ///
    /// Java参考: StoreCombinationServiceImpl.saveCombination()
    pub async fn save_combination(
        db: &DatabaseConnection,
        request: &StoreCombinationRequest,
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

        // 解析时间为毫秒时间戳
        let start_time = Self::parse_date_to_millis(&request.start_time)?;
        let stop_time = Self::parse_date_to_millis(&request.stop_time)?;

        // 计算价格（从attrValue中取最小价格）
        let (price, ot_price, cost, total_quota) = Self::calc_price_and_quota(&request.attr_value);

        let sort = request.sort.unwrap_or(0);
        let now_millis = Local::now().timestamp_millis();

        let combination = store_combination::ActiveModel {
            product_id: Set(request.product_id),
            image: Set(request.image.clone()),
            images: Set(request.images.clone()),
            title: Set(request.title.clone()),
            attr: Set(None),
            people: Set(request.people),
            info: Set(String::new()),
            price: Set(price),
            sort: Set(sort),
            sales: Set(0),
            stock: Set(total_quota),
            add_time: Set(now_millis),
            is_host: Set(0),
            is_show: Set(if request.is_show { 1i16 } else { 0i16 }),
            is_del: Set(0i16),
            combination: Set(None),
            mer_use: Set(None),
            is_postage: Set(Some(0i16)),
            postage: Set(Some(Decimal::ZERO)),
            start_time: Set(start_time),
            stop_time: Set(stop_time),
            effective_time: Set(request.effective_time),
            cost: Set(cost),
            browse: Set(Some(0)),
            unit_name: Set(request.unit_name.clone()),
            temp_id: Set(request.temp_id),
            weight: Set(Some(Decimal::ZERO)),
            volume: Set(Some(Decimal::ZERO)),
            num: Set(request.num),
            quota: Set(total_quota),
            quota_show: Set(total_quota),
            ot_price: Set(ot_price),
            once_num: Set(request.once_num.unwrap_or(0)),
            virtual_ration: Set(request.virtual_ration.unwrap_or(0)),
            mer_id: Set(None),
            ..Default::default()
        };

        let inserted = combination.insert(db).await?;
        let combination_id = inserted.id;

        // 保存商品属性
        Self::save_attrs(db, combination_id, &request.attr).await?;
        // 保存商品属性值
        Self::save_attr_values(db, combination_id, &request.attr_value).await?;
        // 保存富文本描述
        Self::save_description(db, combination_id, request.content.as_deref()).await?;

        Ok(true)
    }
    /// 删除拼团商品（逻辑删除）
    ///
    /// Java参考: StoreCombinationServiceImpl.deleteById()
    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool> {
        let combination = store_combination::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("拼团商品不存在"))?;

        if combination.is_del != 0 {
            return Err(Error::string("拼团商品不存在"));
        }

        // 检查活动是否正在进行中
        if Self::is_activity_running(&combination) {
            return Err(Error::string("活动进行中，无法删除"));
        }

        // 逻辑删除
        let model = store_combination::ActiveModel {
            id: Set(id),
            is_del: Set(1i16),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }

    /// 修改拼团商品
    ///
    /// Java参考: StoreCombinationServiceImpl.updateCombination()
    pub async fn update_combination(
        db: &DatabaseConnection,
        request: &StoreCombinationRequest,
    ) -> Result<bool> {
        let combination_id = request.id.ok_or_else(|| Error::string("拼团商品ID不能为空"))?;

        let existing = store_combination::Entity::find_by_id(combination_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("拼团商品不存在"))?;

        if existing.is_del != 0 {
            return Err(Error::string("拼团商品不存在"));
        }

        // 检查活动是否正在进行中
        if Self::is_activity_running(&existing) {
            return Err(Error::string("活动进行中，无法修改"));
        }

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
        // 解析时间
        let start_time = Self::parse_date_to_millis(&request.start_time)?;
        let stop_time = Self::parse_date_to_millis(&request.stop_time)?;

        // 计算价格
        let (price, ot_price, cost, total_quota) = Self::calc_price_and_quota(&request.attr_value);

        let sort = request.sort.unwrap_or(existing.sort);

        // 更新拼团商品
        let combination = store_combination::ActiveModel {
            id: Set(combination_id),
            product_id: Set(request.product_id),
            image: Set(request.image.clone()),
            images: Set(request.images.clone()),
            title: Set(request.title.clone()),
            people: Set(request.people),
            price: Set(price),
            sort: Set(sort),
            stock: Set(total_quota),
            is_show: Set(if request.is_show { 1i16 } else { 0i16 }),
            start_time: Set(start_time),
            stop_time: Set(stop_time),
            effective_time: Set(request.effective_time),
            cost: Set(cost),
            unit_name: Set(request.unit_name.clone()),
            temp_id: Set(request.temp_id),
            num: Set(request.num),
            quota: Set(total_quota),
            quota_show: Set(total_quota),
            ot_price: Set(ot_price),
            once_num: Set(request.once_num.unwrap_or(existing.once_num)),
            virtual_ration: Set(request.virtual_ration.unwrap_or(existing.virtual_ration)),
            ..Default::default()
        };
        combination.update(db).await?;

        // 删除旧的attr和attrValue
        store_product_attr::Entity::delete_many()
            .filter(store_product_attr::Column::ProductId.eq(combination_id))
            .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_PINGTUAN))
            .exec(db)
            .await?;

        store_product_attr_value::Entity::delete_many()
            .filter(store_product_attr_value::Column::ProductId.eq(combination_id))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_PINGTUAN))
            .exec(db)
            .await?;

        // 重新保存属性
        Self::save_attrs(db, combination_id, &request.attr).await?;
        Self::save_attr_values(db, combination_id, &request.attr_value).await?;

        // 更新富文本
        Self::save_description(db, combination_id, request.content.as_deref()).await?;

        Ok(true)
    }
    /// 更新拼团商品显示状态
    ///
    /// Java参考: StoreCombinationServiceImpl.updateCombinationShow()
    pub async fn update_combination_show(
        db: &DatabaseConnection,
        id: i32,
        is_show: bool,
    ) -> Result<bool> {
        let combination = store_combination::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("拼团商品不存在"))?;

        if combination.is_del != 0 {
            return Err(Error::string("拼团商品不存在"));
        }

        if is_show {
            // 判断关联商品是否存在
            let product = store_product::Entity::find_by_id(combination.product_id)
                .one(db)
                .await?;
            if product.is_none() {
                return Err(Error::string("关联的商品已删除，无法开启活动"));
            }
        }

        let is_show_val: i16 = if is_show { 1 } else { 0 };
        let model = store_combination::ActiveModel {
            id: Set(id),
            is_show: Set(is_show_val),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }

    /// 拼团商品详情（管理端）
    ///
    /// Java参考: StoreCombinationServiceImpl.getAdminDetail()
    pub async fn get_admin_detail(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<serde_json::Value> {
        let combination = store_combination::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("未找到对应拼团商品信息"))?;

        if combination.is_del != 0 {
            return Err(Error::string("未找到对应拼团商品信息"));
        }

        // 查询拼团商品属性
        let attr_list = store_product_attr::Entity::find()
            .filter(store_product_attr::Column::ProductId.eq(id))
            .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_PINGTUAN))
            .all(db)
            .await?;

        // 查询拼团商品sku
        let combination_attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(id))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_PINGTUAN))
            .all(db)
            .await?;
        // 查询主商品sku
        let product_attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(combination.product_id))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_NORMAL))
            .all(db)
            .await?;

        // 组装attrValue: 对比主商品和拼团商品的suk
        let attr_value_list: Vec<serde_json::Value> = product_attr_values.iter().map(|pav| {
            let matched = combination_attr_values.iter().find(|cav| cav.suk == pav.suk);
            match matched {
                Some(cav) => serde_json::json!({
                    "id": cav.id,
                    "productId": cav.product_id,
                    "suk": cav.suk,
                    "stock": cav.stock,
                    "sales": cav.sales,
                    "price": cav.price,
                    "image": cav.image,
                    "cost": cav.cost,
                    "barCode": cav.bar_code,
                    "otPrice": cav.ot_price,
                    "weight": cav.weight,
                    "volume": cav.volume,
                    "quota": cav.quota,
                    "quotaShow": cav.quota_show,
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
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_PINGTUAN))
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

        // 判断specType: 有属性则为多规格
        let spec_type: i16 = if attr_list.is_empty() { 0 } else { 1 };

        let start_time_str = Self::millis_to_date_string(combination.start_time);
        let stop_time_str = Self::millis_to_date_string(combination.stop_time);

        Ok(serde_json::json!({
            "id": combination.id,
            "productId": combination.product_id,
            "storeName": combination.title,
            "image": combination.image,
            "sliderImage": combination.images,
            "price": combination.price,
            "cost": combination.cost,
            "otPrice": combination.ot_price,
            "stock": combination.stock,
            "sales": combination.sales,
            "unitName": combination.unit_name,
            "sort": combination.sort,
            "postage": combination.postage,
            "isShow": combination.is_show != 0,
            "people": combination.people,
            "effectiveTime": combination.effective_time,
            "num": combination.num,
            "onceNum": combination.once_num,
            "virtualRation": combination.virtual_ration,
            "tempId": combination.temp_id,
            "specType": spec_type,
            "startTimeStr": start_time_str,
            "stopTimeStr": stop_time_str,
            "attr": attr_json,
            "attrValue": attr_value_list,
            "content": content,
        }))
    }
    /// 拼团统计
    ///
    /// Java参考: StoreCombinationServiceImpl.getAdminStatistics()
    pub async fn get_admin_statistics(db: &DatabaseConnection) -> Result<CombinationStatisticsResponse> {
        // 参与人数: 所有StorePink记录数
        let count_people = store_pink::Entity::find()
            .count(db)
            .await? as i64;

        // 成团数量: kId=0 且 status=2 的团长记录数
        let count_people_pink = store_pink::Entity::find()
            .filter(store_pink::Column::KId.eq(0))
            .filter(store_pink::Column::Status.eq(2i16))
            .count(db)
            .await? as i64;

        Ok(CombinationStatisticsResponse {
            count_people,
            count_people_pink,
        })
    }

    /// 拼团记录列表（团长列表）
    ///
    /// Java参考: StorePinkServiceImpl.getList()
    pub async fn get_pink_list(
        db: &DatabaseConnection,
        request: &StorePinkSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StorePinkAdminListResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_pink::Entity::find();

        // 只查团长 (kId=0)
        query = query.filter(store_pink::Column::KId.eq(0));

        // 状态筛选
        if let Some(status) = request.status {
            query = query.filter(store_pink::Column::Status.eq(status));
        }

        // 时间筛选 (dateLimit: "2024-01-01,2024-01-31")
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                let parts: Vec<&str> = date_limit.split(',').collect();
                if parts.len() == 2 {
                    let start_millis = Self::parse_date_to_millis(parts[0].trim());
                    let end_millis = Self::parse_date_to_millis(parts[1].trim());
                    if let (Ok(s), Ok(e)) = (start_millis, end_millis) {
                        query = query.filter(store_pink::Column::AddTime.gte(s));
                        // 结束时间加一天的毫秒数
                        query = query.filter(store_pink::Column::AddTime.lt(e + 86400000));
                    }
                }
            }
        }
        // 排序: id desc
        query = query.order_by_desc(store_pink::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let mut list = Vec::new();
        for record in records {
            // 参团人数: 同一个团(kId=record.id 或 id=record.id) 且 同一个cid
            let count_people = store_pink::Entity::find()
                .filter(
                    Condition::any()
                        .add(store_pink::Column::KId.eq(record.id))
                        .add(store_pink::Column::Id.eq(record.id))
                )
                .filter(store_pink::Column::Cid.eq(record.cid))
                .count(db)
                .await? as i64;

            // 拼团商品标题
            let title = store_combination::Entity::find_by_id(record.cid)
                .one(db)
                .await?
                .map(|c| c.title)
                .unwrap_or_default();

            list.push(StorePinkAdminListResponse {
                id: record.id,
                uid: record.uid,
                people: record.people,
                add_time: Self::millis_to_datetime_string(record.add_time),
                stop_time: Self::millis_to_datetime_string(record.stop_time),
                k_id: record.k_id,
                status: record.status,
                nickname: record.nickname.clone(),
                avatar: record.avatar.clone(),
                count_people,
                title,
            });
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
    /// 拼团订单详情
    ///
    /// Java参考: StorePinkServiceImpl.getAdminList()
    pub async fn get_pink_detail(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Vec<StorePinkDetailResponse>> {
        // 查询团长和团员: id=id 或 kId=id
        let pinks = store_pink::Entity::find()
            .filter(
                Condition::any()
                    .add(store_pink::Column::Id.eq(id))
                    .add(store_pink::Column::KId.eq(id))
            )
            .all(db)
            .await?;

        let mut list = Vec::new();
        for pink in pinks {
            // 查询订单状态
            let (order_status_str, refund_status_str) = if !pink.order_id.is_empty() {
                let order = store_order::Entity::find()
                    .filter(store_order::Column::OrderId.eq(&pink.order_id))
                    .one(db)
                    .await?;
                match order {
                    Some(o) => (
                        Self::get_order_status_name(o.status),
                        Self::get_refund_status_name(o.refund_status),
                    ),
                    None => ("未知".to_string(), "未知".to_string()),
                }
            } else {
                ("未知".to_string(), "未知".to_string())
            };

            list.push(StorePinkDetailResponse {
                id: pink.id,
                uid: pink.uid,
                order_id: pink.order_id.clone(),
                total_price: pink.total_price,
                nickname: pink.nickname.clone(),
                avatar: pink.avatar.clone(),
                order_status: order_status_str,
                refund_status: refund_status_str,
            });
        }

        Ok(list)
    }

    // ==================== 私有方法 ====================

    /// 检查活动是否正在进行中
    fn is_activity_running(combination: &store_combination::Model) -> bool {
        if combination.is_show == 0 {
            return false;
        }
        let now_millis = Local::now().timestamp_millis();
        now_millis >= combination.start_time && now_millis <= combination.stop_time
    }
    /// 保存商品属性
    async fn save_attrs(
        db: &DatabaseConnection,
        combination_id: i32,
        attr: &Option<serde_json::Value>,
    ) -> Result<()> {
        if let Some(attr) = attr {
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
                        product_id: Set(combination_id),
                        attr_name: Set(attr_name.to_string()),
                        attr_values: Set(attr_values.to_string()),
                        r#type: Set(Some(PRODUCT_TYPE_PINGTUAN)),
                        is_del: Set(0),
                        ..Default::default()
                    };
                    attr_model.insert(db).await?;
                }
            }
        }
        Ok(())
    }

    /// 保存商品属性值
    async fn save_attr_values(
        db: &DatabaseConnection,
        combination_id: i32,
        attr_value: &Option<serde_json::Value>,
    ) -> Result<()> {
        if let Some(attr_value) = attr_value {
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
                        product_id: Set(combination_id),
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
                        r#type: Set(Some(PRODUCT_TYPE_PINGTUAN)),
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
        Ok(())
    }

    /// 保存富文本描述
    async fn save_description(
        db: &DatabaseConnection,
        combination_id: i32,
        content: Option<&str>,
    ) -> Result<()> {
        // 先删除旧的
        store_product_description::Entity::delete_many()
            .filter(store_product_description::Column::ProductId.eq(combination_id))
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_PINGTUAN))
            .exec(db)
            .await?;

        let desc_content = content.unwrap_or("");
        let desc_model = store_product_description::ActiveModel {
            product_id: Set(combination_id),
            description: Set(desc_content.to_string()),
            r#type: Set(PRODUCT_TYPE_PINGTUAN),
            ..Default::default()
        };
        desc_model.insert(db).await?;
        Ok(())
    }
    /// 从attrValue计算价格和总限量
    /// 返回 (price, otPrice, cost, totalQuota)
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

    /// 解析日期字符串为毫秒时间戳
    fn parse_date_to_millis(date_str: &str) -> Result<i64> {
        use chrono::NaiveDate;
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| Error::string("日期格式不正确，应为 yyyy-MM-dd"))?;
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();
        Ok(datetime.and_local_timezone(Local).unwrap().timestamp_millis())
    }

    /// 毫秒时间戳转日期字符串
    fn millis_to_date_string(millis: i64) -> String {
        use chrono::{DateTime, FixedOffset};
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        let secs = millis / 1000;
        let nsecs = ((millis % 1000) * 1_000_000) as u32;
        let dt = DateTime::from_timestamp(secs, nsecs)
            .map(|d| d.with_timezone(&offset));
        match dt {
            Some(d) => d.format("%Y-%m-%d").to_string(),
            None => String::new(),
        }
    }

    /// 毫秒时间戳转日期时间字符串
    fn millis_to_datetime_string(millis: i64) -> String {
        use chrono::{DateTime, FixedOffset};
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        let secs = millis / 1000;
        let nsecs = ((millis % 1000) * 1_000_000) as u32;
        let dt = DateTime::from_timestamp(secs, nsecs)
            .map(|d| d.with_timezone(&offset));
        match dt {
            Some(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => String::new(),
        }
    }
    /// 获取订单状态名称
    fn get_order_status_name(status: i16) -> String {
        match status {
            0 => "待发货".to_string(),
            1 => "待收货".to_string(),
            2 => "待评价".to_string(),
            3 => "已完成".to_string(),
            _ => "未知".to_string(),
        }
    }

    /// 获取退款状态名称
    fn get_refund_status_name(refund_status: i16) -> String {
        match refund_status {
            0 => "未退款".to_string(),
            1 => "申请中".to_string(),
            2 => "已退款".to_string(),
            3 => "退款中".to_string(),
            _ => "未知".to_string(),
        }
    }

    /// Model -> Response 转换
    fn model_to_response(model: &store_combination::Model) -> StoreCombinationResponse {
        StoreCombinationResponse {
            id: model.id,
            product_id: model.product_id,
            image: model.image.clone(),
            images: model.images.clone(),
            title: model.title.clone(),
            price: model.price,
            cost: model.cost,
            ot_price: model.ot_price,
            sort: model.sort,
            stock: model.stock,
            sales: model.sales,
            unit_name: model.unit_name.clone(),
            postage: model.postage,
            is_postage: model.is_postage.unwrap_or(0) != 0,
            is_show: model.is_show != 0,
            is_del: model.is_del != 0,
            people: model.people,
            start_time: Self::millis_to_date_string(model.start_time),
            stop_time: Self::millis_to_date_string(model.stop_time),
            effective_time: model.effective_time,
            add_time: Self::millis_to_datetime_string(model.add_time),
            info: model.info.clone(),
            temp_id: model.temp_id,
            quota: model.quota,
            quota_show: model.quota_show,
            once_num: model.once_num,
            virtual_ration: model.virtual_ration,
            weight: model.weight,
            volume: model.volume,
            num: model.num,
            count_people_all: 0,
            count_people_pink: 0,
            count_people: 0,
            remaining_quota: model.quota,
            stop_time_str: Self::millis_to_datetime_string(model.stop_time),
        }
    }
}
