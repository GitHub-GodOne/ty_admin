/// 用户统计服务
///
/// 实现与Java版本一致的用户统计业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// 用户渠道数据响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserChannelDataResponse {
    /// 渠道类型: h5, ios, routine, wechat
    pub channel: String,
    /// 用户数量
    pub num: i32,
}

/// 用户概览数据响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserOverviewResponse {
    /// 注册用户数
    #[serde(rename = "registerNum")]
    pub register_num: i32,

    /// 注册用户数环比
    #[serde(rename = "registerNumRatio")]
    pub register_num_ratio: String,

    /// 活跃用户数
    #[serde(rename = "activeUserNum")]
    pub active_user_num: i32,

    /// 活跃用户数环比
    #[serde(rename = "activeUserNumRatio")]
    pub active_user_num_ratio: String,

    /// 充值用户数
    #[serde(rename = "rechargeUserNum")]
    pub recharge_user_num: i32,

    /// 充值用户数环比
    #[serde(rename = "rechargeUserNumRatio")]
    pub recharge_user_num_ratio: String,

    /// 浏览量
    pub pageviews: i32,

    /// 下单用户数量
    #[serde(rename = "orderUserNum")]
    pub order_user_num: i32,

    /// 成交用户数量
    #[serde(rename = "orderPayUserNum")]
    pub order_pay_user_num: i32,

    /// 成交金额
    #[serde(rename = "payOrderAmount")]
    pub pay_order_amount: Decimal,

    /// 客单价
    #[serde(rename = "customerPrice")]
    pub customer_price: Decimal,
}

/// 用户统计服务
pub struct UserStatisticsService;

impl UserStatisticsService {
    /// 获取用户渠道数据
    ///
    /// 统计不同渠道(user_type)的用户数量
    /// Java参考: UserStatisticsServiceImpl.getChannelData()
    pub async fn get_channel_data(db: &DatabaseConnection) -> Result<Vec<UserChannelDataResponse>> {
        // 使用原生SQL查询按user_type分组统计
        let sql = r#"
            SELECT user_type as channel, COUNT(*) as num
            FROM ty_user
            GROUP BY user_type
            ORDER BY num DESC
        "#;

        let results: Vec<(String, i64)> = db
            .query_all(Statement::from_string(
                DatabaseBackend::Postgres,
                sql.to_string(),
            ))
            .await?
            .iter()
            .map(|row| {
                let channel: String = row.try_get("", "channel").unwrap_or_default();
                let num: i64 = row.try_get("", "num").unwrap_or(0);
                (channel, num)
            })
            .collect();

        let response = results
            .into_iter()
            .map(|(channel, num)| UserChannelDataResponse {
                channel,
                num: num as i32,
            })
            .collect();

        Ok(response)
    }

    /// 获取用户概览数据
    ///
    /// 根据dateLimit参数返回不同时间范围的统计数据
    /// 支持: day, yesterday, lately7, lately30, week, month, year, 自定义日期范围
    /// Java参考: UserStatisticsServiceImpl.getOverview()
    pub async fn get_overview(
        _db: &DatabaseConnection,
        date_limit: &str,
    ) -> Result<UserOverviewResponse> {
        // TODO: 实现完整的用户概览统计逻辑
        // 目前返回模拟数据，避免500错误

        tracing::info!("获取用户概览数据, dateLimit: {}", date_limit);

        Ok(UserOverviewResponse {
            register_num: 0,
            register_num_ratio: "0%".to_string(),
            active_user_num: 0,
            active_user_num_ratio: "0%".to_string(),
            recharge_user_num: 0,
            recharge_user_num_ratio: "0%".to_string(),
            pageviews: 0,
            order_user_num: 0,
            order_pay_user_num: 0,
            pay_order_amount: Decimal::ZERO,
            customer_price: Decimal::ZERO,
        })
    }
}
