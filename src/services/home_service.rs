/// Home统计服务
///
/// 实现与Java版本一致的统计业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;
use serde_json::Value;
use std::collections::HashMap;
use chrono::{Datelike, Duration, NaiveDate, Utc};

use crate::models::_entities::{
    store_order, user, user_visit_record, store_product,
};

/// 首页数据响应
#[derive(Debug, serde::Serialize)]
pub struct HomeRateResponse {
    pub sales: Value,
    #[serde(rename = "yesterdaySales")]
    pub yesterday_sales: Value,
    pub pageviews: Value,
    #[serde(rename = "yesterdayPageviews")]
    pub yesterday_pageviews: Value,
    #[serde(rename = "orderNum")]
    pub order_num: Value,
    #[serde(rename = "yesterdayOrderNum")]
    pub yesterday_order_num: Value,
    #[serde(rename = "newUserNum")]
    pub new_user_num: Value,
    #[serde(rename = "yesterdayNewUserNum")]
    pub yesterday_new_user_num: Value,
}

/// 首页经营数据响应
#[derive(Debug, serde::Serialize)]
pub struct HomeOperatingDataResponse {
    #[serde(rename = "notShippingOrderNum")]
    pub not_shipping_order_num: i32,
    #[serde(rename = "refundingOrderNum")]
    pub refunding_order_num: i32,
    #[serde(rename = "notWriteOffOrderNum")]
    pub not_write_off_order_num: i32,
    #[serde(rename = "vigilanceInventoryNum")]
    pub vigilance_inventory_num: i32,
    #[serde(rename = "onSaleProductNum")]
    pub on_sale_product_num: i32,
    #[serde(rename = "notSaleProductNum")]
    pub not_sale_product_num: i32,
    #[serde(rename = "notAuditNum")]
    pub not_audit_num: i32,
    #[serde(rename = "totalRechargeAmount")]
    pub total_recharge_amount: rust_decimal::Decimal,
}

/// Home统计服务
pub struct HomeService;

impl HomeService {
    /// 1. 首页数据 - 今日/昨日对比
    pub async fn index_date(db: &DatabaseConnection) -> Result<HomeRateResponse> {
        let today = Utc::now().date_naive();
        let yesterday = today - Duration::days(1);

        let today_str = today.format("%Y-%m-%d").to_string();
        let yesterday_str = yesterday.format("%Y-%m-%d").to_string();

        // 查询今日和昨日的销售额
        let today_sales = Self::get_sales_by_date(db, &today_str).await?;
        let yesterday_sales = Self::get_sales_by_date(db, &yesterday_str).await?;

        // 查询今日和昨日的访问量
        let today_pageviews = Self::get_pageviews_by_date(db, &today_str).await?;
        let yesterday_pageviews = Self::get_pageviews_by_date(db, &yesterday_str).await?;

        // 查询今日和昨日的订单量
        let today_orders = Self::get_order_count_by_date(db, &today_str).await?;
        let yesterday_orders = Self::get_order_count_by_date(db, &yesterday_str).await?;

        // 查询今日和昨日的新增用户
        let today_users = Self::get_new_user_count_by_date(db, &today_str).await?;
        let yesterday_users = Self::get_new_user_count_by_date(db, &yesterday_str).await?;

        Ok(HomeRateResponse {
            sales: serde_json::json!(today_sales),
            yesterday_sales: serde_json::json!(yesterday_sales),
            pageviews: serde_json::json!(today_pageviews),
            yesterday_pageviews: serde_json::json!(yesterday_pageviews),
            order_num: serde_json::json!(today_orders),
            yesterday_order_num: serde_json::json!(yesterday_orders),
            new_user_num: serde_json::json!(today_users),
            yesterday_new_user_num: serde_json::json!(yesterday_users),
        })
    }

    /// 2. 用户曲线图 - 最近30天
    pub async fn chart_user(db: &DatabaseConnection) -> Result<HashMap<String, Value>> {
        let end_date = Utc::now().date_naive();
        let start_date = end_date - Duration::days(29);

        let mut date_list = Vec::new();
        let mut num_list = Vec::new();

        for i in 0..30 {
            let date = start_date + Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();
            let count = Self::get_new_user_count_by_date(db, &date_str).await?;

            date_list.push(date.format("%m-%d").to_string());
            num_list.push(count);
        }

        let mut map = HashMap::new();
        map.insert("date".to_string(), serde_json::json!(date_list));
        map.insert("num".to_string(), serde_json::json!(num_list));

        Ok(map)
    }

    /// 3. 用户购买统计
    pub async fn chart_user_buy(db: &DatabaseConnection) -> Result<HashMap<String, i32>> {
        // 统计不同购买次数的用户数量
        let zero_count = Self::get_user_count_by_pay_count(db, 0, 0).await?;
        let one_count = Self::get_user_count_by_pay_count(db, 1, 1).await?;
        let history_count = Self::get_user_count_by_pay_count(db, 2, 3).await?;
        let back_count = Self::get_user_count_by_pay_count(db, 1, 999999).await?;

        let mut map = HashMap::new();
        map.insert("zero".to_string(), zero_count);
        map.insert("one".to_string(), one_count);
        map.insert("history".to_string(), history_count);
        map.insert("back".to_string(), back_count);

        Ok(map)
    }

    /// 4. 30天订单量趋势
    pub async fn chart_order(db: &DatabaseConnection) -> Result<HashMap<String, Value>> {
        let end_date = Utc::now().date_naive();
        let start_date = end_date - Duration::days(29);

        let mut date_list = Vec::new();
        let mut order_num_list = Vec::new();
        let mut order_amount_list = Vec::new();

        for i in 0..30 {
            let date = start_date + Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();

            let count = Self::get_order_count_by_date(db, &date_str).await?;
            let amount = Self::get_sales_by_date(db, &date_str).await?;

            date_list.push(date.format("%m-%d").to_string());
            order_num_list.push(count);
            order_amount_list.push(amount);
        }

        let mut map = HashMap::new();
        map.insert("date".to_string(), serde_json::json!(date_list));
        map.insert("orderNum".to_string(), serde_json::json!(order_num_list));
        map.insert("orderAmount".to_string(), serde_json::json!(order_amount_list));

        Ok(map)
    }

    /// 5. 周订单量趋势
    pub async fn chart_order_in_week(db: &DatabaseConnection) -> Result<HashMap<String, Value>> {
        Self::chart_order_compare(db, 7, &["周一", "周二", "周三", "周四", "周五", "周六", "周日"]).await
    }

    /// 6. 月订单量趋势
    pub async fn chart_order_in_month(db: &DatabaseConnection) -> Result<HashMap<String, Value>> {
        let labels: Vec<String> = (1..=31).map(|i| i.to_string()).collect();
        let labels_ref: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        Self::chart_order_compare(db, 30, &labels_ref).await
    }

    /// 7. 年订单量趋势
    pub async fn chart_order_in_year(db: &DatabaseConnection) -> Result<HashMap<String, Value>> {
        let months = ["一月", "二月", "三月", "四月", "五月", "六月",
                      "七月", "八月", "九月", "十月", "十一月", "十二月"];
        Self::chart_order_compare_year(db, &months).await
    }

    /// 8. 首页经营数据
    pub async fn operating_data(db: &DatabaseConnection) -> Result<HomeOperatingDataResponse> {
        // 待发货订单数量 (status=0, shipping_type=1)
        let not_shipping = store_order::Entity::find()
            .filter(store_order::Column::Status.eq(0))
            .filter(store_order::Column::ShippingType.eq(1))
            .filter(store_order::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 退款中订单数量 (refund_status in (1,2))
        let refunding = store_order::Entity::find()
            .filter(store_order::Column::RefundStatus.is_in([1, 2]))
            .filter(store_order::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 待核销订单数量 (status=0, shipping_type=2)
        let not_write_off = store_order::Entity::find()
            .filter(store_order::Column::Status.eq(0))
            .filter(store_order::Column::ShippingType.eq(2))
            .filter(store_order::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 库存预警商品数量 (stock < 10, is_show=1, is_del=0)
        // 注意：Java版本使用ot_num字段作为预警阈值，这里简化为固定值10
        let vigilance_inventory = store_product::Entity::find()
            .filter(store_product::Column::Stock.lt(10))
            .filter(store_product::Column::IsShow.eq(1))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 上架商品数量
        let on_sale = store_product::Entity::find()
            .filter(store_product::Column::IsShow.eq(1))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // 仓库中商品数量
        let not_sale = store_product::Entity::find()
            .filter(store_product::Column::IsShow.eq(0))
            .filter(store_product::Column::IsDel.eq(0))
            .count(db)
            .await? as i32;

        // TODO: 提现待审核数量和充值总额需要相应的表
        let not_audit = 0;
        let total_recharge = rust_decimal::Decimal::ZERO;

        Ok(HomeOperatingDataResponse {
            not_shipping_order_num: not_shipping,
            refunding_order_num: refunding,
            not_write_off_order_num: not_write_off,
            vigilance_inventory_num: vigilance_inventory,
            on_sale_product_num: on_sale,
            not_sale_product_num: not_sale,
            not_audit_num: not_audit,
            total_recharge_amount: total_recharge,
        })
    }

    // ==================== 辅助方法 ====================

    /// 获取指定日期的销售额
    async fn get_sales_by_date(db: &DatabaseConnection, date: &str) -> Result<rust_decimal::Decimal> {
        use sea_orm::sea_query::Expr;

        let result = store_order::Entity::find()
            .filter(Expr::cust(&format!("create_time >= '{} 00:00:00'::timestamp", date)))
            .filter(Expr::cust(&format!("create_time < '{} 23:59:59'::timestamp", date)))
            .filter(store_order::Column::Paid.eq(1))
            .filter(store_order::Column::IsDel.eq(0))
            .filter(store_order::Column::RefundStatus.eq(0))
            .all(db)
            .await?;

        let total: rust_decimal::Decimal = result
            .iter()
            .map(|o| o.pay_price)
            .sum();

        Ok(total)
    }

    /// 获取指定日期的访问量
    async fn get_pageviews_by_date(db: &DatabaseConnection, date: &str) -> Result<i64> {
        let count = user_visit_record::Entity::find()
            .filter(user_visit_record::Column::Date.eq(date))
            .count(db)
            .await?;

        Ok(count as i64)
    }

    /// 获取指定日期的订单数量
    async fn get_order_count_by_date(db: &DatabaseConnection, date: &str) -> Result<i64> {
        use sea_orm::sea_query::Expr;

        let count = store_order::Entity::find()
            .filter(Expr::cust(&format!("create_time >= '{} 00:00:00'::timestamp", date)))
            .filter(Expr::cust(&format!("create_time < '{} 23:59:59'::timestamp", date)))
            .filter(store_order::Column::Paid.eq(1))
            .filter(store_order::Column::IsDel.eq(0))
            .count(db)
            .await?;

        Ok(count as i64)
    }

    /// 获取指定日期的新增用户数量
    async fn get_new_user_count_by_date(db: &DatabaseConnection, date: &str) -> Result<i64> {
        use sea_orm::sea_query::Expr;

        let count = user::Entity::find()
            .filter(Expr::cust(&format!("create_time >= '{} 00:00:00'::timestamp", date)))
            .filter(Expr::cust(&format!("create_time < '{} 23:59:59'::timestamp", date)))
            .count(db)
            .await?;

        Ok(count as i64)
    }

    /// 获取指定购买次数范围的用户数量
    async fn get_user_count_by_pay_count(db: &DatabaseConnection, min: i32, max: i32) -> Result<i32> {
        let count = user::Entity::find()
            .filter(user::Column::PayCount.gte(min))
            .filter(user::Column::PayCount.lte(max))
            .count(db)
            .await?;

        Ok(count as i32)
    }

    /// 对比两个时间段的订单数据（周/月）
    async fn chart_order_compare(
        db: &DatabaseConnection,
        days: i64,
        _labels: &[&str],
    ) -> Result<HashMap<String, Value>> {
        let end_date = Utc::now().date_naive();
        let start_date = end_date - Duration::days(days - 1);
        let pre_end_date = start_date - Duration::days(1);
        let pre_start_date = pre_end_date - Duration::days(days - 1);

        // 当前周期数据
        let (cur_nums, cur_amounts) = Self::get_order_data_range(db, start_date, end_date, days as usize).await?;

        // 上一周期数据
        let (pre_nums, pre_amounts) = Self::get_order_data_range(db, pre_start_date, pre_end_date, days as usize).await?;

        let mut map = HashMap::new();
        map.insert("quality".to_string(), serde_json::json!(cur_nums));
        map.insert("price".to_string(), serde_json::json!(cur_amounts));
        map.insert("preQuality".to_string(), serde_json::json!(pre_nums));
        map.insert("prePrice".to_string(), serde_json::json!(pre_amounts));

        Ok(map)
    }

    /// 对比两年的订单数据
    async fn chart_order_compare_year(
        db: &DatabaseConnection,
        _labels: &[&str],
    ) -> Result<HashMap<String, Value>> {
        let current_year = Utc::now().year();
        let last_year = current_year - 1;

        // 当前年数据
        let (cur_nums, cur_amounts) = Self::get_order_data_by_year(db, current_year).await?;

        // 去年数据
        let (pre_nums, pre_amounts) = Self::get_order_data_by_year(db, last_year).await?;

        let mut map = HashMap::new();
        map.insert("quality".to_string(), serde_json::json!(cur_nums));
        map.insert("price".to_string(), serde_json::json!(cur_amounts));
        map.insert("preQuality".to_string(), serde_json::json!(pre_nums));
        map.insert("prePrice".to_string(), serde_json::json!(pre_amounts));

        Ok(map)
    }

    /// 获取日期范围内的订单数据
    async fn get_order_data_range(
        db: &DatabaseConnection,
        start: NaiveDate,
        end: NaiveDate,
        size: usize,
    ) -> Result<(Vec<i64>, Vec<rust_decimal::Decimal>)> {
        let mut nums = Vec::with_capacity(size);
        let mut amounts = Vec::with_capacity(size);

        let mut current = start;
        while current <= end {
            let date_str = current.format("%Y-%m-%d").to_string();
            let count = Self::get_order_count_by_date(db, &date_str).await?;
            let amount = Self::get_sales_by_date(db, &date_str).await?;

            nums.push(count);
            amounts.push(amount);

            current = current + Duration::days(1);
        }

        Ok((nums, amounts))
    }

    /// 获取指定年份的订单数据（按月）
    async fn get_order_data_by_year(
        db: &DatabaseConnection,
        year: i32,
    ) -> Result<(Vec<i64>, Vec<rust_decimal::Decimal>)> {
        let mut nums = Vec::with_capacity(12);
        let mut amounts = Vec::with_capacity(12);

        for month in 1..=12 {
            let start_date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let end_date = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap() - Duration::days(1)
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap() - Duration::days(1)
            };

            let (month_nums, month_amounts) = Self::get_order_data_range(
                db,
                start_date,
                end_date,
                (end_date - start_date).num_days() as usize + 1,
            ).await?;

            let total_num: i64 = month_nums.iter().sum();
            let total_amount: rust_decimal::Decimal = month_amounts.iter().sum();

            nums.push(total_num);
            amounts.push(total_amount);
        }

        Ok((nums, amounts))
    }
}
