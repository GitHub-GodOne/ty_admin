/// 用户充值服务
///
/// Java参考: UserRechargeServiceImpl.getList(), UserRechargeServiceImpl.getBalanceList()
use sea_orm::*;
use sea_orm::sea_query::Expr;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::user_recharge::*;
use crate::models::_entities::{user, user_recharge};

pub struct UserRechargeService;

impl UserRechargeService {
    /// 充值记录列表
    ///
    /// Java: UserRechargeServiceImpl.getList()
    /// SQL: SELECT * FROM user_recharge WHERE paid=1 [AND conditions] ORDER BY id DESC
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &UserRechargeSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserRechargeResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = user_recharge::Entity::find();

        // 固定条件: paid = 1 (只查已支付)
        query = query.filter(user_recharge::Column::Paid.eq(1));

        // uid筛选
        if let Some(uid) = request.uid {
            query = query.filter(user_recharge::Column::Uid.eq(uid));
        }

        // 关键字搜索（订单号模糊匹配）
        // Java: LIKE '%keywords%' on order_id
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(user_recharge::Column::OrderId.contains(keywords));
            }
        }

        // 时间区间筛选
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                let dates: Vec<&str> = date_limit.split(',').collect();
                if dates.len() == 2 {
                    let start = dates[0].trim();
                    let end = dates[1].trim();
                    if !start.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("create_time >= '{} 00:00:00'::timestamp", start.replace('\'', "''")))
                        );
                    }
                    if !end.is_empty() {
                        query = query.filter(
                            Expr::cust(&format!("create_time <= '{} 23:59:59'::timestamp", end.replace('\'', "''")))
                        );
                    }
                }
            }
        }

        // 排序: id DESC
        query = query.order_by_desc(user_recharge::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        // 批量查询用户头像和昵称
        let uids: Vec<i32> = records.iter().filter_map(|r| r.uid).collect();
        let user_map: HashMap<i32, (Option<String>, Option<String>)> = if !uids.is_empty() {
            user::Entity::find()
                .filter(user::Column::Uid.is_in(uids))
                .all(db)
                .await?
                .into_iter()
                .map(|u| (u.uid, (u.avatar.clone(), u.nickname.clone())))
                .collect()
        } else {
            HashMap::new()
        };

        let list: Vec<UserRechargeResponse> = records.iter().map(|r| {
            let (avatar, nickname) = r.uid
                .and_then(|uid| user_map.get(&uid))
                .cloned()
                .unwrap_or((None, None));
            UserRechargeResponse {
                id: r.id,
                uid: r.uid,
                order_id: r.order_id.clone(),
                price: r.price,
                give_price: r.give_price,
                recharge_type: r.recharge_type.clone(),
                paid: r.paid,
                pay_time: r.pay_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                create_time: r.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                refund_price: r.refund_price,
                avatar,
                nickname,
            }
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 充值余额统计
    ///
    /// Java: UserRechargeServiceImpl.getBalanceList()
    /// 返回: routine(小程序), weChat(公众号/type=public), total(全部), refund(退款), other(其他=total-routine-weChat)
    pub async fn get_balance_list(
        db: &DatabaseConnection,
    ) -> Result<BalanceResponse, DbErr> {
        let mut result = HashMap::new();

        // routine: 小程序充值 (recharge_type = 'routine')
        let routine = Self::get_sum_by_type(db, Some("routine")).await?;
        result.insert("routine".to_string(), routine);

        // weChat: 公众号充值 (recharge_type = 'public')
        let wechat = Self::get_sum_by_type(db, Some("public")).await?;
        result.insert("weChat".to_string(), wechat);

        // total: 全部充值
        let total = Self::get_sum_by_type(db, None).await?;
        result.insert("total".to_string(), total);

        // refund: 退款总额
        let refund = Self::get_sum_by_refund(db).await?;
        result.insert("refund".to_string(), refund);

        // other: 其他 = total - routine - weChat
        let other = total - routine - wechat;
        result.insert("other".to_string(), other);

        Ok(result)
    }

    /// 按类型统计充值金额
    ///
    /// Java: UserRechargeMapper.getSumByType()
    /// SQL: SELECT COALESCE(SUM(price), 0) FROM user_recharge WHERE paid = 1 [AND recharge_type = type]
    async fn get_sum_by_type(
        db: &DatabaseConnection,
        recharge_type: Option<&str>,
    ) -> Result<Decimal, DbErr> {
        use sea_orm::sea_query::{Alias, Query, PostgresQueryBuilder};
        use sea_orm::ConnectionTrait;

        let mut query = Query::select();
        query.expr_as(
            Expr::cust("COALESCE(SUM(price), 0)"),
            Alias::new("total"),
        );
        query.from(Alias::new("ty_user_recharge"));
        query.and_where(Expr::col(Alias::new("paid")).eq(1));

        if let Some(rt) = recharge_type {
            query.and_where(Expr::col(Alias::new("recharge_type")).eq(rt));
        }

        let sql = query.to_string(PostgresQueryBuilder);
        let result = db.query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            sql,
        )).await?;

        match result {
            Some(row) => {
                let total: Decimal = row.try_get_by_index(0).unwrap_or(Decimal::ZERO);
                Ok(total)
            }
            None => Ok(Decimal::ZERO),
        }
    }

    /// 统计退款金额
    ///
    /// Java: UserRechargeMapper.getSumByRefund()
    /// SQL: SELECT COALESCE(SUM(refund_price), 0) FROM user_recharge WHERE refund_price > 0 AND paid = 1
    async fn get_sum_by_refund(db: &DatabaseConnection) -> Result<Decimal, DbErr> {
        use sea_orm::sea_query::{Alias, Query, PostgresQueryBuilder};
        use sea_orm::ConnectionTrait;

        let mut query = Query::select();
        query.expr_as(
            Expr::cust("COALESCE(SUM(refund_price), 0)"),
            Alias::new("total"),
        );
        query.from(Alias::new("ty_user_recharge"));
        query.and_where(Expr::col(Alias::new("paid")).eq(1));
        query.and_where(Expr::cust("refund_price > 0"));

        let sql = query.to_string(PostgresQueryBuilder);
        let result = db.query_one(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            sql,
        )).await?;

        match result {
            Some(row) => {
                let total: Decimal = row.try_get_by_index(0).unwrap_or(Decimal::ZERO);
                Ok(total)
            }
            None => Ok(Decimal::ZERO),
        }
    }
}
