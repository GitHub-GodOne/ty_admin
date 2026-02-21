/// 用户提现服务
///
/// Java参考: UserExtractServiceImpl
use sea_orm::*;
use sea_orm::sea_query::Expr;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::user_extract::*;
use crate::models::_entities::{user, user_extract, user_brokerage_record};

/// 佣金记录常量
const BROKERAGE_RECORD_TYPE_ADD: i32 = 1;
const BROKERAGE_RECORD_TYPE_SUB: i32 = 2;
const BROKERAGE_RECORD_STATUS_COMPLETE: i32 = 3;
// const BROKERAGE_RECORD_STATUS_WITHDRAW: i32 = 5;
const BROKERAGE_RECORD_LINK_TYPE_ORDER: &str = "order";
const BROKERAGE_RECORD_LINK_TYPE_WITHDRAW: &str = "withdraw";

pub struct UserExtractService;

impl UserExtractService {
    /// 提现申请列表
    ///
    /// Java: UserExtractServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &UserExtractSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<serde_json::Value>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = user_extract::Entity::find();

        // 关键字搜索（微信号/姓名/银行卡/开户行/支付宝/失败原因）
        // Java: LIKE '%keywords%' on wechat, realName, bankCode, bankAddress, alipayCode, failMsg
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(user_extract::Column::Wechat.contains(keywords))
                        .add(user_extract::Column::RealName.contains(keywords))
                        .add(user_extract::Column::BankCode.contains(keywords))
                        .add(user_extract::Column::BankAddress.contains(keywords))
                        .add(user_extract::Column::AlipayCode.contains(keywords))
                        .add(user_extract::Column::FailMsg.contains(keywords))
                );
            }
        }

        // 提现状态筛选
        if let Some(status) = request.status {
            query = query.filter(user_extract::Column::Status.eq(status));
        }

        // 提现方式筛选
        if let Some(extract_type) = &request.extract_type {
            if !extract_type.is_empty() {
                query = query.filter(user_extract::Column::ExtractType.eq(extract_type.as_str()));
            }
        }

        // 时间区间筛选
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                let (start, end) = Self::parse_date_limit(date_limit);
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

        // 排序: create_time DESC, id DESC
        query = query
            .order_by_desc(user_extract::Column::CreateTime)
            .order_by_desc(user_extract::Column::Id);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        // 批量查询用户昵称
        let uids: Vec<i32> = records.iter().filter_map(|r| r.uid).collect();
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

        // 转换为JSON Value，附加nickName字段
        // Java: userExtract.setNickName(nickName)
        let list: Vec<serde_json::Value> = records.iter().map(|r| {
            let nick_name = r.uid
                .and_then(|uid| user_map.get(&uid))
                .cloned()
                .unwrap_or_default();
            let mut val = serde_json::to_value(r).unwrap_or_default();
            if let Some(obj) = val.as_object_mut() {
                obj.insert("nickName".to_string(), serde_json::Value::String(nick_name));
                // 格式化时间
                if let Some(ct) = r.create_time {
                    obj.insert("createTime".to_string(),
                        serde_json::Value::String(ct.format("%Y-%m-%d %H:%M:%S").to_string()));
                }
                if let Some(ut) = r.update_time {
                    obj.insert("updateTime".to_string(),
                        serde_json::Value::String(ut.format("%Y-%m-%d %H:%M:%S").to_string()));
                }
                if let Some(ft) = r.fail_time {
                    obj.insert("failTime".to_string(),
                        serde_json::Value::String(ft.format("%Y-%m-%d %H:%M:%S").to_string()));
                }
            }
            val
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 修改提现申请
    ///
    /// Java: UserExtractServiceImpl.updateExtract()
    pub async fn update_extract(
        db: &DatabaseConnection,
        request: &UserExtractUpdateRequest,
    ) -> Result<bool, DbErr> {
        let extract = user_extract::Entity::find_by_id(request.id)
            .one(db)
            .await?;
        let extract = match extract {
            Some(e) => e,
            None => return Ok(false),
        };
        let mut active: user_extract::ActiveModel = extract.into();
        if let Some(ref v) = request.real_name {
            active.real_name = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.extract_type {
            active.extract_type = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.bank_code {
            active.bank_code = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.bank_name {
            active.bank_name = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.alipay_code {
            active.alipay_code = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.extract_price {
            active.extract_price = Set(Some(*v));
        }
        if let Some(ref v) = request.wechat {
            active.wechat = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.mark {
            active.mark = Set(Some(v.clone()));
        }
        if let Some(ref v) = request.qrcode_url {
            active.qrcode_url = Set(Some(v.clone()));
        }
        active.update_time = Set(Some(chrono::Local::now().naive_local()));
        active.update(db).await?;
        Ok(true)
    }

    /// 提现统计
    ///
    /// Java: UserExtractServiceImpl.getBalance()
    /// withdrawn = 已提现(status=1), toBeWithdrawn = 待提现(status=0)
    /// commissionTotal = 佣金总金额, unDrawn = 未提现 = commissionTotal - subWithdraw
    pub async fn get_balance(
        db: &DatabaseConnection,
        date_limit: &str,
    ) -> Result<ExtractBalanceResponse, DbErr> {
        let (start, end) = if !date_limit.is_empty() {
            Self::parse_date_limit(date_limit)
        } else {
            (String::new(), String::new())
        };

        // 已提现 (status=1)
        let withdrawn = Self::get_sum(db, 1, &start, &end).await?;
        // 待提现/审核中 (status=0)
        let to_be_withdrawn = Self::get_sum(db, 0, &start, &end).await?;

        // 佣金总金额 (brokerage_record: link_type=order, type=add(1), status=complete(3))
        let commission_total = Self::get_total_spread_price(db, date_limit).await?;
        // 消耗的佣金 (brokerage_record: type=sub(2), status=complete(3))
        let sub_withdraw = Self::get_sub_spread_price(db, date_limit).await?;
        // 未提现 = 佣金总金额 - 消耗佣金
        let un_drawn = commission_total - sub_withdraw;

        Ok(ExtractBalanceResponse {
            withdrawn,
            un_drawn,
            commission_total,
            to_be_withdrawn,
        })
    }

    /// 提现审核
    ///
    /// Java: UserExtractServiceImpl.updateStatus()
    /// status: -1=拒绝, 1=同意
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: i32,
        back_message: Option<String>,
    ) -> Result<bool, DbErr> {
        // 拒绝时必须填写驳回原因
        if status == -1 {
            if back_message.as_ref().map_or(true, |m| m.is_empty()) {
                return Err(DbErr::Custom("驳回时请填写驳回原因".to_string()));
            }
        }

        let extract = user_extract::Entity::find_by_id(id).one(db).await?;
        let extract = match extract {
            Some(e) => e,
            None => return Err(DbErr::Custom("提现申请记录不存在".to_string())),
        };
        if extract.status != Some(0) {
            return Err(DbErr::Custom("提现申请已处理过".to_string()));
        }

        let uid = extract.uid.ok_or(DbErr::Custom("提现用户数据异常".to_string()))?;
        let user_record = user::Entity::find()
            .filter(user::Column::Uid.eq(uid))
            .one(db)
            .await?;
        let user_record = match user_record {
            Some(u) => u,
            None => return Err(DbErr::Custom("提现用户数据异常".to_string())),
        };

        let now = chrono::Local::now().naive_local();

        // 拒绝: 恢复用户佣金 + 创建佣金记录
        if status == -1 {
            let extract_price = extract.extract_price.unwrap_or(Decimal::ZERO);
            let brokerage_price = user_record.brokerage_price.unwrap_or(Decimal::ZERO);

            // 更新提现记录
            let mut active_extract: user_extract::ActiveModel = extract.into();
            active_extract.status = Set(Some(-1));
            active_extract.fail_msg = Set(back_message);
            active_extract.update_time = Set(Some(now));
            active_extract.update(db).await?;

            // 返还用户佣金
            let mut active_user: user::ActiveModel = user_record.into();
            active_user.brokerage_price = Set(Some(brokerage_price + extract_price));
            active_user.update(db).await?;

            // 创建佣金记录: 提现申请拒绝返还佣金
            let brokerage_record = user_brokerage_record::ActiveModel {
                uid: Set(uid),
                link_id: Set(id.to_string()),
                link_type: Set(BROKERAGE_RECORD_LINK_TYPE_WITHDRAW.to_string()),
                r#type: Set(BROKERAGE_RECORD_TYPE_ADD),
                title: Set("提现失败".to_string()),
                price: Set(extract_price),
                balance: Set(brokerage_price + extract_price),
                mark: Set(format!("提现申请拒绝返还佣金{}", extract_price)),
                status: Set(BROKERAGE_RECORD_STATUS_COMPLETE),
                frozen_time: Set(0),
                thaw_time: Set(0),
                create_time: Set(Some(now)),
                update_time: Set(Some(now)),
                brokerage_level: Set(None),
                ..Default::default()
            };
            brokerage_record.insert(db).await?;
            return Ok(true);
        }

        // 同意: 更新提现记录状态 + 更新佣金记录状态
        if status == 1 {
            // 查找对应的佣金记录
            let brokerage = user_brokerage_record::Entity::find()
                .filter(user_brokerage_record::Column::LinkId.eq(id.to_string()))
                .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_WITHDRAW))
                .one(db)
                .await?;
            let brokerage = match brokerage {
                Some(b) => b,
                None => return Err(DbErr::Custom("对应的佣金记录不存在".to_string())),
            };

            // 更新提现记录
            let mut active_extract: user_extract::ActiveModel = extract.into();
            active_extract.status = Set(Some(1));
            active_extract.update_time = Set(Some(now));
            active_extract.update(db).await?;

            // 更新佣金记录状态为完成
            let mut active_brokerage: user_brokerage_record::ActiveModel = brokerage.into();
            active_brokerage.status = Set(BROKERAGE_RECORD_STATUS_COMPLETE);
            active_brokerage.update_time = Set(Some(now));
            active_brokerage.update(db).await?;
            return Ok(true);
        }

        Ok(false)
    }

    /// 根据状态获取提现总额
    ///
    /// Java: UserExtractServiceImpl.getSum()
    async fn get_sum(
        db: &DatabaseConnection,
        status: i32,
        start_time: &str,
        end_time: &str,
    ) -> Result<Decimal, DbErr> {
        let mut query = user_extract::Entity::find();
        query = query.filter(user_extract::Column::Status.eq(status));
        if !start_time.is_empty() && !end_time.is_empty() {
            query = query.filter(
                Expr::cust(&format!(
                    "create_time >= '{} 00:00:00'::timestamp AND create_time <= '{} 23:59:59'::timestamp",
                    start_time.replace('\'', "''"),
                    end_time.replace('\'', "''")
                ))
            );
        }
        let records = query.all(db).await?;
        let sum = records.iter()
            .filter_map(|r| r.extract_price)
            .fold(Decimal::ZERO, |acc, p| acc + p);
        Ok(sum)
    }

    /// 佣金总金额（按时间范围）
    ///
    /// Java: UserBrokerageRecordServiceImpl.getTotalSpreadPriceBydateLimit()
    /// link_type=order, type=add(1), status=complete(3)
    async fn get_total_spread_price(
        db: &DatabaseConnection,
        date_limit: &str,
    ) -> Result<Decimal, DbErr> {
        let mut query = user_brokerage_record::Entity::find();
        query = query
            .filter(user_brokerage_record::Column::LinkType.eq(BROKERAGE_RECORD_LINK_TYPE_ORDER))
            .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_ADD))
            .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_COMPLETE));
        if !date_limit.is_empty() {
            let (start, end) = Self::parse_date_limit(date_limit);
            if !start.is_empty() && !end.is_empty() {
                query = query.filter(
                    Expr::cust(&format!(
                        "update_time >= '{} 00:00:00'::timestamp AND update_time <= '{} 23:59:59'::timestamp",
                        start.replace('\'', "''"),
                        end.replace('\'', "''")
                    ))
                );
            }
        }
        let records = query.all(db).await?;
        let sum = records.iter().fold(Decimal::ZERO, |acc, r| acc + r.price);
        Ok(sum)
    }

    /// 消耗的佣金（按时间范围）
    ///
    /// Java: UserBrokerageRecordServiceImpl.getSubSpreadPriceByDateLimit()
    /// type=sub(2), status=complete(3)
    async fn get_sub_spread_price(
        db: &DatabaseConnection,
        date_limit: &str,
    ) -> Result<Decimal, DbErr> {
        let mut query = user_brokerage_record::Entity::find();
        query = query
            .filter(user_brokerage_record::Column::Type.eq(BROKERAGE_RECORD_TYPE_SUB))
            .filter(user_brokerage_record::Column::Status.eq(BROKERAGE_RECORD_STATUS_COMPLETE));
        if !date_limit.is_empty() {
            let (start, end) = Self::parse_date_limit(date_limit);
            if !start.is_empty() && !end.is_empty() {
                query = query.filter(
                    Expr::cust(&format!(
                        "update_time >= '{} 00:00:00'::timestamp AND update_time <= '{} 23:59:59'::timestamp",
                        start.replace('\'', "''"),
                        end.replace('\'', "''")
                    ))
                );
            }
        }
        let records = query.all(db).await?;
        let sum = records.iter().fold(Decimal::ZERO, |acc, r| acc + r.price);
        Ok(sum)
    }

    /// 解析dateLimit参数为(start, end)日期字符串
    ///
    /// Java: CrmebDateUtil.getDateLimit()
    fn parse_date_limit(date_limit: &str) -> (String, String) {
        let now = chrono::Local::now();
        match date_limit {
            "today" => {
                let d = now.format("%Y-%m-%d").to_string();
                (d.clone(), d)
            }
            "yesterday" => {
                let d = (now - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
                (d.clone(), d)
            }
            "lately7" => {
                let s = (now - chrono::Duration::days(7)).format("%Y-%m-%d").to_string();
                let e = now.format("%Y-%m-%d").to_string();
                (s, e)
            }
            "lately30" => {
                let s = (now - chrono::Duration::days(30)).format("%Y-%m-%d").to_string();
                let e = now.format("%Y-%m-%d").to_string();
                (s, e)
            }
            "month" => {
                let s = now.format("%Y-%m-01").to_string();
                let e = now.format("%Y-%m-%d").to_string();
                (s, e)
            }
            "year" => {
                let s = now.format("%Y-01-01").to_string();
                let e = now.format("%Y-%m-%d").to_string();
                (s, e)
            }
            other => {
                if let Some((s, e)) = other.split_once(',') {
                    (s.trim().to_string(), e.trim().to_string())
                } else {
                    let d = now.format("%Y-%m-%d").to_string();
                    (d.clone(), d)
                }
            }
        }
    }
}
