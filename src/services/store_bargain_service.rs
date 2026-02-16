/// 砍价商品服务
///
/// Java参考: StoreBargainServiceImpl, StoreBargainUserServiceImpl, StoreBargainUserHelpServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;
use rust_decimal::Decimal;

use crate::models::_entities::{
    store_bargain, store_bargain_user, store_bargain_user_help,
    store_product, store_product_attr, store_product_attr_value,
    store_product_description, user,
};
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_bargain::*;

/// 砍价商品类型常量 (Java: Constants.PRODUCT_TYPE_BARGAIN = 2)
const PRODUCT_TYPE_BARGAIN: i16 = 2;
/// 普通商品类型常量
const PRODUCT_TYPE_NORMAL: i16 = 0;

pub struct StoreBargainService;

impl StoreBargainService {
    /// 分页列表
    ///
    /// Java参考: StoreBargainServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreBargainSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreBargainResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_bargain::Entity::find();

        // 未删除
        query = query.filter(store_bargain::Column::IsDel.eq(0i16));

        // 关键字搜索
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(store_bargain::Column::Id.eq(keywords.parse::<i32>().unwrap_or(-1)))
                        .add(store_bargain::Column::StoreName.contains(keywords))
                        .add(store_bargain::Column::Title.contains(keywords))
                );
            }
        }
        // 状态筛选
        if let Some(status) = request.status {
            query = query.filter(store_bargain::Column::Status.eq(status));
        }

        // 排序: sort desc, id desc
        query = query
            .order_by_desc(store_bargain::Column::Sort)
            .order_by_desc(store_bargain::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let products = paginator.fetch_page((page - 1) as u64).await?;

        let mut list = Vec::new();
        for product in products {
            let bargain_id = product.id;

            // 砍价参与人数 (Java: storeBargainUserService.getListByBargainId)
            let count_people_all = store_bargain_user::Entity::find()
                .filter(store_bargain_user::Column::BargainId.eq(Some(bargain_id)))
                .filter(store_bargain_user::Column::IsDel.eq(0i16))
                .count(db)
                .await? as i64;

            // 帮忙砍价人数 (Java: storeBargainUserHelpService.getHelpCountByBargainId)
            let count_people_help = store_bargain_user_help::Entity::find()
                .filter(store_bargain_user_help::Column::BargainId.eq(Some(bargain_id)))
                .count(db)
                .await? as i64;

            // 砍价成功人数 (status=3)
            let count_people_success = store_bargain_user::Entity::find()
                .filter(store_bargain_user::Column::BargainId.eq(Some(bargain_id)))
                .filter(store_bargain_user::Column::Status.eq(3i16))
                .filter(store_bargain_user::Column::IsDel.eq(0i16))
                .count(db)
                .await? as i64;

            // 限量剩余 = quota - (quota_show - quota)... Java: surplusQuota = quota
            // Java中 surplusQuota 直接取 quota 字段
            let surplus_quota = product.quota;

            let mut resp = Self::model_to_response(&product);
            resp.count_people_all = count_people_all;
            resp.count_people_help = count_people_help;
            resp.count_people_success = count_people_success;
            resp.surplus_quota = surplus_quota;

            list.push(resp);
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
    /// 新增砍价商品
    ///
    /// Java参考: StoreBargainServiceImpl.saveBargain()
    pub async fn save_bargain(
        db: &DatabaseConnection,
        request: &StoreBargainRequest,
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

        // 验证价格差 >= peopleNum * 0.01
        // Java: if (price.subtract(minPrice).compareTo(multiply) < 0)
        if let Some(attr_value) = &request.attr_value {
            if let Some(arr) = attr_value.as_array() {
                for item in arr {
                    let price = Self::get_decimal_from_json(item, "price");
                    let min_price = Self::get_decimal_from_json(item, "minPrice");
                    let people_num = Decimal::from(request.people_num);
                    let min_diff = people_num * Decimal::new(1, 2); // peopleNum * 0.01
                    if price - min_price < min_diff {
                        return Err(Error::string("砍价金额不能低于砍价人数*0.01"));
                    }
                }
            }
        }

        // 解析时间为时间戳(秒)
        let start_time = Self::parse_date_to_timestamp(&request.start_time)?;
        let stop_time = Self::parse_date_to_timestamp(&request.stop_time)?;

        // 计算价格（从attrValue中取最小价格）
        let (price, min_price, cost, total_quota) = Self::calc_price_and_quota(&request.attr_value);

        let sort = request.sort.unwrap_or(0);
        let now_ts = Local::now().timestamp();

        // 插入砍价商品
        let bargain = store_bargain::ActiveModel {
            product_id: Set(request.product_id),
            title: Set(request.title.clone()),
            image: Set(request.image.clone()),
            unit_name: Set(Some(request.unit_name.clone())),
            images: Set(request.images.clone()),
            start_time: Set(start_time),
            stop_time: Set(stop_time),
            store_name: Set(request.store_name.clone()),
            price: Set(Some(price)),
            min_price: Set(Some(min_price)),
            num: Set(Some(request.num)),
            bargain_num: Set(request.bargain_num),
            status: Set(if request.status { 1i16 } else { 0i16 }),
            give_integral: Set(None),
            info: Set(request.info.clone()),
            cost: Set(Some(cost)),
            sort: Set(sort),
            is_hot: Set(0i16),
            is_del: Set(0i16),
            add_time: Set(Some(now_ts)),
            is_postage: Set(0i16),
            postage: Set(Some(Decimal::ZERO)),
            rule: Set(request.rule.clone()),
            look: Set(Some(0)),
            share: Set(Some(0)),
            temp_id: Set(Some(request.temp_id)),
            weight: Set(Some(Decimal::ZERO)),
            volume: Set(Some(Decimal::ZERO)),
            quota: Set(total_quota),
            quota_show: Set(total_quota),
            people_num: Set(Some(request.people_num)),
            stock: Set(Some(total_quota)),
            sales: Set(Some(0)),
            bargain_max_price: Set(None),
            bargain_min_price: Set(None),
            ..Default::default()
        };

        let inserted = bargain.insert(db).await?;
        let bargain_id = inserted.id;

        // 保存商品属性
        Self::save_attrs(db, bargain_id, &request.attr).await?;

        // 保存商品属性值
        Self::save_attr_values(db, bargain_id, &request.attr_value).await?;

        // 保存富文本描述
        Self::save_description(db, bargain_id, request.content.as_deref()).await?;

        Ok(true)
    }
    /// 删除砍价商品（逻辑删除）
    ///
    /// Java参考: StoreBargainServiceImpl.deleteById()
    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool> {
        let bargain = store_bargain::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("砍价商品不存在"))?;

        if bargain.is_del != 0 {
            return Err(Error::string("砍价商品不存在"));
        }

        // 检查活动是否正在进行中
        if Self::is_activity_running(&bargain) {
            return Err(Error::string("活动进行中，无法删除"));
        }

        // 逻辑删除
        let model = store_bargain::ActiveModel {
            id: Set(id),
            is_del: Set(1i16),
            ..Default::default()
        };
        model.update(db).await?;

        // 将正在参与的用户状态改为失败(status=2)
        // Java: storeBargainUserService.deleteByBargainId(id)
        let participating_users = store_bargain_user::Entity::find()
            .filter(store_bargain_user::Column::BargainId.eq(Some(id)))
            .filter(store_bargain_user::Column::Status.eq(1i16))
            .all(db)
            .await?;

        for u in participating_users {
            let update_model = store_bargain_user::ActiveModel {
                id: Set(u.id),
                status: Set(2i16),
                ..Default::default()
            };
            update_model.update(db).await?;
        }

        Ok(true)
    }
    /// 修改砍价商品
    ///
    /// Java参考: StoreBargainServiceImpl.updateBargain()
    pub async fn update_bargain(
        db: &DatabaseConnection,
        request: &StoreBargainRequest,
    ) -> Result<bool> {
        let bargain_id = request.id.ok_or_else(|| Error::string("砍价商品ID不能为空"))?;

        let existing = store_bargain::Entity::find_by_id(bargain_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("砍价商品不存在"))?;

        if existing.is_del != 0 {
            return Err(Error::string("砍价商品不存在"));
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

        // 验证价格差 >= peopleNum * 0.01
        if let Some(attr_value) = &request.attr_value {
            if let Some(arr) = attr_value.as_array() {
                for item in arr {
                    let price = Self::get_decimal_from_json(item, "price");
                    let min_price_val = Self::get_decimal_from_json(item, "minPrice");
                    let people_num = Decimal::from(request.people_num);
                    let min_diff = people_num * Decimal::new(1, 2);
                    if price - min_price_val < min_diff {
                        return Err(Error::string("砍价金额不能低于砍价人数*0.01"));
                    }
                }
            }
        }
        // 解析时间
        let start_time = Self::parse_date_to_timestamp(&request.start_time)?;
        let stop_time = Self::parse_date_to_timestamp(&request.stop_time)?;

        // 计算价格
        let (price, min_price, cost, total_quota) = Self::calc_price_and_quota(&request.attr_value);

        let sort = request.sort.unwrap_or(existing.sort);

        // 更新砍价商品
        let bargain = store_bargain::ActiveModel {
            id: Set(bargain_id),
            product_id: Set(request.product_id),
            title: Set(request.title.clone()),
            image: Set(request.image.clone()),
            unit_name: Set(Some(request.unit_name.clone())),
            images: Set(request.images.clone()),
            start_time: Set(start_time),
            stop_time: Set(stop_time),
            store_name: Set(request.store_name.clone()),
            price: Set(Some(price)),
            min_price: Set(Some(min_price)),
            num: Set(Some(request.num)),
            bargain_num: Set(request.bargain_num),
            status: Set(if request.status { 1i16 } else { 0i16 }),
            info: Set(request.info.clone()),
            cost: Set(Some(cost)),
            sort: Set(sort),
            temp_id: Set(Some(request.temp_id)),
            quota: Set(total_quota),
            quota_show: Set(total_quota),
            people_num: Set(Some(request.people_num)),
            stock: Set(Some(total_quota)),
            rule: Set(request.rule.clone()),
            ..Default::default()
        };
        bargain.update(db).await?;

        // 删除旧的attr和attrValue
        store_product_attr::Entity::delete_many()
            .filter(store_product_attr::Column::ProductId.eq(bargain_id))
            .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_BARGAIN))
            .exec(db)
            .await?;

        store_product_attr_value::Entity::delete_many()
            .filter(store_product_attr_value::Column::ProductId.eq(bargain_id))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_BARGAIN))
            .exec(db)
            .await?;

        // 重新保存属性
        Self::save_attrs(db, bargain_id, &request.attr).await?;
        Self::save_attr_values(db, bargain_id, &request.attr_value).await?;

        // 更新富文本
        Self::save_description(db, bargain_id, request.content.as_deref()).await?;

        Ok(true)
    }
    /// 更新砍价状态
    ///
    /// Java参考: StoreBargainServiceImpl.updateBargainStatus()
    pub async fn update_bargain_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let bargain = store_bargain::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("砍价商品不存在"))?;

        if bargain.is_del != 0 {
            return Err(Error::string("砍价商品不存在"));
        }

        if status {
            // 判断关联商品是否存在
            let product = store_product::Entity::find_by_id(bargain.product_id)
                .one(db)
                .await?;
            if product.is_none() {
                return Err(Error::string("关联的商品已删除，无法开启活动"));
            }
        }

        let status_val: i16 = if status { 1 } else { 0 };
        let model = store_bargain::ActiveModel {
            id: Set(id),
            status: Set(status_val),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }
    /// 砍价商品详情（管理端）
    ///
    /// Java参考: StoreBargainServiceImpl.getAdminDetail()
    pub async fn get_admin_detail(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<serde_json::Value> {
        let bargain = store_bargain::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("未找到对应砍价商品信息"))?;

        if bargain.is_del != 0 {
            return Err(Error::string("未找到对应砍价商品信息"));
        }

        // 查询砍价商品属性
        let attr_list = store_product_attr::Entity::find()
            .filter(store_product_attr::Column::ProductId.eq(id))
            .filter(store_product_attr::Column::Type.eq(PRODUCT_TYPE_BARGAIN))
            .all(db)
            .await?;

        // 查询砍价商品sku
        let bargain_attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(id))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_BARGAIN))
            .all(db)
            .await?;

        // 查询主商品sku
        let product_attr_values = store_product_attr_value::Entity::find()
            .filter(store_product_attr_value::Column::ProductId.eq(bargain.product_id))
            .filter(store_product_attr_value::Column::Type.eq(PRODUCT_TYPE_NORMAL))
            .all(db)
            .await?;

        // 组装attrValue: 对比主商品和砍价商品的suk
        let attr_value_list: Vec<serde_json::Value> = product_attr_values.iter().map(|pav| {
            let matched = bargain_attr_values.iter().find(|bav| bav.suk == pav.suk);
            match matched {
                Some(bav) => serde_json::json!({
                    "id": bav.id,
                    "productId": bav.product_id,
                    "suk": bav.suk,
                    "stock": bav.stock,
                    "sales": bav.sales,
                    "price": bav.price,
                    "image": bav.image,
                    "cost": bav.cost,
                    "barCode": bav.bar_code,
                    "otPrice": bav.ot_price,
                    "weight": bav.weight,
                    "volume": bav.volume,
                    "quota": bav.quota,
                    "quotaShow": bav.quota_show,
                    "minPrice": bargain.min_price,
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
                    "minPrice": null,
                }),
            }
        }).collect();

        // 查询富文本
        let description = store_product_description::Entity::find()
            .filter(store_product_description::Column::ProductId.eq(id))
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_BARGAIN))
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

        // 时间戳转字符串
        let start_time_str = Self::timestamp_to_date_string(bargain.start_time);
        let stop_time_str = Self::timestamp_to_date_string(bargain.stop_time);

        Ok(serde_json::json!({
            "id": bargain.id,
            "productId": bargain.product_id,
            "title": bargain.title,
            "image": bargain.image,
            "sliderImage": bargain.images,
            "unitName": bargain.unit_name,
            "storeName": bargain.store_name,
            "price": bargain.price,
            "minPrice": bargain.min_price,
            "cost": bargain.cost,
            "stock": bargain.stock,
            "sales": bargain.sales,
            "sort": bargain.sort,
            "postage": bargain.postage,
            "status": bargain.status,
            "num": bargain.num,
            "bargainNum": bargain.bargain_num,
            "peopleNum": bargain.people_num,
            "tempId": bargain.temp_id,
            "quota": bargain.quota,
            "quotaShow": bargain.quota_show,
            "rule": bargain.rule,
            "info": bargain.info,
            "startTimeStr": start_time_str,
            "stopTimeStr": stop_time_str,
            "attr": attr_json,
            "attrValue": attr_value_list,
            "content": content,
        }))
    }
    /// 砍价用户列表
    ///
    /// Java参考: StoreBargainUserServiceImpl.getList()
    pub async fn get_bargain_user_list(
        db: &DatabaseConnection,
        request: &StoreBargainUserSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreBargainUserResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_bargain_user::Entity::find();

        // 未删除
        query = query.filter(store_bargain_user::Column::IsDel.eq(0i16));

        // 状态筛选
        if let Some(status) = request.status {
            query = query.filter(store_bargain_user::Column::Status.eq(status));
        }

        // 时间筛选 (Java: dateLimit)
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                // 格式: "2024-01-01,2024-01-31"
                let parts: Vec<&str> = date_limit.split(',').collect();
                if parts.len() == 2 {
                    let start_ts = Self::parse_date_to_timestamp(parts[0].trim());
                    let end_ts = Self::parse_date_to_timestamp(parts[1].trim());
                    if let (Ok(s), Ok(e)) = (start_ts, end_ts) {
                        query = query.filter(store_bargain_user::Column::AddTime.gte(Some(s)));
                        // 结束时间加一天的秒数
                        query = query.filter(store_bargain_user::Column::AddTime.lt(Some(e + 86400)));
                    }
                }
            }
        }

        // 排序: id desc
        query = query.order_by_desc(store_bargain_user::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let mut list = Vec::new();
        for record in records {
            // 查询用户信息
            let (avatar, nickname) = if let Some(uid) = record.uid {
                let u = user::Entity::find_by_id(uid).one(db).await?;
                match u {
                    Some(u) => (u.avatar.clone(), u.nickname.clone()),
                    None => (None, None),
                }
            } else {
                (None, None)
            };
            // 查询砍价商品信息
            let (title, people_num) = if let Some(bargain_id) = record.bargain_id {
                let b = store_bargain::Entity::find_by_id(bargain_id).one(db).await?;
                match b {
                    Some(b) => (Some(b.title), b.people_num),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

            // 剩余砍价次数 = bargain.bargainNum - 已帮砍次数
            let help_count = if let Some(bargain_id) = record.bargain_id {
                store_bargain_user_help::Entity::find()
                    .filter(store_bargain_user_help::Column::BargainId.eq(Some(bargain_id)))
                    .filter(store_bargain_user_help::Column::BargainUserId.eq(Some(record.id)))
                    .count(db)
                    .await? as i32
            } else {
                0
            };

            let bargain_num = if let Some(bargain_id) = record.bargain_id {
                let b = store_bargain::Entity::find_by_id(bargain_id).one(db).await?;
                b.map(|b| b.bargain_num).unwrap_or(0)
            } else {
                0
            };

            let remaining_num = bargain_num - help_count;

            // 当前价格 = bargainPrice - price
            let now_price = match (record.bargain_price, record.price) {
                (Some(bp), Some(p)) => Some(bp - p),
                _ => None,
            };

            let add_time_str = record.add_time.map(|ts| Self::timestamp_to_datetime_string(ts));
            let data_time_str = record.add_time.map(|ts| Self::timestamp_to_date_string(ts));

            list.push(StoreBargainUserResponse {
                id: record.id,
                uid: record.uid,
                bargain_id: record.bargain_id,
                bargain_price_min: record.bargain_price_min,
                bargain_price: record.bargain_price,
                price: record.price,
                status: record.status,
                add_time: add_time_str,
                avatar,
                data_time: data_time_str,
                nickname,
                now_price,
                num: remaining_num,
                people_num,
                title,
            });
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
    /// 砍价帮助记录列表
    ///
    /// Java参考: StoreBargainUserHelpServiceImpl.getList()
    pub async fn get_bargain_user_help_list(
        db: &DatabaseConnection,
        bargain_user_id: i32,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreBargainUserHelpResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_bargain_user_help::Entity::find();

        // 按砍价用户ID筛选
        query = query.filter(store_bargain_user_help::Column::BargainUserId.eq(Some(bargain_user_id)));

        // 排序: id desc
        query = query.order_by_desc(store_bargain_user_help::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let mut list = Vec::new();
        for record in records {
            // 查询用户信息
            let (avatar, nickname) = if let Some(uid) = record.uid {
                let u = user::Entity::find_by_id(uid).one(db).await?;
                match u {
                    Some(u) => (u.avatar.clone(), u.nickname.clone()),
                    None => (None, None),
                }
            } else {
                (None, None)
            };

            let add_time_str = record.add_time.map(|ts| Self::timestamp_to_datetime_string(ts));

            list.push(StoreBargainUserHelpResponse {
                id: record.id,
                uid: record.uid,
                bargain_id: record.bargain_id,
                bargain_user_id: record.bargain_user_id,
                price: record.price,
                add_time: add_time_str,
                avatar,
                nickname,
            });
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    // ==================== 私有方法 ====================
    /// 检查活动是否正在进行中
    fn is_activity_running(bargain: &store_bargain::Model) -> bool {
        if bargain.status == 0 {
            return false;
        }
        let now_ts = Local::now().timestamp();
        now_ts >= bargain.start_time && now_ts <= bargain.stop_time
    }

    /// 保存商品属性
    async fn save_attrs(
        db: &DatabaseConnection,
        bargain_id: i32,
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
                        product_id: Set(bargain_id),
                        attr_name: Set(attr_name.to_string()),
                        attr_values: Set(attr_values.to_string()),
                        r#type: Set(Some(PRODUCT_TYPE_BARGAIN)),
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
        bargain_id: i32,
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
                        product_id: Set(bargain_id),
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
                        r#type: Set(Some(PRODUCT_TYPE_BARGAIN)),
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
        bargain_id: i32,
        content: Option<&str>,
    ) -> Result<()> {
        // 先删除旧的
        store_product_description::Entity::delete_many()
            .filter(store_product_description::Column::ProductId.eq(bargain_id))
            .filter(store_product_description::Column::Type.eq(PRODUCT_TYPE_BARGAIN))
            .exec(db)
            .await?;

        let desc_content = content.unwrap_or("");
        let desc_model = store_product_description::ActiveModel {
            product_id: Set(bargain_id),
            description: Set(desc_content.to_string()),
            r#type: Set(PRODUCT_TYPE_BARGAIN),
            ..Default::default()
        };
        desc_model.insert(db).await?;
        Ok(())
    }
    /// 从attrValue计算价格和总限量
    /// 返回 (price, minPrice, cost, totalQuota)
    fn calc_price_and_quota(attr_value: &Option<serde_json::Value>) -> (Decimal, Decimal, Decimal, i32) {
        let mut min_price = Decimal::MAX;
        let mut min_min_price = Decimal::ZERO;
        let mut min_cost = Decimal::ZERO;
        let mut total_quota = 0i32;

        if let Some(value) = attr_value {
            if let Some(arr) = value.as_array() {
                for item in arr {
                    let price = Self::get_decimal_from_json(item, "price");
                    let item_min_price = Self::get_decimal_from_json(item, "minPrice");
                    let cost = Self::get_decimal_from_json(item, "cost");
                    let quota = item.get("quota").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

                    if price < min_price {
                        min_price = price;
                        min_min_price = item_min_price;
                        min_cost = cost;
                    }
                    total_quota += quota;
                }
            }
        }

        if min_price == Decimal::MAX {
            min_price = Decimal::ZERO;
        }

        (min_price, min_min_price, min_cost, total_quota)
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

    /// 解析日期字符串为时间戳(秒)
    fn parse_date_to_timestamp(date_str: &str) -> Result<i64> {
        use chrono::NaiveDate;
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
            .map_err(|_| Error::string("日期格式不正确，应为 yyyy-MM-dd"))?;
        let datetime = date.and_hms_opt(0, 0, 0).unwrap();
        Ok(datetime.and_local_timezone(Local).unwrap().timestamp())
    }

    /// 时间戳(秒)转日期字符串
    fn timestamp_to_date_string(ts: i64) -> String {
        use chrono::{DateTime, FixedOffset};
        let offset = FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8
        let dt = DateTime::from_timestamp(ts, 0)
            .map(|d| d.with_timezone(&offset));
        match dt {
            Some(d) => d.format("%Y-%m-%d").to_string(),
            None => String::new(),
        }
    }

    /// 时间戳(秒)转日期时间字符串
    fn timestamp_to_datetime_string(ts: i64) -> String {
        use chrono::{DateTime, FixedOffset};
        let offset = FixedOffset::east_opt(8 * 3600).unwrap();
        let dt = DateTime::from_timestamp(ts, 0)
            .map(|d| d.with_timezone(&offset));
        match dt {
            Some(d) => d.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => String::new(),
        }
    }

    /// Model -> Response 转换
    fn model_to_response(model: &store_bargain::Model) -> StoreBargainResponse {
        StoreBargainResponse {
            id: model.id,
            product_id: model.product_id,
            title: model.title.clone(),
            image: model.image.clone(),
            unit_name: model.unit_name.clone(),
            stock: model.stock,
            sales: model.sales,
            images: model.images.clone(),
            start_time: Self::timestamp_to_date_string(model.start_time),
            stop_time: Self::timestamp_to_date_string(model.stop_time),
            store_name: model.store_name.clone(),
            price: model.price,
            min_price: model.min_price,
            num: model.num,
            bargain_max_price: model.bargain_max_price,
            bargain_min_price: model.bargain_min_price,
            bargain_num: model.bargain_num,
            status: model.status,
            give_integral: model.give_integral,
            info: model.info.clone(),
            cost: model.cost,
            sort: model.sort,
            is_hot: model.is_hot != 0,
            is_del: model.is_del != 0,
            add_time: model.add_time.map(|ts| Self::timestamp_to_datetime_string(ts)),
            is_postage: model.is_postage != 0,
            postage: model.postage,
            rule: model.rule.clone(),
            look: model.look,
            share: model.share,
            temp_id: model.temp_id,
            weight: model.weight,
            volume: model.volume,
            quota: model.quota,
            quota_show: model.quota_show,
            people_num: model.people_num,
            surplus_quota: model.quota,
            count_people_all: 0,
            count_people_help: 0,
            count_people_success: 0,
        }
    }
}


