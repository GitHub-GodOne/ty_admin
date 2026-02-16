/// 订单管理服务
///
/// 实现与Java版本一致的订单管理业务逻辑
/// Java参考: StoreOrderServiceImpl
use loco_rs::prelude::*;
use rust_decimal::Decimal;
use sea_orm::*;
use sea_orm::sea_query::Expr;
use std::collections::HashMap;

use crate::dtos::store_order::*;
use crate::dtos::common::PageParamRequest;
use crate::common::pagination::PageResponse;
use crate::models::_entities::{
    store_order, store_order_info, store_order_status, store_pink, system_admin, system_store, user,
};
use crate::services::system_config_service::SystemConfigService;

/// 支付方式常量
const PAY_TYPE_WE_CHAT: &str = "weixin";
const PAY_TYPE_YUE: &str = "yue";
const PAY_TYPE_ALI_PAY: &str = "alipay";
const PAY_TYPE_STR_WE_CHAT: &str = "微信支付";
const PAY_TYPE_STR_YUE: &str = "余额支付";
const PAY_TYPE_STR_ALI_PAY: &str = "支付宝支付";
const PAY_TYPE_STR_OTHER: &str = "其他支付";

pub struct StoreOrderService;

impl StoreOrderService {
    /// 分页列表
    /// Java参考: StoreOrderServiceImpl.getAdminList()
    pub async fn get_admin_list(
        db: &DatabaseConnection,
        request: &StoreOrderSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<PageResponse<StoreOrderDetailResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_order::Entity::find();

        // 按订单号搜索
        if let Some(order_no) = &request.order_no {
            if !order_no.is_empty() {
                query = query.filter(store_order::Column::OrderId.eq(order_no.as_str()));
            }
        }

        // 时间区间
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                query = Self::apply_date_limit(query, date_limit);
            }
        }

        // 订单状态
        if let Some(status) = &request.status {
            if !status.is_empty() {
                query = Self::apply_status_where(query, status);
            }
        }

        // 订单类型
        let order_type = request.order_type.unwrap_or(2);
        if order_type != 2 {
            query = query.filter(store_order::Column::Type.eq(order_type));
        }

        // 系统未删除
        query = query.filter(store_order::Column::IsSystemDel.eq(0));

        // 按id降序
        query = query.order_by_desc(store_order::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let orders = paginator.fetch_page((page - 1) as u64).await?;

        // 格式化订单列表
        let list = Self::format_order_list(db, orders).await?;

        Ok(PageResponse::new(list, total, page as u64, limit as u64))
    }

    /// 获取订单各状态数量
    /// Java参考: StoreOrderServiceImpl.getOrderStatusNum()
    pub async fn get_order_status_num(
        db: &DatabaseConnection,
        date_limit: &Option<String>,
        order_type: Option<i32>,
        order_no: &Option<String>,
    ) -> Result<StoreOrderCountItemResponse> {
        let ot = if order_type == Some(2) { None } else { order_type };

        Ok(StoreOrderCountItemResponse {
            all: Self::get_count(db, date_limit, "all", ot, order_no).await?,
            un_paid: Self::get_count(db, date_limit, "unPaid", ot, order_no).await?,
            not_shipped: Self::get_count(db, date_limit, "notShipped", ot, order_no).await?,
            spike: Self::get_count(db, date_limit, "spike", ot, order_no).await?,
            bargain: Self::get_count(db, date_limit, "bargain", ot, order_no).await?,
            complete: Self::get_count(db, date_limit, "complete", ot, order_no).await?,
            to_be_written_off: Self::get_count(db, date_limit, "toBeWrittenOff", ot, order_no).await?,
            refunding: Self::get_count(db, date_limit, "refunding", ot, order_no).await?,
            refunded: Self::get_count(db, date_limit, "refunded", ot, order_no).await?,
            deleted: Self::get_count(db, date_limit, "deleted", ot, order_no).await?,
        })
    }

    /// 获取订单统计数据
    /// Java参考: StoreOrderServiceImpl.getOrderData()
    pub async fn get_order_data(
        db: &DatabaseConnection,
        date_limit: &Option<String>,
    ) -> Result<StoreOrderTopItemResponse> {
        Ok(StoreOrderTopItemResponse {
            count: Self::get_count(db, date_limit, "all", None, &None).await?,
            amount: Self::get_amount(db, date_limit, "").await?,
            we_chat_amount: Self::get_amount(db, date_limit, PAY_TYPE_WE_CHAT).await?,
            yue_amount: Self::get_amount(db, date_limit, PAY_TYPE_YUE).await?,
        })
    }

    /// 订单删除
    /// Java参考: StoreOrderServiceImpl.delete()
    pub async fn delete(db: &DatabaseConnection, order_no: &str) -> Result<bool> {
        let order = Self::get_info_exception(db, order_no).await?;
        if order.is_del == 0 {
            return Err(Error::string("您选择的的订单存在用户未删除的订单，无法删除用户未删除的订单！"));
        }
        if order.is_system_del == Some(1) {
            return Err(Error::string("此订单已经被删除了!"));
        }
        let mut active: store_order::ActiveModel = order.into();
        active.is_system_del = Set(Some(1i16));
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;
        Ok(true)
    }

    /// 备注订单
    /// Java参考: StoreOrderServiceImpl.mark()
    pub async fn mark(db: &DatabaseConnection, order_no: &str, mark: &str) -> Result<bool> {
        let order = Self::get_info_exception(db, order_no).await?;
        let mut active: store_order::ActiveModel = order.into();
        active.remark = Set(Some(mark.to_string()));
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;
        Ok(true)
    }

    /// 修改订单(改价)
    /// Java参考: StoreOrderServiceImpl.updatePrice()
    pub async fn update_price(
        db: &DatabaseConnection,
        request: &StoreOrderUpdatePriceRequest,
    ) -> Result<bool> {
        let order = Self::get_info_exception(db, &request.order_no).await?;
        if order.paid != 0 {
            return Err(Error::string(&format!("订单号为 {} 的订单已支付", order.order_id)));
        }
        if order.is_alter_price != 0 {
            return Err(Error::string("系统只支持一次改价"));
        }
        if order.pay_price == request.pay_price {
            return Err(Error::string(&format!(
                "修改价格不能和原支付价格相同 原价 {} 修改价 {}",
                order.pay_price, request.pay_price
            )));
        }
        let old_price = order.pay_price.to_string();

        // 修改订单价格
        let mut active: store_order::ActiveModel = order.clone().into();
        active.pay_price = Set(request.pay_price);
        active.is_alter_price = Set(1i16);
        active.before_pay_price = Set(order.pay_price);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        // 记录订单状态日志
        Self::create_order_log(
            db,
            order.id,
            "edit",
            &format!("订单改价 原价{}元 修改为{}元", old_price, request.pay_price),
        ).await?;

        Ok(true)
    }

    /// 订单详情
    /// Java参考: StoreOrderServiceImpl.info()
    pub async fn info(db: &DatabaseConnection, order_no: &str) -> Result<StoreOrderInfoResponse> {
        let order = Self::get_info_exception(db, order_no).await?;
        if order.is_system_del == Some(1) {
            return Err(Error::string("未找到对应订单信息"));
        }

        // 获取订单商品详情
        let order_infos = Self::get_order_info_list(db, order.id).await?;

        // 获取用户信息
        let user_info = user::Entity::find_by_id(order.uid)
            .one(db)
            .await?;
        let (nike_name, phone) = if let Some(u) = &user_info {
            (
                u.nickname.clone(),
                Some(Self::mask_mobile(u.phone.as_deref().unwrap_or(""))),
            )
        } else {
            (None, None)
        };

        // 获取订单类型文本
        let order_type_text = Self::get_order_type_str(db, &order).await;

        let status_str = Self::get_status(&order);
        let pay_type_str = Self::get_pay_type(&order.pay_type);
        let pro_total_price = order.total_price - order.total_postage;

        Ok(StoreOrderInfoResponse {
            id: order.id,
            order_id: order.order_id.clone(),
            uid: order.uid,
            real_name: order.real_name.clone(),
            user_phone: Self::mask_mobile(&order.user_phone),
            user_address: order.user_address.clone(),
            total_num: order.total_num,
            total_price: order.total_price,
            pay_price: order.pay_price,
            pay_postage: order.pay_postage,
            coupon_price: order.coupon_price,
            deduction_price: order.deduction_price,
            pay_type: order.pay_type.clone(),
            create_time: order.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            status: order.status as i32,
            refund_status: order.refund_status as i32,
            express_record_type: order.express_record_type,
            delivery_name: order.delivery_name.clone(),
            delivery_type: order.delivery_type.clone(),
            delivery_id: order.delivery_id.clone(),
            delivery_code: order.delivery_code.clone(),
            mark: order.mark.clone(),
            is_del: order.is_del != 0,
            remark: order.remark.clone(),
            refund_price: order.refund_price,
            use_integral: order.use_integral,
            back_integral: order.back_integral,
            verify_code: order.verify_code.clone(),
            shipping_type: order.shipping_type as i32,
            status_str,
            pay_type_str,
            nike_name,
            phone,
            order_info: order_infos,
            spread_name: None,
            pro_total_price,
            refund_reason_time: order.refund_reason_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            refund_reason_wap_img: order.refund_reason_wap_img.clone(),
            shipment_pic: order.shipment_pic.clone(),
            shipment_task_id: order.shipment_task_id.clone(),
            shipment_order_id: order.shipment_order_id.clone(),
            shipment_num: order.shipment_num.clone(),
            order_type_text,
        })
    }

    /// 发送货
    /// Java参考: StoreOrderServiceImpl.send()
    pub async fn send(
        db: &DatabaseConnection,
        request: &StoreOrderSendRequest,
    ) -> Result<String> {
        let order = Self::get_info_exception(db, &request.order_no).await?;
        if order.is_del != 0 {
            return Err(Error::string("订单已删除,不能发货!"));
        }
        if order.status > 0 {
            return Err(Error::string("订单已发货请勿重复操作!"));
        }
        // 拼团订单检查
        if let Some(cid) = order.combination_id {
            if cid > 0 {
                let pink = store_pink::Entity::find_by_id(order.pink_id)
                    .one(db)
                    .await?;
                if let Some(p) = pink {
                    if p.status != 2 {
                        return Err(Error::string("当前订单正在拼团中不能发货！"));
                    }
                }
            }
        }

        let result = match request.delivery_type.as_str() {
            "express" => {
                Self::send_express(db, &order, request).await?;
                String::new()
            }
            "send" => {
                Self::send_delivery(db, &order, request).await?;
                String::new()
            }
            "fictitious" => {
                Self::send_virtual(db, &order).await?;
                String::new()
            }
            _ => return Err(Error::string("类型错误")),
        };
        Ok(result)
    }

    /// 退款
    /// Java参考: StoreOrderServiceImpl.refund()
    pub async fn refund(
        db: &DatabaseConnection,
        request: &StoreOrderRefundRequest,
    ) -> Result<bool> {
        let order = Self::get_info_exception(db, &request.order_no).await?;
        if order.paid == 0 {
            return Err(Error::string("未支付无法退款"));
        }
        let amount = request.amount.unwrap_or(Decimal::ZERO);
        if order.refund_price + amount > order.pay_price {
            return Err(Error::string("退款金额大于支付金额，请修改退款金额"));
        }
        if amount <= Decimal::ZERO && order.pay_price != Decimal::ZERO {
            return Err(Error::string("退款金额不能为0，请修改退款金额"));
        }

        // 修改订单退款状态
        let mut active: store_order::ActiveModel = order.clone().into();
        active.refund_status = Set(3i16);
        active.refund_price = Set(amount);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        // 余额支付退款处理
        if order.pay_type == PAY_TYPE_YUE {
            // 更新用户余额
            user::Entity::update_many()
                .col_expr(
                    user::Column::NowMoney,
                    Expr::col(user::Column::NowMoney).add(amount),
                )
                .filter(user::Column::Uid.eq(order.uid))
                .exec(db)
                .await?;
        }

        // 记录退款日志
        Self::create_order_log(
            db,
            order.id,
            "refund_price",
            &format!("退款给用户{}元", amount),
        ).await?;

        Ok(true)
    }

    /// 拒绝退款
    /// Java参考: StoreOrderServiceImpl.refundRefuse()
    pub async fn refund_refuse(
        db: &DatabaseConnection,
        order_no: &str,
        reason: &str,
    ) -> Result<bool> {
        if reason.is_empty() {
            return Err(Error::string("请填写拒绝退款原因"));
        }
        let order = Self::get_info_exception(db, order_no).await?;
        let mut active: store_order::ActiveModel = order.clone().into();
        active.refund_reason = Set(Some(reason.to_string()));
        active.refund_status = Set(0i16);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Self::create_order_log(
            db,
            order.id,
            "refund_refuse",
            &format!("拒绝退款原因:{}", reason),
        ).await?;

        // 拼团订单恢复状态
        if order.pink_id > 0 {
            let pink = store_pink::Entity::find_by_id(order.pink_id as i32)
                .one(db)
                .await?;
            if let Some(p) = pink {
                if p.status == 3 {
                    let mut pa: store_pink::ActiveModel = p.into();
                    pa.status = Set(1i16);
                    pa.update(db).await?;
                }
            }
        }

        Ok(true)
    }

    /// 快递查询
    /// Java参考: StoreOrderServiceImpl.getLogisticsInfo()
    pub async fn get_logistics_info(
        db: &DatabaseConnection,
        order_no: &str,
    ) -> Result<LogisticsResultVo> {
        let order = Self::get_info_exception(db, order_no).await?;
        // 返回基本物流信息（实际物流查询需要第三方API）
        Ok(LogisticsResultVo {
            number: order.delivery_id.clone(),
            express_type: order.delivery_code.clone(),
            list: vec![],
            delivery_status: None,
            is_sign: None,
            exp_name: order.delivery_name.clone(),
            exp_site: None,
            exp_phone: None,
            courier: None,
            courier_phone: None,
            update_time: None,
            take_time: None,
            logo: None,
        })
    }

    /// 核销订单头部数据
    /// Java参考: StoreOrderVerificationImpl.getOrderVerificationData()
    pub async fn get_verification_data(
        db: &DatabaseConnection,
    ) -> Result<StoreStaffTopDetail> {
        let now = chrono::Local::now();
        let today_start = now.format("%Y-%m-%d 00:00:00").to_string();
        let today_end = now.format("%Y-%m-%d 23:59:59").to_string();
        let month_start = now.format("%Y-%m-01 00:00:00").to_string();

        // 基础查询条件：已支付、自提订单、未删除
        let base_filter = |q: Select<store_order::Entity>| -> Select<store_order::Entity> {
            q.filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::ShippingType.eq(2))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0))
        };

        // 总订单数
        let order_count = base_filter(store_order::Entity::find()).count(db).await? as i64;

        // 总金额
        let sum_price = Self::sum_pay_price(db, &base_filter(store_order::Entity::find())).await?;

        // 今日订单
        let today_count = base_filter(store_order::Entity::find())
            .filter(Expr::cust(&format!("create_time >= '{}'::timestamp", today_start)))
            .filter(Expr::cust(&format!("create_time <= '{}'::timestamp", today_end)))
            .count(db).await? as i64;

        let today_price = Self::sum_pay_price(
            db,
            &base_filter(store_order::Entity::find())
                .filter(Expr::cust(&format!("create_time >= '{}'::timestamp", today_start)))
                .filter(Expr::cust(&format!("create_time <= '{}'::timestamp", today_end))),
        ).await?;

        // 本月订单
        let month_count = base_filter(store_order::Entity::find())
            .filter(Expr::cust(&format!("create_time >= '{}'::timestamp", month_start)))
            .count(db).await? as i64;

        let month_price = Self::sum_pay_price(
            db,
            &base_filter(store_order::Entity::find())
                .filter(Expr::cust(&format!("create_time >= '{}'::timestamp", month_start))),
        ).await?;

        // 各状态数量
        let unpaid_count = store_order::Entity::find()
            .filter(store_order::Column::Paid.eq(0))
            .filter(store_order::Column::ShippingType.eq(2))
            .filter(store_order::Column::IsDel.eq(0))
            .count(db).await? as i64;

        let unshipped_count = base_filter(store_order::Entity::find())
            .filter(store_order::Column::Status.eq(0))
            .filter(store_order::Column::RefundStatus.eq(0))
            .count(db).await? as i64;

        let received_count = base_filter(store_order::Entity::find())
            .filter(store_order::Column::Status.eq(1))
            .count(db).await? as i64;

        let complete_count = base_filter(store_order::Entity::find())
            .filter(store_order::Column::Status.eq(3))
            .count(db).await? as i64;

        let evaluated_count = base_filter(store_order::Entity::find())
            .filter(store_order::Column::Status.eq(2))
            .count(db).await? as i64;

        let refund_count = base_filter(store_order::Entity::find())
            .filter(store_order::Column::RefundStatus.ne(0))
            .count(db).await? as i64;

        let verification_count = base_filter(store_order::Entity::find())
            .filter(store_order::Column::Status.eq(0))
            .filter(store_order::Column::RefundStatus.eq(0))
            .count(db).await? as i64;

        Ok(StoreStaffTopDetail {
            complete_count,
            evaluated_count,
            month_count,
            month_price,
            order_count,
            pro_count: order_count,
            pro_price: sum_price,
            received_count,
            refund_count,
            sum_price,
            today_count,
            today_price,
            unpaid_count,
            unshipped_count,
            verification_count,
        })
    }

    /// 核销订单月列表数据
    /// Java参考: StoreOrderVerificationImpl.getOrderVerificationDetail()
    pub async fn get_verification_detail(
        db: &DatabaseConnection,
        request: &StoreOrderStatisticsRequest,
    ) -> Result<Vec<StoreStaffDetail>> {
        let mut query = store_order::Entity::find()
            .filter(store_order::Column::Paid.eq(1))
            .filter(store_order::Column::ShippingType.eq(2))
            .filter(store_order::Column::IsDel.eq(0))
            .filter(store_order::Column::IsSystemDel.eq(0));

        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                query = Self::apply_date_limit(query, date_limit);
            }
        }

        let orders = query.all(db).await?;

        // 按日期分组统计
        let mut date_map: HashMap<String, (i64, Decimal)> = HashMap::new();
        for order in &orders {
            let date = order.create_time
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_default();
            let entry = date_map.entry(date).or_insert((0, Decimal::ZERO));
            entry.0 += 1;
            entry.1 += order.pay_price;
        }

        let mut result: Vec<StoreStaffDetail> = date_map
            .into_iter()
            .map(|(time, (count, price))| StoreStaffDetail { count, price, time })
            .collect();
        result.sort_by(|a, b| b.time.cmp(&a.time));

        Ok(result)
    }

    /// 核销码核销订单
    /// Java参考: StoreOrderVerificationImpl.verificationOrderByCode()
    pub async fn verification_order_by_code(
        db: &DatabaseConnection,
        v_code: &str,
    ) -> Result<serde_json::Value> {
        let order = store_order::Entity::find()
            .filter(store_order::Column::VerifyCode.eq(v_code))
            .filter(store_order::Column::IsDel.eq(0))
            .filter(store_order::Column::IsSystemDel.eq(0))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("未找到对应核销订单"))?;

        if order.paid == 0 {
            return Err(Error::string("订单未支付"));
        }
        if order.status != 0 {
            return Err(Error::string("订单状态异常，无法核销"));
        }
        if order.refund_status != 0 {
            return Err(Error::string("订单正在退款中，无法核销"));
        }

        // 核销：更新状态为已收货
        let mut active: store_order::ActiveModel = order.clone().into();
        active.status = Set(2i16);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Self::create_order_log(db, order.id, "write_off", "核销订单").await?;

        Ok(serde_json::json!({"orderId": order.order_id, "status": "success"}))
    }

    /// 核销码查询待核销订单
    /// Java参考: StoreOrderVerificationImpl.getVerificationOrderByCode()
    pub async fn get_verification_order_by_code(
        db: &DatabaseConnection,
        v_code: &str,
    ) -> Result<serde_json::Value> {
        let order = store_order::Entity::find()
            .filter(store_order::Column::VerifyCode.eq(v_code))
            .filter(store_order::Column::IsDel.eq(0))
            .filter(store_order::Column::IsSystemDel.eq(0))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("未找到对应核销订单"))?;

        // 获取订单商品信息
        let order_infos = Self::get_order_info_list(db, order.id).await?;

        Ok(serde_json::json!({
            "orderId": order.order_id,
            "realName": order.real_name,
            "userPhone": order.user_phone,
            "payPrice": order.pay_price,
            "totalNum": order.total_num,
            "status": order.status,
            "paid": order.paid != 0,
            "orderInfo": order_infos,
        }))
    }

    /// 订单统计详情
    /// Java参考: StoreOrderServiceImpl.orderStatisticsByTime()
    pub async fn order_statistics_by_time(
        db: &DatabaseConnection,
        date_limit: &str,
        stat_type: i32,
    ) -> Result<StoreOrderStatisticsResponse> {
        let (start_time, end_time) = Self::parse_date_limit(date_limit);

        // 当前时间段订单
        let current_orders = Self::get_order_payed_by_date(db, &start_time, &end_time).await?;

        // 计算天数差
        let days = Self::days_between(&start_time, &end_time);
        let per_start = Self::add_days(&start_time, -(days as i64));

        // 上一个时间段订单
        let per_orders = Self::get_order_payed_by_date(db, &per_start, &start_time).await?;

        let mut response = StoreOrderStatisticsResponse {
            chart: vec![],
            growth_rate: 0,
            increase_time: "0".to_string(),
            increase_time_status: 1,
            time: Decimal::ZERO,
        };

        if stat_type == 1 {
            // 按金额统计
            let per_sum: Decimal = per_orders.iter().map(|o| o.pay_price).sum();
            let current_sum: Decimal = current_orders.iter().map(|o| o.pay_price).sum();
            response.time = current_sum;
            response.chart = Self::build_price_chart(db, &start_time, &end_time).await?;

            let increase = current_sum - per_sum;
            response.increase_time = increase.to_string();
            response.increase_time_status = if increase >= Decimal::ZERO { 1 } else { 2 };
            if increase <= Decimal::ZERO {
                response.growth_rate = 0;
            } else if per_sum == Decimal::ZERO {
                response.growth_rate = 100;
            } else {
                let rate = (increase / per_sum * Decimal::from(100)).to_string();
                response.growth_rate = rate.parse::<f64>().unwrap_or(0.0) as i32;
            }
        } else {
            // 按订单数统计
            let current_count = current_orders.len() as i64;
            let per_count = per_orders.len() as i64;
            response.time = Decimal::from(current_count);
            response.chart = Self::build_count_chart(db, &start_time, &end_time).await?;

            let increase = current_count - per_count;
            response.increase_time = increase.to_string();
            response.increase_time_status = if increase >= 0 { 1 } else { 2 };
            if increase <= 0 {
                response.growth_rate = 0;
            } else if per_count == 0 {
                response.growth_rate = increase as i32;
            } else {
                response.growth_rate = ((increase as f64 / per_count as f64) * 100.0) as i32;
            }
        }

        Ok(response)
    }

    /// 获取面单默认配置信息
    /// Java参考: StoreOrderServiceImpl.getDeliveryInfo()
    pub async fn get_delivery_info(db: &DatabaseConnection) -> Result<ExpressSheetVo> {
        // 从system_config获取面单配置
        let export_open = SystemConfigService::get_value_by_key(db, "config_export_open")
            .await.unwrap_or_default();
        let export_id = SystemConfigService::get_value_by_key(db, "config_export_id")
            .await.unwrap_or_default();
        let export_com = SystemConfigService::get_value_by_key(db, "config_export_com")
            .await.unwrap_or_default();
        let export_temp_id = SystemConfigService::get_value_by_key(db, "config_export_temp_id")
            .await.unwrap_or_default();
        let export_to_name = SystemConfigService::get_value_by_key(db, "config_export_to_name")
            .await.unwrap_or_default();
        let export_to_tel = SystemConfigService::get_value_by_key(db, "config_export_to_tel")
            .await.unwrap_or_default();
        let export_to_address = SystemConfigService::get_value_by_key(db, "config_export_to_address")
            .await.unwrap_or_default();
        let export_siid = SystemConfigService::get_value_by_key(db, "config_export_siid")
            .await.unwrap_or_default();

        Ok(ExpressSheetVo {
            export_id: export_id.parse().ok(),
            export_com: Some(export_com).filter(|s| !s.is_empty()),
            export_temp_id: Some(export_temp_id).filter(|s| !s.is_empty()),
            export_to_name: Some(export_to_name).filter(|s| !s.is_empty()),
            export_to_tel: Some(export_to_tel).filter(|s| !s.is_empty()),
            export_to_address: Some(export_to_address).filter(|s| !s.is_empty()),
            export_siid: Some(export_siid).filter(|s| !s.is_empty()),
            export_open: export_open.parse().ok(),
        })
    }

    /// 更改订单运单号
    /// Java参考: StoreOrderServiceImpl.updateTrackingNumber()
    pub async fn update_tracking_number(
        db: &DatabaseConnection,
        request: &StoreOrderSendRequest,
    ) -> Result<bool> {
        let order = Self::get_info_exception(db, &request.order_no).await?;
        if order.is_del != 0 {
            return Err(Error::string("订单已删除,不能修改运单号!"));
        }
        if order.status != 1 {
            return Err(Error::string("待收货订单才能修改运单号"));
        }

        match request.delivery_type.as_str() {
            "express" => Self::send_express(db, &order, request).await?,
            "send" => Self::send_delivery(db, &order, request).await?,
            "fictitious" => Self::send_virtual(db, &order).await?,
            _ => return Err(Error::string("类型错误")),
        };

        Self::create_order_log(
            db,
            order.id,
            "express",
            &format!(
                "变更运单号,快递公司:{},运单号:{}",
                request.express_name.as_deref().unwrap_or(""),
                request.express_number.as_deref().unwrap_or("")
            ),
        ).await?;

        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 根据订单号获取订单，不存在则报错
    async fn get_info_exception(
        db: &DatabaseConnection,
        order_no: &str,
    ) -> Result<store_order::Model> {
        store_order::Entity::find()
            .filter(store_order::Column::OrderId.eq(order_no))
            .one(db)
            .await?
            .ok_or_else(|| Error::string("未找到对应订单信息"))
    }

    /// 按状态统计订单数量
    async fn get_count(
        db: &DatabaseConnection,
        date_limit: &Option<String>,
        status: &str,
        order_type: Option<i32>,
        order_no: &Option<String>,
    ) -> Result<i64> {
        let mut query = store_order::Entity::find();

        // 时间区间
        if let Some(dl) = date_limit {
            if !dl.is_empty() {
                query = Self::apply_date_limit(query, dl);
            }
        }

        // 订单类型
        if let Some(ot) = order_type {
            query = query.filter(store_order::Column::Type.eq(ot));
        }

        // 订单号
        if let Some(no) = order_no {
            if !no.is_empty() {
                query = query.filter(store_order::Column::OrderId.eq(no.as_str()));
            }
        }

        // 状态条件
        query = Self::apply_status_where(query, status);

        Ok(query.count(db).await? as i64)
    }

    /// 按条件统计订单金额
    async fn get_amount(
        db: &DatabaseConnection,
        date_limit: &Option<String>,
        pay_type: &str,
    ) -> Result<Decimal> {
        let mut query = store_order::Entity::find()
            .filter(store_order::Column::Paid.eq(1))
            .filter(store_order::Column::RefundStatus.ne(3))
            .filter(store_order::Column::IsSystemDel.eq(0));

        if let Some(dl) = date_limit {
            if !dl.is_empty() {
                query = Self::apply_date_limit(query, dl);
            }
        }

        if !pay_type.is_empty() {
            query = query.filter(store_order::Column::PayType.eq(pay_type));
        }

        let orders = query.all(db).await?;
        Ok(Self::sum_pay_price_vec(&orders))
    }

    /// 应用时间区间过滤
    /// date_limit 格式: "today", "yesterday", "lately7", "lately30", "month", "year"
    /// 或 "2024-01-01,2024-01-31"
    fn apply_date_limit(
        query: Select<store_order::Entity>,
        date_limit: &str,
    ) -> Select<store_order::Entity> {
        let now = chrono::Local::now();
        let (start, end) = match date_limit {
            "today" => {
                let s = now.format("%Y-%m-%d 00:00:00").to_string();
                let e = now.format("%Y-%m-%d 23:59:59").to_string();
                (s, e)
            }
            "yesterday" => {
                let yesterday = now - chrono::Duration::days(1);
                let s = yesterday.format("%Y-%m-%d 00:00:00").to_string();
                let e = yesterday.format("%Y-%m-%d 23:59:59").to_string();
                (s, e)
            }
            "lately7" => {
                let start_day = now - chrono::Duration::days(7);
                let s = start_day.format("%Y-%m-%d 00:00:00").to_string();
                let e = now.format("%Y-%m-%d 23:59:59").to_string();
                (s, e)
            }
            "lately30" => {
                let start_day = now - chrono::Duration::days(30);
                let s = start_day.format("%Y-%m-%d 00:00:00").to_string();
                let e = now.format("%Y-%m-%d 23:59:59").to_string();
                (s, e)
            }
            "month" => {
                let s = now.format("%Y-%m-01 00:00:00").to_string();
                let e = now.format("%Y-%m-%d 23:59:59").to_string();
                (s, e)
            }
            "year" => {
                let s = now.format("%Y-01-01 00:00:00").to_string();
                let e = now.format("%Y-%m-%d 23:59:59").to_string();
                (s, e)
            }
            other => {
                // 自定义区间: "2024-01-01,2024-01-31"
                if let Some((s, e)) = other.split_once(',') {
                    (
                        format!("{} 00:00:00", s.trim()),
                        format!("{} 23:59:59", e.trim()),
                    )
                } else {
                    return query;
                }
            }
        };

        query
            .filter(Expr::cust(&format!("create_time >= '{}'::timestamp", start)))
            .filter(Expr::cust(&format!("create_time <= '{}'::timestamp", end)))
    }

    /// 应用订单状态过滤条件
    /// Java参考: StoreOrderServiceImpl.getStatusWhere()
    fn apply_status_where(
        query: Select<store_order::Entity>,
        status: &str,
    ) -> Select<store_order::Entity> {
        match status {
            // 未支付
            "unPaid" => query
                .filter(store_order::Column::Paid.eq(0))
                .filter(store_order::Column::Status.eq(0))
                .filter(store_order::Column::RefundStatus.eq(0))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 未发货
            "notShipped" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::Status.eq(0))
                .filter(store_order::Column::RefundStatus.eq(0))
                .filter(store_order::Column::ShippingType.eq(1))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 待收货
            "spike" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::Status.eq(1))
                .filter(store_order::Column::RefundStatus.eq(0))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 待评价
            "bargain" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::Status.eq(2))
                .filter(store_order::Column::RefundStatus.eq(0))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 已完成
            "complete" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::Status.eq(3))
                .filter(store_order::Column::RefundStatus.eq(0))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 待核销
            "toBeWrittenOff" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::Status.eq(0))
                .filter(store_order::Column::RefundStatus.eq(0))
                .filter(store_order::Column::ShippingType.eq(2))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 退款中
            "refunding" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::RefundStatus.eq(1))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 已退款
            "refunded" => query
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::RefundStatus.eq(3))
                .filter(store_order::Column::IsDel.eq(0))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 已删除
            "deleted" => query
                .filter(store_order::Column::IsDel.eq(1))
                .filter(store_order::Column::IsSystemDel.eq(0)),
            // 全部（排除系统删除）
            _ => query
                .filter(store_order::Column::IsSystemDel.eq(0)),
        }
    }

    /// 获取订单状态描述
    /// Java参考: StoreOrderServiceImpl.getStatus()
    fn get_status(order: &store_order::Model) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if order.is_del != 0 {
            map.insert("key".to_string(), "delete".to_string());
            map.insert("value".to_string(), "已删除".to_string());
            return map;
        }

        if order.refund_status == 1 {
            map.insert("key".to_string(), "refunding".to_string());
            map.insert("value".to_string(), "申请退款中".to_string());
            return map;
        }
        if order.refund_status == 2 {
            map.insert("key".to_string(), "refund".to_string());
            map.insert("value".to_string(), "已退款".to_string());
            return map;
        }
        if order.refund_status == 3 {
            map.insert("key".to_string(), "refund".to_string());
            map.insert("value".to_string(), "已退款".to_string());
            return map;
        }

        if order.paid == 0 {
            map.insert("key".to_string(), "unPaid".to_string());
            map.insert("value".to_string(), "未支付".to_string());
            return map;
        }

        // 已支付
        if order.status == 0 {
            if order.shipping_type == 1 {
                map.insert("key".to_string(), "notShipped".to_string());
                map.insert("value".to_string(), "未发货".to_string());
            } else {
                map.insert("key".to_string(), "toBeWrittenOff".to_string());
                map.insert("value".to_string(), "待核销".to_string());
            }
        } else if order.status == 1 {
            map.insert("key".to_string(), "spike".to_string());
            map.insert("value".to_string(), "待收货".to_string());
        } else if order.status == 2 {
            map.insert("key".to_string(), "bargain".to_string());
            map.insert("value".to_string(), "待评价".to_string());
        } else if order.status == 3 {
            map.insert("key".to_string(), "complete".to_string());
            map.insert("value".to_string(), "已完成".to_string());
        }

        map
    }

    /// 获取支付方式文本
    fn get_pay_type(pay_type: &str) -> String {
        match pay_type {
            PAY_TYPE_WE_CHAT => PAY_TYPE_STR_WE_CHAT.to_string(),
            PAY_TYPE_YUE => PAY_TYPE_STR_YUE.to_string(),
            PAY_TYPE_ALI_PAY => PAY_TYPE_STR_ALI_PAY.to_string(),
            _ => PAY_TYPE_STR_OTHER.to_string(),
        }
    }

    /// 获取订单类型文本
    /// Java参考: StoreOrderServiceImpl.getOrderTypeStr()
    async fn get_order_type_str(
        _db: &DatabaseConnection,
        order: &store_order::Model,
    ) -> String {
        if order.shipping_type == 2 {
            return "核销订单".to_string();
        }
        if order.seckill_id > 0 {
            return "秒杀订单".to_string();
        }
        if let Some(bargain_id) = order.bargain_id {
            if bargain_id > 0 {
                return "砍价订单".to_string();
            }
        }
        if let Some(combination_id) = order.combination_id {
            if combination_id > 0 {
                return "拼团订单".to_string();
            }
        }
        if order.r#type == 1 {
            return "视频号订单".to_string();
        }
        "普通订单".to_string()
    }

    /// 格式化订单列表（添加商品信息和状态）
    async fn format_order_list(
        db: &DatabaseConnection,
        orders: Vec<store_order::Model>,
    ) -> Result<Vec<StoreOrderDetailResponse>> {
        let mut list = Vec::new();
        for order in &orders {
            let order_infos = Self::get_order_info_list(db, order.id).await?;
            let status_str = Self::get_status(order);
            let pay_type_str = Self::get_pay_type(&order.pay_type);
            let pro_total_price = order.total_price - order.total_postage;

            list.push(StoreOrderDetailResponse {
                order_id: order.order_id.clone(),
                pay_price: order.pay_price,
                pay_type: order.pay_type.clone(),
                create_time: order.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                status: order.status as i32,
                product_list: order_infos,
                status_str,
                pay_type_str,
                is_del: order.is_del != 0,
                refund_reason_wap_img: order.refund_reason_wap_img.clone(),
                refund_reason_wap_explain: order.refund_reason_wap_explain.clone(),
                refund_reason_time: order.refund_reason_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                refund_reason_wap: order.refund_reason_wap.clone(),
                refund_reason: order.refund_reason.clone(),
                refund_price: order.refund_price,
                refund_status: order.refund_status as i32,
                verify_code: order.verify_code.clone(),
                order_type: Self::get_order_type_str(db, order).await,
                remark: order.remark.clone(),
                real_name: order.real_name.clone(),
                pro_total_price,
                coupon_price: order.coupon_price,
                before_pay_price: order.before_pay_price,
                paid: order.paid != 0,
                order_type_num: order.r#type,
                is_alter_price: order.is_alter_price != 0,
                shipment_pic: order.shipment_pic.clone(),
                shipment_task_id: order.shipment_task_id.clone(),
                shipment_order_id: order.shipment_order_id.clone(),
            });
        }
        Ok(list)
    }

    /// 获取订单商品详情列表
    async fn get_order_info_list(
        db: &DatabaseConnection,
        order_id: i32,
    ) -> Result<Vec<StoreOrderInfoOldVo>> {
        let infos = store_order_info::Entity::find()
            .filter(store_order_info::Column::OrderId.eq(order_id))
            .all(db)
            .await?;

        let mut list = Vec::new();
        for info in infos {
            // 解析info字段中的JSON
            let info_json: Option<serde_json::Value> = serde_json::from_str(&info.info).ok();
            list.push(StoreOrderInfoOldVo {
                id: info.id,
                order_id: order_id,
                product_id: info.product_id,
                info: info_json,
                unique_key: info.unique.clone(),
            });
        }
        Ok(list)
    }

    /// 快递发货
    async fn send_express(
        db: &DatabaseConnection,
        order: &store_order::Model,
        request: &StoreOrderSendRequest,
    ) -> Result<()> {
        if request.express_name.is_none() || request.express_number.is_none() {
            return Err(Error::string("请选择快递公司和填写快递单号"));
        }

        let mut active: store_order::ActiveModel = order.clone().into();
        active.delivery_name = Set(request.express_name.clone());
        active.delivery_code = Set(request.express_code.clone());
        active.delivery_id = Set(request.express_number.clone());
        active.delivery_type = Set(Some("express".to_string()));
        active.status = Set(1i16);
        active.express_record_type = Set(request.express_record_type.as_ref().and_then(|s| s.parse().ok()));
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Self::create_order_log(
            db,
            order.id,
            "express",
            &format!(
                "已发货 快递公司:{} 快递单号:{}",
                request.express_name.as_deref().unwrap_or(""),
                request.express_number.as_deref().unwrap_or("")
            ),
        ).await?;

        Ok(())
    }

    /// 送货发货
    async fn send_delivery(
        db: &DatabaseConnection,
        order: &store_order::Model,
        request: &StoreOrderSendRequest,
    ) -> Result<()> {
        if request.delivery_name.is_none() || request.delivery_tel.is_none() {
            return Err(Error::string("请填写送货人姓名和电话"));
        }

        let mut active: store_order::ActiveModel = order.clone().into();
        active.delivery_name = Set(request.delivery_name.clone());
        active.delivery_id = Set(request.delivery_tel.clone());
        active.delivery_type = Set(Some("send".to_string()));
        active.status = Set(1i16);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Self::create_order_log(
            db,
            order.id,
            "delivery",
            &format!(
                "已配送 送货人:{} 电话:{}",
                request.delivery_name.as_deref().unwrap_or(""),
                request.delivery_tel.as_deref().unwrap_or("")
            ),
        ).await?;

        Ok(())
    }

    /// 虚拟发货
    async fn send_virtual(
        db: &DatabaseConnection,
        order: &store_order::Model,
    ) -> Result<()> {
        let mut active: store_order::ActiveModel = order.clone().into();
        active.delivery_type = Set(Some("fictitious".to_string()));
        active.status = Set(1i16);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;

        Self::create_order_log(db, order.id, "delivery_fictitious", "虚拟发货").await?;

        Ok(())
    }

    /// 创建订单状态日志
    async fn create_order_log(
        db: &DatabaseConnection,
        order_id: i32,
        change_type: &str,
        change_message: &str,
    ) -> Result<()> {
        let log = store_order_status::ActiveModel {
            id: NotSet,
            oid: Set(order_id),
            change_type: Set(change_type.to_string()),
            change_message: Set(change_message.to_string()),
            create_time: Set(chrono::Local::now().naive_local()),
        };
        log.insert(db).await?;
        Ok(())
    }

    /// 手机号脱敏
    fn mask_mobile(phone: &str) -> String {
        if phone.len() >= 7 {
            format!("{}****{}", &phone[..3], &phone[phone.len() - 4..])
        } else {
            phone.to_string()
        }
    }

    /// 计算订单金额总和（从查询结果）
    async fn sum_pay_price(
        db: &DatabaseConnection,
        query: &Select<store_order::Entity>,
    ) -> Result<Decimal> {
        let orders = query.clone().all(db).await?;
        Ok(Self::sum_pay_price_vec(&orders))
    }

    /// 计算订单金额总和（从Vec）
    fn sum_pay_price_vec(orders: &[store_order::Model]) -> Decimal {
        orders.iter().map(|o| o.pay_price).sum()
    }

    /// 解析日期区间字符串
    /// 返回 (start_date, end_date) 格式: "YYYY-MM-DD"
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

    /// 计算两个日期之间的天数
    fn days_between(start: &str, end: &str) -> i64 {
        let start_date = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d").unwrap_or_default();
        let end_date = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d").unwrap_or_default();
        (end_date - start_date).num_days().max(1)
    }

    /// 日期加减天数
    fn add_days(date: &str, days: i64) -> String {
        let d = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap_or_default();
        (d + chrono::Duration::days(days)).format("%Y-%m-%d").to_string()
    }

    /// 构建金额统计图表数据
    async fn build_price_chart(
        db: &DatabaseConnection,
        start: &str,
        end: &str,
    ) -> Result<Vec<StoreOrderStatisticsChartItemResponse>> {
        let days = Self::days_between(start, end);
        let mut chart = Vec::new();

        for i in 0..=days {
            let date = Self::add_days(start, i);
            let day_start = format!("{} 00:00:00", date);
            let day_end = format!("{} 23:59:59", date);

            let orders = store_order::Entity::find()
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::RefundStatus.ne(3))
                .filter(store_order::Column::IsSystemDel.eq(0))
                .filter(Expr::cust(&format!("pay_time >= '{}'::timestamp", day_start)))
                .filter(Expr::cust(&format!("pay_time <= '{}'::timestamp", day_end)))
                .all(db)
                .await?;

            let amount: Decimal = orders.iter().map(|o| o.pay_price).sum();
            chart.push(StoreOrderStatisticsChartItemResponse {
                num: amount.to_string(),
                time: date,
            });
        }

        Ok(chart)
    }

    /// 构建订单数统计图表数据
    async fn build_count_chart(
        db: &DatabaseConnection,
        start: &str,
        end: &str,
    ) -> Result<Vec<StoreOrderStatisticsChartItemResponse>> {
        let days = Self::days_between(start, end);
        let mut chart = Vec::new();

        for i in 0..=days {
            let date = Self::add_days(start, i);
            let day_start = format!("{} 00:00:00", date);
            let day_end = format!("{} 23:59:59", date);

            let count = store_order::Entity::find()
                .filter(store_order::Column::Paid.eq(1))
                .filter(store_order::Column::RefundStatus.ne(3))
                .filter(store_order::Column::IsSystemDel.eq(0))
                .filter(Expr::cust(&format!("pay_time >= '{}'::timestamp", day_start)))
                .filter(Expr::cust(&format!("pay_time <= '{}'::timestamp", day_end)))
                .count(db)
                .await?;

            chart.push(StoreOrderStatisticsChartItemResponse {
                num: count.to_string(),
                time: date,
            });
        }

        Ok(chart)
    }

    /// 获取指定日期范围内已支付的订单
    async fn get_order_payed_by_date(
        db: &DatabaseConnection,
        start: &str,
        end: &str,
    ) -> Result<Vec<store_order::Model>> {
        let start_time = format!("{} 00:00:00", start);
        let end_time = format!("{} 23:59:59", end);

        let orders = store_order::Entity::find()
            .filter(store_order::Column::Paid.eq(1))
            .filter(store_order::Column::RefundStatus.ne(3))
            .filter(store_order::Column::IsSystemDel.eq(0))
            .filter(Expr::cust(&format!("pay_time >= '{}'::timestamp", start_time)))
            .filter(Expr::cust(&format!("pay_time <= '{}'::timestamp", end_time)))
            .all(db)
            .await?;

        Ok(orders)
    }

    /// 核销订单列表
    /// Java参考: StoreOrderServiceImpl.getWriteOffList()
    pub async fn get_write_off_list(
        db: &DatabaseConnection,
        request: &SystemWriteOffOrderSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<SystemWriteOffOrderResponse> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        // 基础条件: is_del=0 AND shipping_type=2
        let mut base_query = store_order::Entity::find()
            .filter(store_order::Column::IsDel.eq(0))
            .filter(store_order::Column::ShippingType.eq(2));

        // 时间过滤
        if let Some(date_limit) = &request.date_limit {
            if !date_limit.is_empty() {
                base_query = Self::apply_date_limit(base_query, date_limit);
            }
        }

        // 关键字过滤: real_name LIKE OR user_phone= OR order_id= OR id=
        if let Some(keywords) = &request.keywords {
            if !keywords.is_empty() {
                let kw = keywords.trim().to_string();
                base_query = base_query.filter(
                    Condition::any()
                        .add(store_order::Column::RealName.contains(&kw))
                        .add(store_order::Column::UserPhone.eq(&kw))
                        .add(store_order::Column::OrderId.eq(&kw))
                        .add(Expr::cust(&format!("id = '{}'", kw))),
                );
            }
        }

        // 门店过滤
        if let Some(store_id) = request.store_id {
            if store_id > 0 {
                base_query = base_query.filter(store_order::Column::StoreId.eq(store_id));
            }
        }

        // 统计: 订单总金额 (refund_status=0)
        let total_price_orders = base_query.clone()
            .filter(store_order::Column::RefundStatus.eq(0))
            .all(db)
            .await?;
        let order_total_price: Decimal = total_price_orders.iter().map(|o| o.pay_price).sum();

        // 统计: 退款总金额 (refund_status=2)
        let refund_orders = base_query.clone()
            .filter(store_order::Column::RefundStatus.eq(2))
            .all(db)
            .await?;
        let refund_total_price: Decimal = refund_orders.iter().map(|o| o.refund_price).sum();
        let refund_total = refund_orders.len() as i64;

        // 分页查询
        let paginator = base_query.clone()
            .order_by_desc(store_order::Column::Id)
            .paginate(db, limit as u64);
        let total = paginator.num_items().await? as i64;
        let orders = paginator.fetch_page((page - 1) as u64).await?;

        // 格式化订单
        let list = Self::format_write_off_order(db, orders).await?;

        Ok(SystemWriteOffOrderResponse {
            total,
            order_total_price,
            refund_total_price,
            refund_total,
            list: PageResponse::new(list, total as u64, page as u64, limit as u64),
        })
    }

    /// 格式化核销订单列表（添加门店、店员、商品、推广人信息）
    /// Java参考: StoreOrderServiceImpl.formatOrder()
    async fn format_write_off_order(
        db: &DatabaseConnection,
        orders: Vec<store_order::Model>,
    ) -> Result<Vec<StoreOrderItemResponse>> {
        if orders.is_empty() {
            return Ok(vec![]);
        }

        // 收集各种ID
        let store_ids: Vec<i32> = orders.iter().map(|o| o.store_id).collect::<std::collections::HashSet<_>>().into_iter().collect();
        let clerk_ids: Vec<i32> = orders.iter().map(|o| o.clerk_id).collect::<std::collections::HashSet<_>>().into_iter().collect();
        let order_ids: Vec<i32> = orders.iter().map(|o| o.id).collect();
        let user_ids: Vec<i32> = orders.iter().map(|o| o.uid).collect::<std::collections::HashSet<_>>().into_iter().collect();

        // 批量查询门店
        let store_map = Self::get_store_map(db, &store_ids).await?;
        // 批量查询店员(admin)
        let admin_map = Self::get_admin_map(db, &clerk_ids).await?;
        // 批量查询订单详情
        let order_info_map = Self::get_order_info_map(db, &order_ids).await?;
        // 批量查询用户
        let user_map = Self::get_user_map(db, &user_ids).await?;

        // 收集推广人ID
        let spread_uids: Vec<i32> = user_map.values()
            .filter_map(|u| u.spread_uid)
            .filter(|&uid| uid > 0)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let spread_map = if !spread_uids.is_empty() {
            Self::get_user_map(db, &spread_uids).await?
        } else {
            HashMap::new()
        };

        // 批量查询拼团信息
        let pink_ids: Vec<i32> = orders.iter()
            .filter(|o| o.pink_id > 0)
            .map(|o| o.pink_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let pink_map = Self::get_pink_map(db, &pink_ids).await?;

        let mut list = Vec::new();
        for order in &orders {
            let store_name = store_map.get(&order.store_id)
                .map(|s| s.name.clone())
                .unwrap_or_default();

            let clerk_name = admin_map.get(&order.clerk_id)
                .map(|a| a.real_name.clone())
                .unwrap_or_default();

            let product_list = order_info_map.get(&order.id)
                .cloned()
                .unwrap_or_default();

            let status_str = Self::get_status(order);
            let pay_type_str = Self::get_pay_type(&order.pay_type);

            // 推广人信息
            let mut spread_info = StoreOrderSpreadInfoResponse { id: 0, name: String::new() };
            if let Some(u) = user_map.get(&order.uid) {
                if let Some(spread_uid) = u.spread_uid {
                    if let Some(spread_user) = spread_map.get(&spread_uid) {
                        spread_info.id = spread_user.uid;
                        spread_info.name = spread_user.nickname.clone().unwrap_or_default();
                    }
                }
            }

            // 订单类型
            let order_type = Self::get_write_off_order_type(order, &pink_map);

            list.push(StoreOrderItemResponse {
                id: order.id,
                order_id: order.order_id.clone(),
                uid: order.uid,
                real_name: order.real_name.clone(),
                user_phone: order.user_phone.clone(),
                total_price: order.total_price,
                pay_price: order.pay_price,
                paid: order.paid != 0,
                pay_time: order.pay_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                pay_type: order.pay_type.clone(),
                create_time: order.create_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                status: order.status as i32,
                store_name,
                clerk_name,
                product_list,
                status_str,
                pay_type_str,
                total_postage: order.total_postage,
                pay_postage: order.pay_postage,
                gain_integral: order.gain_integral,
                use_integral: order.use_integral,
                back_integral: order.back_integral,
                is_del: order.is_del != 0,
                is_system_del: order.is_system_del == Some(1),
                mark: order.mark.clone(),
                remark: order.remark.clone(),
                refund_reason_wap_img: order.refund_reason_wap_img.clone(),
                refund_reason_wap_explain: order.refund_reason_wap_explain.clone(),
                refund_reason_time: order.refund_reason_time.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                refund_reason_wap: order.refund_reason_wap.clone(),
                refund_reason: order.refund_reason.clone(),
                refund_price: order.refund_price,
                refund_status: order.refund_status as i32,
                total_num: order.total_num,
                shipping_type: order.shipping_type as i32,
                verify_code: order.verify_code.clone(),
                spread_info,
                order_type,
            });
        }
        Ok(list)
    }

    /// 获取核销订单类型文本
    /// Java参考: formatOrder() 中的订单类型逻辑
    fn get_write_off_order_type(
        order: &store_order::Model,
        pink_map: &HashMap<i32, store_pink::Model>,
    ) -> String {
        // 核销
        if !order.verify_code.is_empty() {
            return "[核销订单]".to_string();
        }
        // 秒杀
        if order.seckill_id > 0 {
            return "[秒杀订单]".to_string();
        }
        // 砍价
        if let Some(bargain_id) = order.bargain_id {
            if bargain_id > 0 {
                return "[砍价订单]".to_string();
            }
        }
        // 拼团
        if order.pink_id > 0 {
            if let Some(pink) = pink_map.get(&order.pink_id) {
                let pink_status = match pink.status {
                    2 => "已完成",
                    3 => "未完成",
                    _ => "正在进行中",
                };
                return format!("[拼团订单]{}", pink_status);
            }
        }
        "[普通订单]".to_string()
    }

    /// 批量查询门店 map
    async fn get_store_map(
        db: &DatabaseConnection,
        store_ids: &[i32],
    ) -> Result<HashMap<i32, system_store::Model>> {
        if store_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let stores = system_store::Entity::find()
            .filter(system_store::Column::Id.is_in(store_ids.to_vec()))
            .all(db)
            .await?;
        Ok(stores.into_iter().map(|s| (s.id, s)).collect())
    }

    /// 批量查询管理员 map
    async fn get_admin_map(
        db: &DatabaseConnection,
        admin_ids: &[i32],
    ) -> Result<HashMap<i32, system_admin::Model>> {
        if admin_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let admins = system_admin::Entity::find()
            .filter(system_admin::Column::Id.is_in(admin_ids.to_vec()))
            .all(db)
            .await?;
        Ok(admins.into_iter().map(|a| (a.id, a)).collect())
    }

    /// 批量查询订单详情 map
    async fn get_order_info_map(
        db: &DatabaseConnection,
        order_ids: &[i32],
    ) -> Result<HashMap<i32, Vec<StoreOrderInfoOldVo>>> {
        if order_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let infos = store_order_info::Entity::find()
            .filter(store_order_info::Column::OrderId.is_in(order_ids.to_vec()))
            .all(db)
            .await?;
        let mut map: HashMap<i32, Vec<StoreOrderInfoOldVo>> = HashMap::new();
        for info in infos {
            let info_json: Option<serde_json::Value> = serde_json::from_str(&info.info).ok();
            let vo = StoreOrderInfoOldVo {
                id: info.id,
                order_id: info.order_id,
                product_id: info.product_id,
                info: info_json,
                unique_key: info.unique.clone(),
            };
            map.entry(info.order_id).or_default().push(vo);
        }
        Ok(map)
    }

    /// 批量查询用户 map
    async fn get_user_map(
        db: &DatabaseConnection,
        user_ids: &[i32],
    ) -> Result<HashMap<i32, user::Model>> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let users = user::Entity::find()
            .filter(user::Column::Uid.is_in(user_ids.to_vec()))
            .all(db)
            .await?;
        Ok(users.into_iter().map(|u| (u.uid, u)).collect())
    }

    /// 批量查询拼团 map
    async fn get_pink_map(
        db: &DatabaseConnection,
        pink_ids: &[i32],
    ) -> Result<HashMap<i32, store_pink::Model>> {
        if pink_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let pinks = store_pink::Entity::find()
            .filter(store_pink::Column::Id.is_in(pink_ids.to_vec()))
            .all(db)
            .await?;
        Ok(pinks.into_iter().map(|p| (p.id, p)).collect())
    }
}
