/// 优惠券服务
///
/// Java参考: StoreCouponServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::models::_entities::{
    store_coupon, store_product, category,
};
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_coupon::*;

pub struct StoreCouponService;

impl StoreCouponService {
    /// 分页列表
    ///
    /// Java参考: StoreCouponServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreCouponSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreCouponResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_coupon::Entity::find();

        // 未删除
        query = query.filter(store_coupon::Column::IsDel.eq(0));

        // 优惠券类型
        if let Some(coupon_type) = request.coupon_type {
            query = query.filter(store_coupon::Column::Type.eq(coupon_type));
        }

        // 使用类型
        if let Some(use_type) = request.use_type {
            query = query.filter(store_coupon::Column::UseType.eq(use_type));
        }

        // 状态
        if let Some(status) = request.status {
            query = query.filter(store_coupon::Column::Status.eq(status));
        }

        // 名称模糊搜索
        if let Some(name) = &request.name {
            if !name.is_empty() {
                query = query.filter(store_coupon::Column::Name.contains(name));
            }
        }

        // 排序: sort desc, id desc
        query = query
            .order_by_desc(store_coupon::Column::Sort)
            .order_by_desc(store_coupon::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let coupons = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<StoreCouponResponse> = coupons.iter()
            .map(|c| Self::model_to_response(c))
            .collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 新增优惠券
    ///
    /// Java参考: StoreCouponServiceImpl.create()
    pub async fn create(
        db: &DatabaseConnection,
        request: &StoreCouponRequest,
    ) -> Result<bool> {
        // 验证: 限量时total必须>0
        if request.is_limited {
            let total = request.total.unwrap_or(0);
            if total <= 0 {
                return Err(Error::string("请输入数量！"));
            }
        }

        // 验证: useType>1时primaryKey必填
        if request.use_type > 1 {
            let pk = request.primary_key.as_deref().unwrap_or("");
            if pk.is_empty() {
                return Err(Error::string("请选择商品/分类！"));
            }
        }

        let now = Local::now().naive_local();

        // 处理领取时间
        let (receive_start, receive_end) = Self::parse_receive_time(request)?;

        // 处理使用时间
        let (is_fixed_time, use_start, use_end, day) = Self::parse_use_time(request)?;

        let total = if request.is_limited { request.total.unwrap_or(0) } else { 0 };

        let coupon = store_coupon::ActiveModel {
            name: Set(request.name.clone()),
            money: Set(request.money),
            is_limited: Set(Some(if request.is_limited { 1i16 } else { 0i16 })),
            total: Set(total),
            last_total: Set(Some(total)),
            use_type: Set(request.use_type as i16),
            primary_key: Set(request.primary_key.clone().unwrap_or_default()),
            min_price: Set(request.min_price),
            receive_start_time: Set(receive_start),
            receive_end_time: Set(receive_end),
            is_fixed_time: Set(Some(is_fixed_time as i16)),
            use_start_time: Set(use_start),
            use_end_time: Set(use_end),
            day: Set(day),
            r#type: Set(request.coupon_type as i16),
            sort: Set(request.sort.unwrap_or(0)),
            status: Set(if request.status.unwrap_or(true) { 1i16 } else { 0i16 }),
            is_del: Set(0i16),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };

        coupon.insert(db).await?;
        Ok(true)
    }

    /// 修改优惠券状态
    ///
    /// Java参考: StoreCouponServiceImpl.updateStatus()
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let coupon = store_coupon::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("优惠券不存在"))?;

        let current_status = coupon.status != 0;
        if current_status == status {
            return Err(Error::string("优惠券状态无需变更"));
        }

        let now = Local::now().naive_local();
        let mut active: store_coupon::ActiveModel = coupon.into();
        active.status = Set(if status { 1i16 } else { 0i16 });
        active.update_time = Set(Some(now));
        active.update(db).await?;

        Ok(true)
    }

    /// 优惠券详情
    ///
    /// Java参考: StoreCouponServiceImpl.info()
    pub async fn info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<StoreCouponInfoResponse> {
        let coupon = store_coupon::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("优惠券信息不存在或者已失效！"))?;

        if coupon.is_del != 0 || coupon.status == 0 {
            return Err(Error::string("优惠券信息不存在或者已失效！"));
        }

        // 构建coupon request DTO
        let coupon_dto = Self::model_to_request(&coupon);

        let mut product_list: Vec<serde_json::Value> = vec![];
        let mut category_list: Vec<serde_json::Value> = vec![];

        // 解析primaryKey
        let primary_key = &coupon.primary_key;
        if !primary_key.is_empty() {
            let ids: Vec<i32> = primary_key
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();

            if coupon.use_type == 2 {
                // 商品券: 加载关联商品
                for pid in &ids {
                    if let Some(p) = store_product::Entity::find_by_id(*pid).one(db).await? {
                        product_list.push(serde_json::json!({
                            "id": p.id,
                            "storeName": p.store_name,
                            "image": p.image,
                            "price": p.price,
                        }));
                    }
                }
            } else if coupon.use_type == 3 {
                // 品类券: 加载关联品类
                for cid in &ids {
                    if let Some(c) = category::Entity::find_by_id(*cid).one(db).await? {
                        category_list.push(serde_json::json!({
                            "id": c.id,
                            "name": c.name,
                        }));
                    }
                }
            }
        }

        Ok(StoreCouponInfoResponse {
            coupon: coupon_dto,
            product: product_list,
            category: category_list,
        })
    }

    /// 发送优惠券列表
    ///
    /// Java参考: StoreCouponServiceImpl.getSendList()
    /// 筛选条件: 未删除、状态开启、未过期、有剩余数量
    pub async fn get_send_list(
        db: &DatabaseConnection,
        request: &CouponSendListRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreCouponSendResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let now = Local::now().naive_local();

        let mut query = store_coupon::Entity::find();

        // 未删除
        query = query.filter(store_coupon::Column::IsDel.eq(0));
        // 状态开启
        query = query.filter(store_coupon::Column::Status.eq(1));

        // 数量条件: 不限量 或 剩余数量>=0
        query = query.filter(
            Condition::any()
                .add(store_coupon::Column::IsLimited.eq(0))
                .add(store_coupon::Column::LastTotal.gte(0))
        );

        // 领取时间未过期: receiveEndTime为空 或 receiveEndTime > 当前时间
        use sea_orm::sea_query::Expr;
        query = query.filter(
            Condition::any()
                .add(store_coupon::Column::ReceiveEndTime.is_null())
                .add(Expr::cust(&format!(
                    "receive_end_time > '{}'::timestamp",
                    now.format("%Y-%m-%d %H:%M:%S")
                )))
        );

        // 优惠券类型
        if let Some(coupon_type) = request.coupon_type {
            query = query.filter(store_coupon::Column::Type.eq(coupon_type));
        }

        // 关键词搜索
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(store_coupon::Column::Name.contains(keywords));
            }
        }

        // 排序: sort desc, id desc
        query = query
            .order_by_desc(store_coupon::Column::Sort)
            .order_by_desc(store_coupon::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let coupons = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<StoreCouponSendResponse> = coupons.iter()
            .map(|c| Self::model_to_send_response(c))
            .collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 删除优惠券 (软删除)
    ///
    /// Java参考: StoreCouponServiceImpl.delete()
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let coupon = store_coupon::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("优惠券不存在"))?;

        if coupon.is_del != 0 {
            return Err(Error::string("优惠券不存在"));
        }

        let now = Local::now().naive_local();
        let mut active: store_coupon::ActiveModel = coupon.into();
        active.is_del = Set(1i16);
        active.update_time = Set(Some(now));
        active.update(db).await?;

        Ok(true)
    }

    // ==================== 私有辅助方法 ====================

    /// Model -> StoreCouponResponse
    fn model_to_response(c: &store_coupon::Model) -> StoreCouponResponse {
        StoreCouponResponse {
            id: c.id,
            name: c.name.clone(),
            money: c.money,
            is_limited: c.is_limited.unwrap_or(0) != 0,
            total: c.total,
            last_total: c.last_total.unwrap_or(0),
            use_type: c.use_type as i32,
            primary_key: c.primary_key.clone(),
            min_price: c.min_price,
            receive_start_time: Some(c.receive_start_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            receive_end_time: c.receive_end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            is_fixed_time: c.is_fixed_time.unwrap_or(0) != 0,
            use_start_time: c.use_start_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            use_end_time: c.use_end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            day: c.day.unwrap_or(0),
            coupon_type: c.r#type as i32,
            sort: c.sort,
            status: c.status != 0,
            is_del: c.is_del != 0,
            create_time: c.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: c.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }

    /// Model -> StoreCouponSendResponse (精简字段)
    fn model_to_send_response(c: &store_coupon::Model) -> StoreCouponSendResponse {
        StoreCouponSendResponse {
            id: c.id,
            name: c.name.clone(),
            money: c.money,
            is_limited: c.is_limited.unwrap_or(0) != 0,
            total: c.total,
            last_total: c.last_total.unwrap_or(0),
            use_type: c.use_type as i32,
            min_price: c.min_price,
            coupon_type: c.r#type as i32,
        }
    }

    /// Model -> StoreCouponRequest (用于详情返回)
    fn model_to_request(c: &store_coupon::Model) -> StoreCouponRequest {
        let is_forever = c.receive_end_time.is_some();
        StoreCouponRequest {
            id: Some(c.id),
            name: c.name.clone(),
            money: c.money,
            is_limited: c.is_limited.unwrap_or(0) != 0,
            total: Some(c.total),
            use_type: c.use_type as i32,
            primary_key: Some(c.primary_key.clone()),
            min_price: c.min_price,
            is_forever: Some(is_forever),
            receive_start_time: Some(c.receive_start_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            receive_end_time: c.receive_end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            is_fixed_time: Some(c.is_fixed_time.unwrap_or(0) != 0),
            use_start_time: c.use_start_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            use_end_time: c.use_end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            day: c.day,
            coupon_type: c.r#type as i32,
            sort: Some(c.sort),
            status: Some(c.status != 0),
        }
    }

    /// 解析领取时间
    ///
    /// isForever=false: receiveStartTime设为当前时间, receiveEndTime为None
    /// isForever=true: 使用请求中的时间范围
    fn parse_receive_time(
        request: &StoreCouponRequest,
    ) -> Result<(chrono::NaiveDateTime, Option<chrono::NaiveDateTime>)> {
        let is_forever = request.is_forever.unwrap_or(false);

        if !is_forever {
            // 不限时: 领取开始时间为当前时间
            let now = Local::now().naive_local();
            Ok((now, None))
        } else {
            // 有固定领取时间
            let start_str = request.receive_start_time.as_deref()
                .ok_or_else(|| Error::string("请选择领取时间范围！"))?;
            let end_str = request.receive_end_time.as_deref()
                .ok_or_else(|| Error::string("请选择领取时间范围！"))?;

            let start = chrono::NaiveDateTime::parse_from_str(start_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| Error::string("领取开始时间格式错误"))?;
            let end = chrono::NaiveDateTime::parse_from_str(end_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| Error::string("领取结束时间格式错误"))?;

            if start >= end {
                return Err(Error::string("请选择正确的领取时间范围！"));
            }

            Ok((start, Some(end)))
        }
    }

    /// 解析使用时间
    ///
    /// isFixedTime=false: 使用天数, useStartTime/useEndTime为None
    /// isFixedTime=true: 使用固定时间范围
    ///
    /// 返回: (is_fixed_time_int, use_start, use_end, day)
    fn parse_use_time(
        request: &StoreCouponRequest,
    ) -> Result<(i32, Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>, Option<i32>)> {
        let is_fixed_time = request.is_fixed_time.unwrap_or(false);

        if !is_fixed_time {
            // 非固定时间: 使用天数
            let day = request.day.unwrap_or(0);
            if day <= 0 {
                return Err(Error::string("请输入天数！"));
            }
            Ok((0, None, None, Some(day)))
        } else {
            // 固定时间
            let start_str = request.use_start_time.as_deref()
                .ok_or_else(|| Error::string("请选择使用时间范围"))?;
            let end_str = request.use_end_time.as_deref()
                .ok_or_else(|| Error::string("请选择使用时间范围"))?;

            let start = chrono::NaiveDateTime::parse_from_str(start_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| Error::string("使用开始时间格式错误"))?;
            let end = chrono::NaiveDateTime::parse_from_str(end_str, "%Y-%m-%d %H:%M:%S")
                .map_err(|_| Error::string("使用结束时间格式错误"))?;

            Ok((1, Some(start), Some(end), None))
        }
    }
}
