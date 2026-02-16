/// 用户管理服务
///
/// 实现与Java版本一致的用户管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;
use sea_orm::sea_query::Expr;

use crate::models::_entities::user;
use crate::dtos::user::{UserSearchRequest, UserResponse};
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

        // 转换为响应格式
        let list: Vec<UserResponse> = users
            .into_iter()
            .map(|u| Self::model_to_response(u))
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
            tag_id: model.tag_id,
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
}
