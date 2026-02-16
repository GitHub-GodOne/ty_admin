/// 分销管理服务层
///
/// 处理分销相关的业务逻辑
/// Java代码参考: RetailShopServiceImpl, UserServiceImpl
use sea_orm::*;
use sea_orm::sea_query::Expr;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::retail_shop::*;
use crate::models::_entities::{
    user, user_brokerage_record, user_extract, store_order,
};
use crate::services::system_config_service::SystemConfigService;

pub struct RetailShopService;

impl RetailShopService {
    /// 获取分销员列表
    ///
    /// Java: RetailShopServiceImpl.getSpreadPeopleList()
    pub async fn get_spread_people_list(
        db: &DatabaseConnection,
        request: &RetailShopListRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SpreadUserResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        // 查询推广员列表
        let mut query = user::Entity::find();
        query = query.filter(user::Column::IsPromoter.eq(1i16));

        // 关键字搜索
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(Expr::cust(&format!("uid::text = '{}'", keywords.replace('\'', "''"))))
                        .add(user::Column::Nickname.contains(keywords))
                        .add(user::Column::Phone.contains(keywords))
                );
            }
        }

        // 时间筛选（成为分销员时间）
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                let dates: Vec<&str> = date_limit.split(',').collect();
                if dates.len() == 2 {
                    let start = dates[0].trim();
                    let end = dates[1].trim();
                    if !start.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("promoter_time >= '{} 00:00:00'::timestamp", start))
                        );
                    }
                    if !end.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("promoter_time <= '{} 23:59:59'::timestamp", end))
                        );
                    }
                }
            }
        }

        query = query.order_by_desc(user::Column::Uid);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let users = paginator.fetch_page((page - 1) as u64).await?;

        // 批量获取上级推广员信息
        let spread_uids: Vec<i32> = users.iter()
            .filter_map(|u| u.spread_uid)
            .filter(|&uid| uid > 0)
            .collect();
        let spread_users: HashMap<i32, user::Model> = if !spread_uids.is_empty() {
            user::Entity::find()
                .filter(user::Column::Uid.is_in(spread_uids))
                .all(db)
                .await?
                .into_iter()
                .map(|u| (u.uid, u))
                .collect()
        } else {
            HashMap::new()
        };

        let mut list = Vec::new();
        for u in users {
            let uid = u.uid;

            // 上级推广员名称
            let spread_nickname = if let Some(spread_uid) = u.spread_uid {
                if spread_uid > 0 {
                    spread_users.get(&spread_uid)
                        .and_then(|su| su.nickname.clone())
                        .unwrap_or_else(|| "--".to_string())
                } else {
                    "无".to_string()
                }
            } else {
                "无".to_string()
            };

            // 获取佣金记录（已完成的订单佣金）
            let records = user_brokerage_record::Entity::find()
                .filter(user_brokerage_record::Column::Uid.eq(uid))
                .filter(user_brokerage_record::Column::LinkType.eq("order"))
                .filter(user_brokerage_record::Column::Type.eq(1))
                .filter(user_brokerage_record::Column::Status.eq(3))
                .all(db)
                .await?;
// __CONTINUE_SVC_2__

            let (spread_order_num, spread_order_total_price, total_brokerage_price,
                 extract_count_price, extract_count_num, freeze_brokerage_price) = if records.is_empty() {
                (0, Decimal::ZERO, Decimal::ZERO, Decimal::ZERO, 0, Decimal::ZERO)
            } else {
                // 推广订单数
                let order_num = records.len() as i32;
                // 佣金总金额
                let total_brokerage: Decimal = records.iter()
                    .map(|r| r.price)
                    .sum();
                // 推广订单额 - 根据订单号查询订单总价
                let order_nos: Vec<String> = records.iter()
                    .map(|r| r.link_id.clone())
                    .collect();
                let order_total = Self::get_spread_order_total_price(db, &order_nos).await?;

                // 已提现金额和次数
                let (ext_price, ext_num) = Self::get_user_extract_info(db, uid).await?;

                // 冻结中佣金
                let freeze = Self::get_freeze_price(db, uid).await?;

                (order_num, order_total, total_brokerage, ext_price, ext_num, freeze)
            };

            list.push(SpreadUserResponse {
                uid,
                real_name: u.real_name,
                nickname: u.nickname,
                avatar: u.avatar,
                phone: u.phone,
                brokerage_price: u.brokerage_price,
                spread_uid: u.spread_uid,
                spread_nickname,
                pay_count: u.pay_count,
                spread_count: u.spread_count,
                spread_order_num,
                spread_order_total_price,
                total_brokerage_price,
                extract_count_price,
                extract_count_num,
                freeze_brokerage_price,
                promoter_time: u.promoter_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            });
        }

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 根据条件获取推广人列表
    ///
    /// Java: UserServiceImpl.getUserListBySpreadLevel()
    pub async fn get_user_list_by_spread_level(
        db: &DatabaseConnection,
        request: &RetailShopStairUserRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SpreadLevelUserResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        match request.spread_type {
            1 => {
                // 一级推广人：spread_uid = request.uid
                Self::get_first_spread_user_list(db, request, page, limit).await
            }
            2 => {
                // 二级推广人
                Self::get_second_spread_user_list(db, request, page, limit).await
            }
            _ => {
                // 全部推广人
                Self::get_all_spread_user_list(db, request, page, limit).await
            }
        }
    }

    /// 根据条件获取推广订单列表
    ///
    /// Java: UserServiceImpl.getOrderListBySpreadLevel()
    pub async fn get_order_list_by_spread_level(
        db: &DatabaseConnection,
        request: &RetailShopStairUserRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SpreadOrderResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        // 查询佣金记录
        let mut query = user_brokerage_record::Entity::find();
        query = query.filter(user_brokerage_record::Column::Uid.eq(request.uid));
        query = query.filter(user_brokerage_record::Column::LinkType.eq("order"));
        query = query.filter(user_brokerage_record::Column::Status.eq(3));

        // 按推广等级筛选
        if request.spread_type == 1 {
            query = query.filter(user_brokerage_record::Column::BrokerageLevel.eq(1));
        } else if request.spread_type == 2 {
            query = query.filter(user_brokerage_record::Column::BrokerageLevel.eq(2));
        }

        // 关键字搜索（搜索订单号）
        if let Some(nick_name) = &request.nick_name {
            if !nick_name.is_empty() {
                query = query.filter(user_brokerage_record::Column::LinkId.contains(nick_name));
            }
        }

        // 时间筛选
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                let dates: Vec<&str> = date_limit.split(',').collect();
                if dates.len() == 2 {
                    let start = dates[0].trim();
                    let end = dates[1].trim();
                    if !start.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("update_time >= '{} 00:00:00'::timestamp", start))
                        );
                    }
                    if !end.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("update_time <= '{} 23:59:59'::timestamp", end))
                        );
                    }
                }
            }
        }

        query = query.order_by_desc(user_brokerage_record::Column::UpdateTime);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        // 批量查询订单信息
        let order_ids: Vec<String> = records.iter().map(|r| r.link_id.clone()).collect();
        let orders: HashMap<String, store_order::Model> = if !order_ids.is_empty() {
            store_order::Entity::find()
                .filter(store_order::Column::OrderId.is_in(order_ids))
                .all(db)
                .await?
                .into_iter()
                .map(|o| (o.order_id.clone(), o))
                .collect()
        } else {
            HashMap::new()
        };
// __CONTINUE_SVC_3__

        let list: Vec<SpreadOrderResponse> = records.iter().map(|r| {
            let order = orders.get(&r.link_id);
            SpreadOrderResponse {
                id: order.map(|o| o.id).unwrap_or(0),
                order_id: order.map(|o| o.order_id.clone()).unwrap_or_default(),
                real_name: order.map(|o| o.real_name.clone()).unwrap_or_default(),
                user_phone: order.map(|o| o.user_phone.clone()).unwrap_or_default(),
                price: r.price,
                update_time: r.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            }
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 清除上级推广人
    ///
    /// Java: UserServiceImpl.clearSpread()
    pub async fn clear_spread(db: &DatabaseConnection, user_id: i32) -> Result<bool, DbErr> {
        // 查询当前用户
        let user_opt = user::Entity::find_by_id(user_id).one(db).await?;
        let user_model = match user_opt {
            Some(u) => u,
            None => return Ok(false),
        };

        let old_spread_uid = user_model.spread_uid.unwrap_or(0);

        // 更新用户：清除推广关系
        let now = chrono::Local::now().naive_local();
        let mut active: user::ActiveModel = user_model.into();
        active.path = Set("/0/".to_string());
        active.spread_uid = Set(Some(0));
        active.spread_time = Set(None);
        active.update_time = Set(now);
        active.update(db).await?;

        // 如果有上级推广人，减少其推广人数
        if old_spread_uid > 0 {
            user::Entity::update_many()
                .filter(user::Column::Uid.eq(old_spread_uid))
                .col_expr(user::Column::SpreadCount, Expr::col(user::Column::SpreadCount).sub(1))
                .exec(db)
                .await?;
        }

        Ok(true)
    }

    /// 获取分销配置信息
    ///
    /// Java: RetailShopServiceImpl.getManageInfo()
    pub async fn get_manage_info(db: &DatabaseConnection) -> Result<RetailShopConfigRequest, DbErr> {
        let get_int = |val: String| -> i32 {
            val.parse::<i32>().unwrap_or(0)
        };

        let brokerage_func_status = get_int(SystemConfigService::get_value_by_key(db, "brokerage_func_status").await.unwrap_or_default());
        let store_brokerage_ratio = get_int(SystemConfigService::get_value_by_key(db, "store_brokerage_ratio").await.unwrap_or_default());
        let store_brokerage_two = get_int(SystemConfigService::get_value_by_key(db, "store_brokerage_two").await.unwrap_or_default());
        let extract_time = get_int(SystemConfigService::get_value_by_key(db, "extract_time").await.unwrap_or_default());
        let store_brokerage_quota = get_int(SystemConfigService::get_value_by_key(db, "store_brokerage_quota").await.unwrap_or_default());
        let store_brokerage_is_bubble = get_int(SystemConfigService::get_value_by_key(db, "store_brokerage_is_bubble").await.unwrap_or_default());
        let brokerage_bindind = get_int(SystemConfigService::get_value_by_key(db, "brokerage_bindind").await.unwrap_or_default());
// __CONTINUE_SVC_4__

        let user_extract_min_price_str = SystemConfigService::get_value_by_key(db, "user_extract_min_price").await
            .unwrap_or_default();
        let user_extract_min_price = user_extract_min_price_str.parse::<Decimal>().unwrap_or(Decimal::ZERO);

        let user_extract_bank = SystemConfigService::get_value_by_key(db, "user_extract_bank").await
            .unwrap_or_default()
            .replace("\\n", "\n");

        Ok(RetailShopConfigRequest {
            brokerage_func_status,
            store_brokerage_quota,
            store_brokerage_ratio,
            store_brokerage_two,
            brokerage_bindind,
            user_extract_min_price,
            user_extract_bank,
            extract_time,
            store_brokerage_is_bubble,
        })
    }

    /// 保存分销配置信息
    ///
    /// Java: RetailShopServiceImpl.setManageInfo()
    pub async fn set_manage_info(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        request: &RetailShopConfigRequest,
    ) -> Result<bool, DbErr> {
        // 返佣比例之和不能超过100%
        let ratio = request.store_brokerage_two + request.store_brokerage_ratio;
        if ratio > 100 || ratio < 0 {
            return Ok(false);
        }

        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "brokerage_func_status", &request.brokerage_func_status.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "store_brokerage_ratio", &request.store_brokerage_ratio.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "store_brokerage_two", &request.store_brokerage_two.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "user_extract_min_price", &request.user_extract_min_price.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "user_extract_bank", &request.user_extract_bank).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "extract_time", &request.extract_time.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "brokerage_bindind", &request.brokerage_bindind.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "store_brokerage_quota", &request.store_brokerage_quota.to_string()).await;
        let _ = SystemConfigService::update_or_save_value_by_name(db, redis, "store_brokerage_is_bubble", &request.store_brokerage_is_bubble.to_string()).await;

        Ok(true)
    }

    // ==================== 私有辅助方法 ====================

    /// 一级推广人列表
    async fn get_first_spread_user_list(
        db: &DatabaseConnection,
        request: &RetailShopStairUserRequest,
        page: i32,
        limit: i32,
    ) -> Result<CommonPage<SpreadLevelUserResponse>, DbErr> {
        let mut query = user::Entity::find();
        query = query.filter(user::Column::SpreadUid.eq(request.uid));

        if let Some(nick_name) = &request.nick_name {
            if !nick_name.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(user::Column::Nickname.contains(nick_name))
                        .add(Expr::cust(&format!("uid::text = '{}'", nick_name.replace('\'', "''"))))
                        .add(user::Column::Phone.eq(nick_name.as_str()))
                );
            }
        }
// __CONTINUE_SVC_5__

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let users = paginator.fetch_page((page - 1) as u64).await?;

        let list = users.into_iter().map(|u| Self::user_to_spread_level_response(u)).collect();
        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 二级推广人列表
    async fn get_second_spread_user_list(
        db: &DatabaseConnection,
        request: &RetailShopStairUserRequest,
        page: i32,
        limit: i32,
    ) -> Result<CommonPage<SpreadLevelUserResponse>, DbErr> {
        // 先获取一级推广员的uid列表
        let first_users = user::Entity::find()
            .filter(user::Column::SpreadUid.eq(request.uid))
            .all(db)
            .await?;

        if first_users.is_empty() {
            return Ok(CommonPage::new(vec![], 0, page, limit));
        }

        let first_uids: Vec<i32> = first_users.iter().map(|u| u.uid).collect();

        let mut query = user::Entity::find();
        query = query.filter(user::Column::SpreadUid.is_in(first_uids));

        if let Some(nick_name) = &request.nick_name {
            if !nick_name.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(user::Column::Nickname.contains(nick_name))
                        .add(Expr::cust(&format!("uid::text = '{}'", nick_name.replace('\'', "''"))))
                        .add(user::Column::Phone.eq(nick_name.as_str()))
                );
            }
        }

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let users = paginator.fetch_page((page - 1) as u64).await?;

        let list = users.into_iter().map(|u| Self::user_to_spread_level_response(u)).collect();
        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 全部推广人列表（一级+二级）
    async fn get_all_spread_user_list(
        db: &DatabaseConnection,
        request: &RetailShopStairUserRequest,
        page: i32,
        limit: i32,
    ) -> Result<CommonPage<SpreadLevelUserResponse>, DbErr> {
        // 获取所有一级推广员
        let first_users = user::Entity::find()
            .filter(user::Column::SpreadUid.eq(request.uid))
            .all(db)
            .await?;

        if first_users.is_empty() {
            return Ok(CommonPage::new(vec![], 0, page, limit));
        }

        let first_uids: Vec<i32> = first_users.iter().map(|u| u.uid).collect();

        // 获取二级推广员
        let second_users = user::Entity::find()
            .filter(user::Column::SpreadUid.is_in(first_uids.clone()))
            .all(db)
            .await?;

        // 合并所有uid
        let mut all_uids: Vec<i32> = first_uids;
        all_uids.extend(second_users.iter().map(|u| u.uid));
        all_uids.sort();
        all_uids.dedup();
// __CONTINUE_SVC_6__

        if all_uids.is_empty() {
            return Ok(CommonPage::new(vec![], 0, page, limit));
        }

        let mut query = user::Entity::find();
        query = query.filter(user::Column::Uid.is_in(all_uids));

        if let Some(nick_name) = &request.nick_name {
            if !nick_name.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(user::Column::Nickname.contains(nick_name))
                        .add(Expr::cust(&format!("uid::text = '{}'", nick_name.replace('\'', "''"))))
                        .add(user::Column::Phone.eq(nick_name.as_str()))
                );
            }
        }

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let users = paginator.fetch_page((page - 1) as u64).await?;

        let list = users.into_iter().map(|u| Self::user_to_spread_level_response(u)).collect();
        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 根据订单号列表获取推广订单总价
    async fn get_spread_order_total_price(
        db: &DatabaseConnection,
        order_nos: &[String],
    ) -> Result<Decimal, DbErr> {
        if order_nos.is_empty() {
            return Ok(Decimal::ZERO);
        }
        let orders = store_order::Entity::find()
            .filter(store_order::Column::OrderId.is_in(order_nos.to_vec()))
            .all(db)
            .await?;
        let total: Decimal = orders.iter().map(|o| o.pay_price).sum();
        Ok(total)
    }

    /// 获取用户提现信息（已提现金额、提现次数）
    async fn get_user_extract_info(
        db: &DatabaseConnection,
        uid: i32,
    ) -> Result<(Decimal, i32), DbErr> {
        let extracts = user_extract::Entity::find()
            .filter(user_extract::Column::Uid.eq(uid as u32))
            .filter(user_extract::Column::Status.eq(1)) // status=1 已提现
            .all(db)
            .await?;

        let count = extracts.len() as i32;
        let total: Decimal = extracts.iter()
            .filter_map(|e| e.extract_price)
            .sum();
        Ok((total, count))
    }

    /// 获取冻结中佣金
    async fn get_freeze_price(db: &DatabaseConnection, uid: i32) -> Result<Decimal, DbErr> {
        let records = user_brokerage_record::Entity::find()
            .filter(user_brokerage_record::Column::Uid.eq(uid))
            .filter(user_brokerage_record::Column::LinkType.eq("order"))
            .filter(user_brokerage_record::Column::Status.eq(1)) // status=1 冻结中
            .all(db)
            .await?;
        let total: Decimal = records.iter().map(|r| r.price).sum();
        Ok(total)
    }

    /// 用户模型转推广等级响应
    fn user_to_spread_level_response(u: user::Model) -> SpreadLevelUserResponse {
        SpreadLevelUserResponse {
            uid: u.uid,
            avatar: u.avatar,
            nickname: u.nickname,
            is_promoter: u.is_promoter.unwrap_or(0) != 0,
            spread_count: u.spread_count,
            pay_count: u.pay_count,
        }
    }
}