/// 资金监控服务
///
/// Java参考: UserBillServiceImpl.fundMonitoring(), UserFundsMonitorServiceImpl.getBrokerageRecord()
use sea_orm::*;
use sea_orm::sea_query::Expr;
use std::collections::HashMap;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::funds_monitor::*;
use crate::models::_entities::{user, user_bill, user_brokerage_record};

/// 佣金记录常量
/// Java: BrokerageRecordConstants
const BROKERAGE_RECORD_TYPE_ADD: i32 = 1;
const BROKERAGE_RECORD_TYPE_SUB: i32 = 2;
const BROKERAGE_RECORD_STATUS_COMPLETE: i32 = 3;
const BROKERAGE_RECORD_STATUS_WITHDRAW: i32 = 5;
const BROKERAGE_RECORD_LINK_TYPE_ORDER: &str = "order";
const BROKERAGE_RECORD_LINK_TYPE_WITHDRAW: &str = "withdraw";
const BROKERAGE_RECORD_LINK_TYPE_YUE: &str = "yue";

pub struct FundsMonitorService;

impl FundsMonitorService {
    /// 资金监控列表
    ///
    /// Java: UserBillServiceImpl.fundMonitoring()
    /// SQL: SELECT b.*, u.nickname FROM user_bill b LEFT JOIN user u ON b.uid = u.uid
    ///      WHERE b.category = 'now_money' [AND conditions] ORDER BY b.id DESC
    pub async fn fund_monitoring(
        db: &DatabaseConnection,
        request: &FundsMonitorRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<MonitorResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = user_bill::Entity::find();

        // 固定条件: category = 'now_money'
        query = query.filter(user_bill::Column::Category.eq("now_money"));

        // 关键字搜索（uid或昵称）
        // Java: LIKE '%keywords%' on uid and nickname (via LEFT JOIN)
        // 由于SeaORM不方便做LEFT JOIN LIKE，这里先按uid精确匹配或模糊查询
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                // 先查询匹配昵称的用户uid列表
                let matching_users = user::Entity::find()
                    .filter(user::Column::Nickname.contains(keywords))
                    .all(db)
                    .await?;
                let matching_uids: Vec<i32> = matching_users.iter().map(|u| u.uid).collect();

                // uid精确匹配 OR 昵称匹配的uid列表
                let uid_parsed = keywords.parse::<i32>().ok();
                let mut condition = Condition::any();
                if let Some(uid_val) = uid_parsed {
                    condition = condition.add(user_bill::Column::Uid.eq(uid_val));
                }
                if !matching_uids.is_empty() {
                    condition = condition.add(user_bill::Column::Uid.is_in(matching_uids.clone()));
                }
                // 如果既不是uid也没有匹配的昵称，添加一个不可能的条件
                if uid_parsed.is_none() && matching_uids.is_empty() {
                    // 没有匹配结果时，用nickname模糊匹配uid（不会匹配到任何记录）
                    condition = condition.add(user_bill::Column::Uid.eq(0i32));
                }
                query = query.filter(condition);
            }
        }
        // title类型筛选
        // Java: recharge→充值支付, admin→后台操作, productRefund→商品退款, payProduct→购买商品
        if let Some(title) = &request.title {
            if !title.is_empty() {
                let title_value = match title.as_str() {
                    "recharge" => Some("充值支付"),
                    "admin" => Some("后台操作"),
                    "productRefund" => Some("商品退款"),
                    "payProduct" => Some("购买商品"),
                    _ => None,
                };
                if let Some(tv) = title_value {
                    query = query.filter(user_bill::Column::Title.eq(tv));
                }
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
        query = query.order_by_desc(user_bill::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let bills = paginator.fetch_page((page - 1) as u64).await?;

        // 批量查询用户昵称
        let uids: Vec<i32> = bills.iter().map(|b| b.uid).collect();
        let user_map: HashMap<i32, String> = if !uids.is_empty() {
            user::Entity::find()
                .filter(user::Column::Uid.is_in(uids))
                .all(db)
                .await?
                .into_iter()
                .map(|u| (u.uid, u.nickname.unwrap_or_default()))
                .collect()
        } else {
            HashMap::new()
        };

        let list: Vec<MonitorResponse> = bills.iter().map(|b| {
            let nick_name = user_map.get(&b.uid).cloned();
            MonitorResponse {
                id: b.id,
                uid: b.uid,
                pm: b.pm,
                title: b.title.clone(),
                number: b.number,
                mark: b.mark.clone(),
                create_time: b.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                nick_name,
            }
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
    /// 佣金记录列表
    ///
    /// Java: UserFundsMonitorServiceImpl.getBrokerageRecord()
    /// → UserBrokerageRecordServiceImpl.getAdminList()
    pub async fn get_brokerage_record(
        db: &DatabaseConnection,
        request: &BrokerageRecordRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<BrokerageRecordResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = user_brokerage_record::Entity::find();

        // 按type筛选
        // Java: UserBrokerageRecordServiceImpl.getAdminList()
        match request.record_type {
            // type=1: 订单返佣 (link_type=order, status=complete(3), type=add(1))
            Some(1) => {
                query = query
                    .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_ORDER))
                    .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_COMPLETE))
                    .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_ADD));
            }
            // type=2: 申请提现 (link_type=withdraw, status=withdraw(5), type=sub(2))
            Some(2) => {
                query = query
                    .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_WITHDRAW))
                    .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_WITHDRAW))
                    .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_SUB));
            }
            // type=3: 提现失败 (link_type=withdraw, status=complete(3), type=add(1))
            Some(3) => {
                query = query
                    .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_WITHDRAW))
                    .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_COMPLETE))
                    .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_ADD));
            }
            // type=4: 提现成功 (link_type=withdraw, status=complete(3), type=sub(2))
            Some(4) => {
                query = query
                    .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_WITHDRAW))
                    .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_COMPLETE))
                    .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_SUB));
            }
            // type=5: 佣金转余额 (link_type=yue, status=complete(3), type=sub(2))
            Some(5) => {
                query = query
                    .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_YUE))
                    .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_COMPLETE))
                    .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_SUB));
            }
            // 默认: status IN (complete(3), withdraw(5))
            _ => {
                query = query.filter(
                    user_brokerage_record::Column::Status.is_in([
                        BROKERAGE_RECORD_STATUS_COMPLETE,
                        BROKERAGE_RECORD_STATUS_WITHDRAW,
                    ])
                );
            }
        }
        // 排序: update_time DESC, id DESC
        query = query
            .order_by_desc(user_brokerage_record::Column::UpdateTime)
            .order_by_desc(user_brokerage_record::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        // 批量查询用户昵称
        let uids: Vec<i32> = records.iter().map(|r| r.uid).collect();
        let user_map: HashMap<i32, String> = if !uids.is_empty() {
            user::Entity::find()
                .filter(user::Column::Uid.is_in(uids))
                .all(db)
                .await?
                .into_iter()
                .map(|u| (u.uid, u.nickname.unwrap_or_default()))
                .collect()
        } else {
            HashMap::new()
        };

        // 转换为响应，处理特殊title
        // Java: if link_type=withdraw && status=complete(3) && type=sub(2) → title="提现成功"
        let list: Vec<BrokerageRecordResponse> = records.iter().map(|r| {
            let user_name = user_map.get(&r.uid).cloned();
            let title = if r.link_type == BROKERAGE_RECORD_LINK_TYPE_WITHDRAW
                && r.status == BROKERAGE_RECORD_STATUS_COMPLETE
                && r.r#type == BROKERAGE_RECORD_TYPE_SUB
            {
                "提现成功".to_string()
            } else {
                r.title.clone()
            };

            BrokerageRecordResponse {
                id: r.id,
                uid: r.uid,
                link_id: r.link_id.clone(),
                link_type: r.link_type.clone(),
                record_type: r.r#type,
                title,
                price: r.price,
                balance: r.balance,
                mark: r.mark.clone(),
                status: r.status,
                frozen_time: r.frozen_time,
                thaw_time: r.thaw_time,
                create_time: r.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: r.update_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                brokerage_level: r.brokerage_level,
                user_name,
            }
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
}
