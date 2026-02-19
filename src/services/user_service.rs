/// 用户管理服务
///
/// 实现与Java版本一致的用户管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;
use sea_orm::sea_query::Expr;
use std::collections::HashMap;

use crate::models::_entities::user;
use crate::models::_entities::user_group;
use crate::models::_entities::user_tag;
use crate::models::_entities::store_order;
use crate::models::_entities::user_integral_record;
use crate::models::_entities::user_sign;
use crate::models::_entities::store_coupon_user;
use crate::models::_entities::user_bill;
use crate::dtos::user::{
    UserSearchRequest, UserResponse, OrderRecordResponse, IntegralRecordResponse,
    SignRecordResponse, CouponUserResponse, BillRecordResponse, UserRelationResponse,
    UserConditionResponse,
};
use crate::dtos::common::{PageParamRequest, CommonPage};

/// 用户服务
pub struct UserService;

impl UserService {
    /// 获取用户列表（管理端）
    ///
    /// 根据搜索条件和分页参数获取用户列表
    /// Java参考: UserServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &UserSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserResponse>> {
        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        // 构建查询条件
        let mut query = user::Entity::find();

        // 关键字搜索（账号/昵称/手机号/真实姓名）
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(user::Column::Account.contains(keywords))
                        .add(user::Column::Nickname.contains(keywords))
                        .add(user::Column::Phone.contains(keywords))
                        .add(user::Column::RealName.contains(keywords))
                );
            }
        }

        // 用户标签筛选 (支持tagId和labelId两种参数名)
        let tag_id = request.tag_id.as_ref().or(request.label_id.as_ref());
        if let Some(tag_id) = tag_id {
            if !tag_id.is_empty() {
                query = query.filter(user::Column::TagId.contains(tag_id));
            }
        }

        // 用户分组筛选
        if let Some(group_id) = &request.group_id {
            if !group_id.is_empty() {
                query = query.filter(user::Column::GroupId.eq(group_id));
            }
        }
        // 用户类型筛选
        if let Some(user_type) = &request.user_type {
            if !user_type.is_empty() {
                query = query.filter(user::Column::UserType.eq(user_type));
            }
        }

        // 用户状态筛选
        if let Some(status) = request.status {
            query = query.filter(user::Column::Status.eq(status));
        }

        // 是否为推广员筛选
        if let Some(is_promoter) = request.is_promoter {
            query = query.filter(user::Column::IsPromoter.eq(is_promoter));
        }

        // 用户等级筛选
        if let Some(level) = request.level {
            query = query.filter(user::Column::Level.eq(level));
        }

        // 性别筛选
        if let Some(sex) = request.sex {
            query = query.filter(user::Column::Sex.eq(sex));
        }

        // 国家筛选
        if let Some(country) = &request.country {
            if !country.is_empty() {
                query = query.filter(user::Column::Country.eq(country));
            }
        }

        // 注册时间范围筛选
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                // 解析日期范围，格式: "2024-01-01,2024-01-31"
                let dates: Vec<&str> = date_limit.split(',').collect();
                if dates.len() == 2 {
                    let start_date = dates[0].trim();
                    let end_date = dates[1].trim();
                    if !start_date.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("create_time >= '{} 00:00:00'::timestamp", start_date))
                        );
                    }
                    if !end_date.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("create_time <= '{} 23:59:59'::timestamp", end_date))
                        );
                    }
                }
            }
        }

        // 消费次数筛选
        if let Some(pay_count_min) = request.pay_count_min {
            query = query.filter(user::Column::PayCount.gte(pay_count_min));
        }
        if let Some(pay_count_max) = request.pay_count_max {
            query = query.filter(user::Column::PayCount.lte(pay_count_max));
        }

        // 访问情况筛选
        if let Some(access_type) = &request.access_type {
            match access_type.as_str() {
                "visitedToday" | "1" => {
                    // 今日访问
                    query = query.filter(
                        Expr::cust("last_login_time >= CURRENT_DATE::timestamp")
                    );
                }
                "visitedYesterday" | "2" => {
                    // 昨日访问
                    query = query.filter(
                        Expr::cust("last_login_time >= (CURRENT_DATE - INTERVAL '1 day')::timestamp AND last_login_time < CURRENT_DATE::timestamp")
                    );
                }
                "notVisited7Days" | "3" => {
                    // 7天未访问
                    query = query.filter(
                        Condition::any()
                            .add(user::Column::LastLoginTime.is_null())
                            .add(Expr::cust("last_login_time < (CURRENT_DATE - INTERVAL '7 days')::timestamp"))
                    );
                }
                "notVisited30Days" | "4" => {
                    // 30天未访问
                    query = query.filter(
                        Condition::any()
                            .add(user::Column::LastLoginTime.is_null())
                            .add(Expr::cust("last_login_time < (CURRENT_DATE - INTERVAL '30 days')::timestamp"))
                    );
                }
                // "0" 或其他值表示不筛选
                _ => {}
            }
        }

        // 默认按UID降序排列
        query = query.order_by_desc(user::Column::Uid);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let users = paginator.fetch_page((page - 1) as u64).await?;

        // 收集需要查询的ID
        let group_ids: Vec<i32> = users.iter()
            .filter_map(|u| u.group_id.as_ref().and_then(|g| g.parse::<i32>().ok()))
            .collect();
        let spread_uids: Vec<i32> = users.iter()
            .filter_map(|u| u.spread_uid)
            .filter(|&uid| uid > 0)
            .collect();
        let tag_ids: Vec<i32> = users.iter()
            .filter_map(|u| u.tag_id.as_ref())
            .flat_map(|t| t.split(',').filter_map(|s| s.trim().parse::<i32>().ok()))
            .collect();

        // 查询分组名称
        let group_map: HashMap<i32, String> = if !group_ids.is_empty() {
            user_group::Entity::find()
                .filter(user_group::Column::Id.is_in(group_ids))
                .all(db)
                .await?
                .into_iter()
                .filter_map(|g| g.group_name.map(|n| (g.id, n)))
                .collect()
        } else {
            HashMap::new()
        };

        // 查询推荐人昵称
        let spread_map: HashMap<i32, String> = if !spread_uids.is_empty() {
            user::Entity::find()
                .filter(user::Column::Uid.is_in(spread_uids))
                .all(db)
                .await?
                .into_iter()
                .filter_map(|u| u.nickname.map(|n| (u.uid, n)))
                .collect()
        } else {
            HashMap::new()
        };

        // 查询标签名称
        let tag_map: HashMap<i32, String> = if !tag_ids.is_empty() {
            user_tag::Entity::find()
                .filter(user_tag::Column::Id.is_in(tag_ids))
                .all(db)
                .await?
                .into_iter()
                .filter_map(|t| t.name.map(|n| (t.id, n)))
                .collect()
        } else {
            HashMap::new()
        };

        // 转换为响应格式
        let list: Vec<UserResponse> = users
            .into_iter()
            .map(|u| Self::model_to_response_with_names(u, &group_map, &spread_map, &tag_map))
            .collect();

        Ok(CommonPage {
            list,
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 将Model转换为Response
    fn model_to_response(model: user::Model) -> UserResponse {
        UserResponse {
            uid: model.uid,
            account: model.account,
            real_name: model.real_name,
            birthday: model.birthday,
            card_id: model.card_id,
            mark: model.mark,
            partner_id: model.partner_id,
            group_id: model.group_id,
            group_name: None,
            tag_id: model.tag_id,
            tag_name: None,
            nickname: model.nickname,
            avatar: model.avatar,
            phone: model.phone,
            add_ip: model.add_ip,
            last_ip: model.last_ip,
            now_money: model.now_money,
            brokerage_price: model.brokerage_price,
            integral: model.integral,
            experience: model.experience,
            sign_num: model.sign_num,
            status: model.status.unwrap_or(0) != 0,
            level: model.level,
            spread_uid: model.spread_uid,
            spread_nickname: None,
            spread_time: model.spread_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            user_type: model.user_type,
            is_promoter: model.is_promoter.unwrap_or(0) != 0,
            pay_count: model.pay_count,
            spread_count: model.spread_count,
            addres: model.addres,
            adminid: model.adminid,
            login_type: model.login_type,
            create_time: model.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: model.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_login_time: model.last_login_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            path: model.path,
            subscribe: model.subscribe.unwrap_or(0) != 0,
            subscribe_time: model.subscribe_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            sex: model.sex,
            country: model.country,
            promoter_time: model.promoter_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            is_logoff: model.is_logoff != 0,
            logoff_time: model.logoff_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }

    /// 将Model转换为Response（带关联名称）
    fn model_to_response_with_names(
        model: user::Model,
        group_map: &HashMap<i32, String>,
        spread_map: &HashMap<i32, String>,
        tag_map: &HashMap<i32, String>,
    ) -> UserResponse {
        // 获取分组名称
        let group_name = model.group_id.as_ref()
            .and_then(|g| g.parse::<i32>().ok())
            .and_then(|id| group_map.get(&id).cloned());

        // 获取推荐人昵称
        let spread_nickname = model.spread_uid
            .filter(|&uid| uid > 0)
            .and_then(|uid| spread_map.get(&uid).cloned());

        // 获取标签名称
        let tag_name = model.tag_id.as_ref().map(|tags| {
            tags.split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .filter_map(|id| tag_map.get(&id))
                .cloned()
                .collect::<Vec<_>>()
                .join(",")
        }).filter(|s| !s.is_empty());

        UserResponse {
            uid: model.uid,
            account: model.account,
            real_name: model.real_name,
            birthday: model.birthday,
            card_id: model.card_id,
            mark: model.mark,
            partner_id: model.partner_id,
            group_id: model.group_id,
            group_name,
            tag_id: model.tag_id,
            tag_name,
            nickname: model.nickname,
            avatar: model.avatar,
            phone: model.phone,
            add_ip: model.add_ip,
            last_ip: model.last_ip,
            now_money: model.now_money,
            brokerage_price: model.brokerage_price,
            integral: model.integral,
            experience: model.experience,
            sign_num: model.sign_num,
            status: model.status.unwrap_or(0) != 0,
            level: model.level,
            spread_uid: model.spread_uid,
            spread_nickname,
            spread_time: model.spread_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            user_type: model.user_type,
            is_promoter: model.is_promoter.unwrap_or(0) != 0,
            pay_count: model.pay_count,
            spread_count: model.spread_count,
            addres: model.addres,
            adminid: model.adminid,
            login_type: model.login_type,
            create_time: model.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: model.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            last_login_time: model.last_login_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            path: model.path,
            subscribe: model.subscribe.unwrap_or(0) != 0,
            subscribe_time: model.subscribe_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            sex: model.sex,
            country: model.country,
            promoter_time: model.promoter_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            is_logoff: model.is_logoff != 0,
            logoff_time: model.logoff_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }

    /// 获取用户详情（Top数据）
    /// Java参考: UserServiceImpl.getTopDetail()
    pub async fn get_top_detail(db: &DatabaseConnection, uid: i32) -> Result<UserResponse> {
        let user = user::Entity::find()
            .filter(user::Column::Uid.eq(uid))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("用户不存在"))?;

        Ok(Self::model_to_response(user))
    }

    /// 根据UID获取用户信息（编辑用）
    /// Java参考: UserServiceImpl.getInfoByUid()
    pub async fn get_info_by_uid(db: &DatabaseConnection, uid: i32) -> Result<UserResponse> {
        let user = user::Entity::find()
            .filter(user::Column::Uid.eq(uid))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("用户不存在"))?;

        Ok(Self::model_to_response(user))
    }

    /// 更新用户信息
    /// Java参考: UserServiceImpl.updateUser()
    pub async fn update_user(
        db: &DatabaseConnection,
        uid: i32,
        request: &crate::dtos::user::UserUpdateRequest,
    ) -> Result<bool> {
        let user = user::Entity::find()
            .filter(user::Column::Uid.eq(uid))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("用户不存在"))?;

        let mut active: user::ActiveModel = user.into();

        if let Some(real_name) = &request.real_name {
            active.real_name = Set(Some(real_name.clone()));
        }
        if let Some(phone) = &request.phone {
            active.phone = Set(Some(phone.clone()));
        }
        if let Some(birthday) = &request.birthday {
            active.birthday = Set(Some(birthday.clone()));
        }
        if let Some(card_id) = &request.card_id {
            active.card_id = Set(Some(card_id.clone()));
        }
        if let Some(addres) = &request.addres {
            active.addres = Set(Some(addres.clone()));
        }
        if let Some(mark) = &request.mark {
            active.mark = Set(Some(mark.clone()));
        }
        if let Some(status) = request.status {
            active.status = Set(Some(status as i16));
        }

        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Ok(true)
    }

    /// 根据条件获取会员对应信息列表
    /// Java参考: UserServiceImpl.getInfoByCondition()
    /// type: 0=消费记录，1=积分明细，2=签到记录，3=持有优惠券，4=余额变动，5=好友关系
    pub async fn get_info_by_condition(
        db: &DatabaseConnection,
        user_id: i32,
        condition_type: i32,
        page_param: &PageParamRequest,
    ) -> Result<UserConditionResponse> {
        // 先验证用户是否存在
        let _user = user::Entity::find()
            .filter(user::Column::Uid.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("用户不存在"))?;

        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        match condition_type {
            0 => Self::get_order_records(db, user_id, page, limit).await,
            1 => Self::get_integral_records(db, user_id, page, limit).await,
            2 => Self::get_sign_records(db, user_id, page, limit).await,
            3 => Self::get_coupon_records(db, user_id, page, limit).await,
            4 => Self::get_bill_records(db, user_id, page, limit).await,
            5 => Self::get_user_relations(db, user_id).await,
            _ => Err(Error::string("无效的查询类型，type必须在0-5之间")),
        }
    }

    /// 获取消费记录 (type=0)
    async fn get_order_records(
        db: &DatabaseConnection,
        user_id: i32,
        page: i32,
        limit: i32,
    ) -> Result<UserConditionResponse> {
        let query = store_order::Entity::find()
            .filter(store_order::Column::Uid.eq(user_id))
            .filter(store_order::Column::Paid.eq(1))
            .filter(store_order::Column::IsDel.eq(0))
            .order_by_desc(store_order::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let orders = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<OrderRecordResponse> = orders
            .into_iter()
            .map(|o| OrderRecordResponse {
                id: o.id,
                order_id: o.order_id,
                real_name: o.real_name,
                user_phone: o.user_phone,
                pay_price: o.pay_price,
                pay_type: o.pay_type,
                pay_time: o.pay_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                status: o.status,
                create_time: o.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            })
            .collect();

        Ok(UserConditionResponse {
            list: serde_json::to_value(list).unwrap_or_default(),
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 获取积分明细 (type=1)
    async fn get_integral_records(
        db: &DatabaseConnection,
        user_id: i32,
        page: i32,
        limit: i32,
    ) -> Result<UserConditionResponse> {
        let query = user_integral_record::Entity::find()
            .filter(user_integral_record::Column::Uid.eq(user_id))
            .order_by_desc(user_integral_record::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<IntegralRecordResponse> = records
            .into_iter()
            .map(|r| IntegralRecordResponse {
                id: r.id,
                uid: r.uid,
                link_id: r.link_id,
                link_type: r.link_type,
                record_type: r.r#type,
                title: r.title,
                integral: r.integral,
                balance: r.balance,
                mark: r.mark,
                status: r.status,
                create_time: r.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            })
            .collect();

        Ok(UserConditionResponse {
            list: serde_json::to_value(list).unwrap_or_default(),
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 获取签到记录 (type=2)
    async fn get_sign_records(
        db: &DatabaseConnection,
        user_id: i32,
        page: i32,
        limit: i32,
    ) -> Result<UserConditionResponse> {
        let query = user_sign::Entity::find()
            .filter(user_sign::Column::Uid.eq(user_id))
            .order_by_desc(user_sign::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<SignRecordResponse> = records
            .into_iter()
            .map(|r| SignRecordResponse {
                id: r.id,
                uid: r.uid,
                title: r.title,
                number: r.number,
                balance: r.balance,
                sign_type: r.r#type,
                create_day: r.create_day.format("%Y-%m-%d").to_string(),
                create_time: r.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect();

        Ok(UserConditionResponse {
            list: serde_json::to_value(list).unwrap_or_default(),
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 获取持有优惠券 (type=3)
    async fn get_coupon_records(
        db: &DatabaseConnection,
        user_id: i32,
        page: i32,
        limit: i32,
    ) -> Result<UserConditionResponse> {
        let query = store_coupon_user::Entity::find()
            .filter(store_coupon_user::Column::Uid.eq(user_id))
            .order_by_desc(store_coupon_user::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<CouponUserResponse> = records
            .into_iter()
            .map(|r| CouponUserResponse {
                id: r.id,
                coupon_id: r.coupon_id,
                uid: r.uid,
                name: r.name,
                money: r.money,
                min_price: r.min_price,
                coupon_type: r.r#type,
                status: r.status,
                start_time: r.start_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                end_time: r.end_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                use_time: r.use_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                create_time: r.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            })
            .collect();

        Ok(UserConditionResponse {
            list: serde_json::to_value(list).unwrap_or_default(),
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 获取余额变动 (type=4)
    async fn get_bill_records(
        db: &DatabaseConnection,
        user_id: i32,
        page: i32,
        limit: i32,
    ) -> Result<UserConditionResponse> {
        let query = user_bill::Entity::find()
            .filter(user_bill::Column::Uid.eq(user_id))
            .filter(user_bill::Column::Category.eq("now_money"))
            .order_by_desc(user_bill::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<BillRecordResponse> = records
            .into_iter()
            .map(|r| BillRecordResponse {
                id: r.id,
                uid: r.uid,
                link_id: r.link_id,
                pm: r.pm,
                title: r.title,
                category: r.category,
                bill_type: r.r#type,
                number: r.number,
                balance: r.balance,
                mark: r.mark,
                status: r.status,
                create_time: r.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            })
            .collect();

        Ok(UserConditionResponse {
            list: serde_json::to_value(list).unwrap_or_default(),
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 获取好友关系 (type=5)
    /// 获取用户的上级推广人链（最多两级）
    async fn get_user_relations(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<UserConditionResponse> {
        let current_user = user::Entity::find()
            .filter(user::Column::Uid.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("用户不存在"))?;

        let mut relations: Vec<UserRelationResponse> = Vec::new();

        // 获取一级推广人
        if let Some(spread_uid) = current_user.spread_uid {
            if spread_uid > 0 {
                if let Some(sp_user1) = user::Entity::find()
                    .filter(user::Column::Uid.eq(spread_uid))
                    .one(db)
                    .await?
                {
                    relations.push(UserRelationResponse {
                        uid: sp_user1.uid,
                        nickname: sp_user1.nickname.clone(),
                        avatar: sp_user1.avatar.clone(),
                        phone: sp_user1.phone.clone(),
                        spread_uid: sp_user1.spread_uid,
                        spread_time: sp_user1.spread_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                        level: 1,
                    });

                    // 获取二级推广人
                    if let Some(spread_uid2) = sp_user1.spread_uid {
                        if spread_uid2 > 0 {
                            if let Some(sp_user2) = user::Entity::find()
                                .filter(user::Column::Uid.eq(spread_uid2))
                                .one(db)
                                .await?
                            {
                                relations.push(UserRelationResponse {
                                    uid: sp_user2.uid,
                                    nickname: sp_user2.nickname,
                                    avatar: sp_user2.avatar,
                                    phone: sp_user2.phone,
                                    spread_uid: sp_user2.spread_uid,
                                    spread_time: sp_user2.spread_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                                    level: 2,
                                });
                            }
                        }
                    }
                }
            }
        }

        let total = relations.len() as i64;
        Ok(UserConditionResponse {
            list: serde_json::to_value(relations).unwrap_or_default(),
            total,
            page_number: 1,
            page_size: total as i32,
        })
    }

    /// 更新用户会员等级
    /// Java参考: UserServiceImpl.updateUserLevel()
    pub async fn update_user_level(
        db: &DatabaseConnection,
        uid: i32,
        level_id: i32,
    ) -> Result<bool> {
        let user = user::Entity::find()
            .filter(user::Column::Uid.eq(uid))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("用户不存在"))?;

        let mut active: user::ActiveModel = user.into();
        active.level = Set(Some(level_id as i16));
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Ok(true)
    }
}
