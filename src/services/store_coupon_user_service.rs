/// 优惠券领取记录服务
///
/// Java参考: StoreCouponUserServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;
use std::collections::HashMap;

use crate::models::_entities::{
    store_coupon, store_coupon_user, user,
};
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_coupon_user::*;

pub struct StoreCouponUserService;

impl StoreCouponUserService {
    /// 分页列表
    ///
    /// Java参考: StoreCouponUserServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreCouponUserSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreCouponUserResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_coupon_user::Entity::find();

        // 名称模糊搜索
        if let Some(name) = &request.name {
            if !name.is_empty() {
                query = query.filter(store_coupon_user::Column::Name.contains(name));
            }
        }

        // 用户ID
        if let Some(uid) = request.uid {
            query = query.filter(store_coupon_user::Column::Uid.eq(uid));
        }

        // 状态
        if let Some(status) = request.status {
            query = query.filter(store_coupon_user::Column::Status.eq(status as i16));
        }

        // 优惠券ID
        if let Some(coupon_id) = request.coupon_id {
            query = query.filter(store_coupon_user::Column::CouponId.eq(coupon_id));
        }

        // 排序: id desc
        query = query.order_by_desc(store_coupon_user::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        if records.is_empty() {
            return Ok(CommonPage::new(vec![], total as i64, page, limit));
        }

        // 批量查询用户信息 (避免N+1)
        let uids: Vec<i32> = records.iter().map(|r| r.uid).collect();
        let users = user::Entity::find()
            .filter(user::Column::Uid.is_in(uids))
            .all(db)
            .await?;
        let user_map: HashMap<i32, &user::Model> = users.iter()
            .map(|u| (u.uid, u))
            .collect();

        let list: Vec<StoreCouponUserResponse> = records.iter()
            .map(|r| {
                let u = user_map.get(&r.uid);
                Self::model_to_response(r, u.copied())
            })
            .collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
// __PLACEHOLDER2__

    /// 管理员发放优惠券给用户
    ///
    /// Java参考: StoreCouponUserServiceImpl.receive()
    pub async fn receive(
        db: &DatabaseConnection,
        request: &StoreCouponUserRequest,
    ) -> Result<bool> {
        // 1. 查询优惠券
        let coupon = store_coupon::Entity::find_by_id(request.coupon_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("优惠券不存在"))?;

        if coupon.is_del != 0 {
            return Err(Error::string("优惠券不存在"));
        }

        // 2. 解析用户ID列表
        let mut uid_list: Vec<i32> = request.uid
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        if uid_list.is_empty() {
            return Err(Error::string("请选择用户"));
        }

        // 3. 检查库存
        if coupon.is_limited.unwrap_or(0) != 0 {
            let last_total = coupon.last_total.unwrap_or(0);
            if last_total < uid_list.len() as i32 {
                return Err(Error::string("优惠券库存不足"));
            }
        }

        // 4. 过滤已领取的用户
        let existing = store_coupon_user::Entity::find()
            .filter(store_coupon_user::Column::CouponId.eq(request.coupon_id))
            .filter(store_coupon_user::Column::Uid.is_in(uid_list.clone()))
            .all(db)
            .await?;
// __PLACEHOLDER3__

        let existing_uids: Vec<i32> = existing.iter().map(|e| e.uid).collect();
        uid_list.retain(|uid| !existing_uids.contains(uid));

        if uid_list.is_empty() {
            return Err(Error::string("所选用户均已领取过该优惠券"));
        }

        // 5. 计算使用时间
        let now = Local::now().naive_local();
        let (start_time, end_time) = if coupon.is_fixed_time.unwrap_or(0) != 0 {
            // 固定时间
            (coupon.use_start_time, coupon.use_end_time)
        } else {
            // 领取后N天
            let day = coupon.day.unwrap_or(0) as i64;
            let end = now + chrono::Duration::days(day);
            (Some(now), Some(end))
        };

        // 6. 批量创建领取记录
        let count = uid_list.len() as i32;
        for uid in &uid_list {
            let record = store_coupon_user::ActiveModel {
                coupon_id: Set(coupon.id),
                uid: Set(*uid),
                cid: Set(0),
                name: Set(coupon.name.clone()),
                money: Set(coupon.money),
                min_price: Set(coupon.min_price),
                r#type: Set("send".to_string()),
                status: Set(0i16),
                start_time: Set(start_time),
                end_time: Set(end_time),
                use_type: Set(Some(coupon.use_type)),
                primary_key: Set(if coupon.use_type > 1 {
                    Some(coupon.primary_key.clone())
                } else {
                    None
                }),
                create_time: Set(Some(now)),
                update_time: Set(Some(now)),
                ..Default::default()
            };
            record.insert(db).await?;
        }

        // 7. 扣减库存
        if coupon.is_limited.unwrap_or(0) != 0 {
            let new_last_total = coupon.last_total.unwrap_or(0) - count;
            let mut active: store_coupon::ActiveModel = coupon.into();
            active.last_total = Set(Some(new_last_total));
            active.update_time = Set(Some(now));
            active.update(db).await?;
        }

        Ok(true)
    }
    // ==================== 私有辅助方法 ====================

    fn model_to_response(
        r: &store_coupon_user::Model,
        u: Option<&user::Model>,
    ) -> StoreCouponUserResponse {
        StoreCouponUserResponse {
            id: r.id,
            coupon_id: r.coupon_id,
            cid: r.cid,
            uid: r.uid,
            name: r.name.clone(),
            money: r.money,
            min_price: r.min_price,
            coupon_type: r.r#type.clone(),
            status: r.status as i32,
            create_time: r.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            start_time: r.start_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            end_time: r.end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            use_time: r.use_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            use_type: r.use_type.unwrap_or(1) as i32,
            nickname: u.and_then(|u| u.nickname.clone()),
            avatar: u.and_then(|u| u.avatar.clone()),
        }
    }
}
