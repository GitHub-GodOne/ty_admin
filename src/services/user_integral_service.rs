/// 用户积分管理 -- 服务层
///
/// 实现与Java版本一致的用户积分管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.UserIntegralRecordServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use sea_orm::sea_query::Expr;

use crate::dtos::user_integral::*;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::models::_entities::user_integral_record;
use crate::models::_entities::user;

/// 积分记录状态常量
const INTEGRAL_RECORD_STATUS_COMPLETE: i16 = 3;

pub struct UserIntegralRecordService;

impl UserIntegralRecordService {
    /// 管理端积分分页列表
    ///
    /// Java: UserIntegralRecordServiceImpl.findAdminList()
    /// 搜索条件:
    /// - 只查询 status = COMPLETE(3) 的记录
    /// - uid: 精确匹配
    /// - keywords: 模糊匹配用户昵称（先查用户uid列表，再过滤）
    /// - dateLimit: 时间区间过滤 update_time
    /// 排序: id DESC
    pub async fn find_admin_list(
        db: &DatabaseConnection,
        request: &AdminIntegralSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserIntegralRecordResponse>> {
        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        let mut query = user_integral_record::Entity::find();

        // 只查询已完成的记录
        query = query.filter(
            user_integral_record::Column::Status.eq(INTEGRAL_RECORD_STATUS_COMPLETE),
        );

        // uid 精确匹配
        if let Some(uid) = request.uid {
            if uid > 0 {
                query = query.filter(user_integral_record::Column::Uid.eq(uid));
            }
        }

        // keywords 搜索用户昵称 -> 先查用户uid列表
        if let Some(keywords) = &request.keywords {
            let kw = keywords.trim();
            if !kw.is_empty() {
                // 查询匹配昵称的用户uid列表
                let user_uids: Vec<i32> = user::Entity::find()
                    .filter(user::Column::Nickname.contains(kw))
                    .all(db)
                    .await?
                    .into_iter()
                    .map(|u| u.uid)
                    .collect();

                if user_uids.is_empty() {
                    // 没有匹配的用户，直接返回空
                    return Ok(CommonPage::new(vec![], 0, page, limit));
                }
                query = query.filter(
                    user_integral_record::Column::Uid.is_in(user_uids),
                );
            }
        }

        // 时间区间过滤 update_time
        if let Some(date_limit) = &request.date_limit {
            let dl = date_limit.trim();
            if !dl.is_empty() {
                let (start_date, end_date) = Self::parse_date_limit(dl);
                query = query.filter(
                    Expr::cust(&format!(
                        "update_time >= '{} 00:00:00'::timestamp",
                        start_date
                    )),
                );
                query = query.filter(
                    Expr::cust(&format!(
                        "update_time <= '{} 23:59:59'::timestamp",
                        end_date
                    )),
                );
            }
        }

        // 排序: id DESC
        query = query.order_by_desc(user_integral_record::Column::Id);

        // 查询总数
        let total = query.clone().count(db).await? as i64;

        // 分页查询
        let records = query
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        // 批量查询用户昵称
        let uid_list: Vec<i32> = records.iter().map(|r| r.uid).collect();
        let user_map = Self::get_user_nickname_map(db, &uid_list).await?;

        let list: Vec<UserIntegralRecordResponse> = records
            .into_iter()
            .map(|r| {
                let nick_name = user_map.get(&r.uid).cloned().flatten();
                Self::model_to_response(r, nick_name)
            })
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }

    // ==================== 内部方法 ====================

    /// 批量查询用户昵称
    async fn get_user_nickname_map(
        db: &DatabaseConnection,
        uid_list: &[i32],
    ) -> Result<std::collections::HashMap<i32, Option<String>>> {
        if uid_list.is_empty() {
            return Ok(std::collections::HashMap::new());
        }

        let unique_uids: Vec<i32> = {
            let mut set = std::collections::HashSet::new();
            uid_list.iter().filter(|&&uid| set.insert(uid)).copied().collect()
        };

        let users = user::Entity::find()
            .filter(user::Column::Uid.is_in(unique_uids))
            .all(db)
            .await?;

        let map: std::collections::HashMap<i32, Option<String>> = users
            .into_iter()
            .map(|u| (u.uid, u.nickname))
            .collect();

        Ok(map)
    }

    /// Model 转 Response
    fn model_to_response(
        model: user_integral_record::Model,
        nick_name: Option<String>,
    ) -> UserIntegralRecordResponse {
        UserIntegralRecordResponse {
            id: model.id,
            uid: model.uid,
            link_id: model.link_id,
            link_type: model.link_type,
            record_type: model.r#type,
            title: model.title,
            integral: model.integral,
            balance: model.balance,
            mark: model.mark,
            status: model.status,
            frozen_time: model.frozen_time,
            thaw_time: model.thaw_time,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            nick_name,
        }
    }

    /// 解析时间区间
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
