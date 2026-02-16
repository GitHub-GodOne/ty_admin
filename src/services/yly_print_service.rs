/// 易联云打印服务层
///
/// 处理易联云打印相关的业务逻辑
/// Java参考: com.zbkj.service.service.impl.YlyPrintServiceImpl
use sea_orm::*;
use tracing::{info, error};

use crate::common::constants;
use crate::dtos::yly_print::{YlyPrintRequest, YlyPrintRequestGoods};
use crate::models::_entities::{store_order, store_order_info};
use crate::services::system_config_service::SystemConfigService;
use crate::utils::redis_client::RedisClient;
use crate::utils::yly_util::YlyUtil;

pub struct YlyPrintService;

impl YlyPrintService {
    /// 易联云打印商品信息
    ///
    /// Java: YlyPrintServiceImpl.YlyPrint(String orderId, boolean isAuto)
    pub async fn yly_print(
        db: &DatabaseConnection,
        redis: &RedisClient,
        order_id: &str,
        is_auto: bool,
    ) -> Result<(), loco_rs::Error> {
        // 检查是否开启打印
        if YlyUtil::check_yly_print_status(db).await {
            return Err(loco_rs::Error::string("易联云 未开启打印"));
        }

        // 判断是否开启自动打印
        if is_auto && YlyUtil::check_yly_print_after_pay_success(db).await {
            return Ok(());
        }

        // 查找订单
        let exit_order = store_order::Entity::find()
            .filter(store_order::Column::OrderId.eq(order_id))
            .one(db)
            .await
            .map_err(|e| loco_rs::Error::string(&e.to_string()))?
            .ok_or_else(|| loco_rs::Error::string("易联云 打印时未找到 订单信息"))?;

        // 检查订单是否已支付
        if exit_order.paid != 1 {
            return Err(loco_rs::Error::string("易联云 打印时出错， 订单未支付"));
        }

        // 查询订单详情
        let order_infos = store_order_info::Entity::find()
            .filter(store_order_info::Column::OrderId.eq(exit_order.id))
            .all(db)
            .await
            .map_err(|e| loco_rs::Error::string(&e.to_string()))?;

        // 构建商品列表
        let goods: Vec<YlyPrintRequestGoods> = order_infos
            .iter()
            .map(|info| {
                YlyPrintRequestGoods::new(
                    &info.product_name,
                    info.price.to_string(),
                    info.pay_num.to_string(),
                    exit_order.pay_price.to_string(),
                )
            })
            .collect();

        // 获取站点名称
        let business_name = SystemConfigService::get_value_by_key(db, constants::CONFIG_KEY_SITE_NAME)
            .await
            .unwrap_or_else(|_| "".to_string());

        // 格式化支付时间
        let date = exit_order
            .pay_time
            .map(|t| t.format(constants::DATE_FORMAT).to_string())
            .unwrap_or_default();

        // 构建打印请求
        let yly_print_request = YlyPrintRequest {
            business_name,
            order_no: exit_order.order_id.clone(),
            date,
            name: exit_order.real_name.clone(),
            phone: exit_order.user_phone.clone(),
            address: exit_order.user_address.clone(),
            note: exit_order.mark.clone(),
            goods,
            amount: exit_order.pro_total_price.to_string(),
            discount: exit_order.deduction_price.to_string(),
            postal: exit_order.pay_postage.to_string(),
            deduction: exit_order.coupon_price.to_string(),
            pay_money: exit_order.pay_price.to_string(),
        };

        // 执行打印
        match YlyUtil::yly_print(db, redis, &yly_print_request).await {
            Ok(_) => {
                info!("易联云打印小票成功: {:?}", serde_json::to_string(&yly_print_request).unwrap_or_default());
            }
            Err(e) => {
                error!("易联云打印小票失败: {}", e);
            }
        }

        Ok(())
    }
}
