/// 订单操作记录 -- 服务层
///
/// 实现与Java版本一致的订单操作记录管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.StoreOrderStatusServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_order_status::*;
use crate::models::_entities::{store_order, store_order_status};

pub struct StoreOrderStatusService;

impl StoreOrderStatusService {
    /// 获取订单操作记录列表
    ///
    /// Java: StoreOrderStatusServiceImpl.getList()
    /// 逻辑：
    /// 1. 根据 orderNo 查找订单
    /// 2. 根据订单 id 查询操作记录
    /// 3. 按 create_time 倒序排列
    /// 4. 分页返回
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreOrderStatusSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreOrderStatusResponse>> {
        // 1. 根据 orderNo 查找订单
        let order = store_order::Entity::find()
            .filter(store_order::Column::OrderId.eq(&request.order_no))
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        // 2. 查询总数
        let total = store_order_status::Entity::find()
            .filter(store_order_status::Column::Oid.eq(order.id))
            .count(db)
            .await? as i64;

        // 3. 分页查询操作记录，按 create_time 倒序
        let records = store_order_status::Entity::find()
            .filter(store_order_status::Column::Oid.eq(order.id))
            .order_by_desc(store_order_status::Column::CreateTime)
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        // 4. 转换为响应 DTO
        let list: Vec<StoreOrderStatusResponse> = records
            .into_iter()
            .map(|r| StoreOrderStatusResponse {
                id: r.id,
                oid: r.oid,
                change_type: r.change_type,
                change_message: r.change_message,
                create_time: r.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }
}
