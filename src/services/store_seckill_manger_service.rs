/// 秒杀时间段配置服务
///
/// Java参考: StoreSeckillMangerServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Timelike;

use crate::models::_entities::store_seckill_manger;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::store_seckill::{
    StoreSeckillMangerSearchRequest, StoreSeckillMangerRequest, StoreSeckillMangerResponse,
};

pub struct StoreSeckillMangerService;

impl StoreSeckillMangerService {
    /// 分页列表
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &StoreSeckillMangerSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<StoreSeckillMangerResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = store_seckill_manger::Entity::find();

        if let Some(name) = &request.name {
            if !name.is_empty() {
                query = query.filter(store_seckill_manger::Column::Name.contains(name));
            }
        }

        if let Some(status) = &request.status {
            if !status.is_empty() {
                query = query.filter(store_seckill_manger::Column::Status.eq(status.clone()));
            }
        }

        query = query.order_by_asc(store_seckill_manger::Column::Sort);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<StoreSeckillMangerResponse> = items
            .into_iter()
            .map(|m| Self::model_to_response(&m))
            .collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 新增秒杀配置
    pub async fn save_manger(
        db: &DatabaseConnection,
        request: &StoreSeckillMangerRequest,
    ) -> Result<bool> {
        let (start_time, end_time) = Self::parse_time_range(&request.time)?;

        // 检查时间段是否已存在
        let existing = Self::check_time_range_unique(db, start_time, end_time).await?;
        if !existing.is_empty() {
            return Err(Error::string("当前时间段的秒杀配置已存在"));
        }

        let now = chrono::Local::now().naive_local();
        let model = store_seckill_manger::ActiveModel {
            name: Set(Some(request.name.clone())),
            start_time: Set(Some(start_time)),
            end_time: Set(Some(end_time)),
            img: Set(request.img.clone()),
            silder_imgs: Set(request.silder_imgs.clone()),
            sort: Set(request.sort),
            status: Set(request.status.clone()),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            is_del: Set(0),
            ..Default::default()
        };

        model.insert(db).await?;
        Ok(true)
    }

    /// 逻辑删除
    pub async fn delete_by_id(db: &DatabaseConnection, id: i32) -> Result<bool> {
        let result = store_seckill_manger::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 更新秒杀配置
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &StoreSeckillMangerRequest,
    ) -> Result<bool> {
        let (start_time, end_time) = Self::parse_time_range(&request.time)?;

        // 检查时间段是否已存在
        let existing = Self::check_time_range_unique(db, start_time, end_time).await?;
        if existing.len() > 1 {
            return Err(Error::string("当前时间段的秒杀配置已存在"));
        } else if existing.len() == 1 {
            // 检查是否是自身
            let contained = store_seckill_manger::Entity::find()
                .filter(store_seckill_manger::Column::StartTime.gte(start_time))
                .filter(store_seckill_manger::Column::EndTime.lte(end_time))
                .all(db)
                .await?;
            let enlarged = store_seckill_manger::Entity::find()
                .filter(store_seckill_manger::Column::StartTime.lte(start_time))
                .filter(store_seckill_manger::Column::EndTime.gte(end_time))
                .all(db)
                .await?;
            if contained.len() > 1 || enlarged.len() > 1 {
                return Err(Error::string("当前时间段的秒杀配置已存在"));
            }
        }

        Self::update_by_condition(db, id, request, start_time, end_time).await
    }

    /// 详情
    pub async fn detail(db: &DatabaseConnection, id: i32) -> Result<StoreSeckillMangerResponse> {
        let model = store_seckill_manger::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("秒杀配置不存在"))?;

        Ok(Self::model_to_response(&model))
    }

    /// 更新状态
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: &str,
    ) -> Result<bool> {
        let now = chrono::Local::now().naive_local();
        let model = store_seckill_manger::ActiveModel {
            id: Set(id),
            status: Set(Some(status.to_string())),
            update_time: Set(Some(now)),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }

    /// 获取当前正在秒杀的时间段
    pub async fn get_current_seckill_manager(
        db: &DatabaseConnection,
    ) -> Result<Vec<store_seckill_manger::Model>> {
        let current_hour = chrono::Local::now().hour() as i32;
        let list = store_seckill_manger::Entity::find()
            .filter(store_seckill_manger::Column::StartTime.lte(current_hour))
            .filter(store_seckill_manger::Column::EndTime.gt(current_hour))
            .all(db)
            .await?;
        Ok(list)
    }

    /// 获取所有秒杀配置
    pub async fn get_all_list(
        db: &DatabaseConnection,
    ) -> Result<Vec<StoreSeckillMangerResponse>> {
        let list = store_seckill_manger::Entity::find()
            .order_by_asc(store_seckill_manger::Column::Sort)
            .all(db)
            .await?;

        if list.is_empty() {
            return Ok(vec![]);
        }

        Ok(list.iter().map(|m| Self::model_to_response(m)).collect())
    }

    // ==================== 私有方法 ====================

    /// 检查时间段是否已存在
    async fn check_time_range_unique(
        db: &DatabaseConnection,
        start_time: i32,
        end_time: i32,
    ) -> Result<Vec<store_seckill_manger::Model>> {
        let list = store_seckill_manger::Entity::find()
            .filter(
                Condition::any()
                    // 开始时间在范围内
                    .add(
                        Condition::all()
                            .add(store_seckill_manger::Column::StartTime.gte(start_time))
                            .add(store_seckill_manger::Column::StartTime.lt(end_time))
                    )
                    // 包含整个范围
                    .add(
                        Condition::all()
                            .add(store_seckill_manger::Column::StartTime.lte(start_time))
                            .add(store_seckill_manger::Column::EndTime.gte(end_time))
                    )
                    // 结束时间在范围内
                    .add(
                        Condition::all()
                            .add(store_seckill_manger::Column::EndTime.gt(start_time))
                            .add(store_seckill_manger::Column::EndTime.lte(end_time))
                    )
            )
            .all(db)
            .await?;
        Ok(list)
    }

    /// 按条件更新
    async fn update_by_condition(
        db: &DatabaseConnection,
        id: i32,
        request: &StoreSeckillMangerRequest,
        start_time: i32,
        end_time: i32,
    ) -> Result<bool> {
        let now = chrono::Local::now().naive_local();
        let model = store_seckill_manger::ActiveModel {
            id: Set(id),
            name: Set(Some(request.name.clone())),
            start_time: Set(Some(start_time)),
            end_time: Set(Some(end_time)),
            img: Set(request.img.clone()),
            silder_imgs: Set(request.silder_imgs.clone()),
            sort: Set(request.sort),
            status: Set(request.status.clone()),
            update_time: Set(Some(now)),
            ..Default::default()
        };
        model.update(db).await?;
        Ok(true)
    }

    /// 解析时间范围字符串 "HH:00,HH:00" -> (start_hour, end_hour)
    fn parse_time_range(time: &str) -> Result<(i32, i32)> {
        if !time.contains(',') {
            return Err(Error::string("时间参数不正确 例如:01:00,02:00"));
        }
        let parts: Vec<&str> = time.split(',').collect();
        let start_parts: Vec<&str> = parts[0].split(':').collect();
        let end_parts: Vec<&str> = parts[1].split(':').collect();
        let start_time: i32 = start_parts[0].parse().map_err(|_| Error::string("时间参数不正确"))?;
        let end_time: i32 = end_parts[0].parse().map_err(|_| Error::string("时间参数不正确"))?;
        Ok((start_time, end_time))
    }

    /// Model -> Response 转换
    fn model_to_response(model: &store_seckill_manger::Model) -> StoreSeckillMangerResponse {
        let current_hour = chrono::Local::now().hour() as i32;
        let start_time = model.start_time.unwrap_or(0);
        let end_time = model.end_time.unwrap_or(0);

        // 格式化时间
        let start_str = if start_time < 10 {
            format!("0{}", start_time)
        } else {
            start_time.to_string()
        };
        let end_str = if end_time < 10 {
            format!("0{}", end_time)
        } else {
            end_time.to_string()
        };
        let time = format!("{}:00,{}:00", start_str, end_str);

        // 计算状态
        let status_val = model.status.as_deref().unwrap_or("0");
        let (status_name, kill_status) = if status_val == "0" {
            ("已关闭".to_string(), 0)
        } else if current_hour < start_time {
            ("未开始".to_string(), 1)
        } else if current_hour >= start_time && current_hour < end_time {
            ("进行中".to_string(), 2)
        } else {
            ("已结束".to_string(), -1)
        };

        StoreSeckillMangerResponse {
            id: model.id,
            name: model.name.clone(),
            start_time: model.start_time,
            end_time: model.end_time,
            img: model.img.clone(),
            silder_imgs: model.silder_imgs.clone(),
            sort: model.sort,
            status: model.status.clone(),
            is_del: model.is_del,
            time: Some(time),
            status_name,
            kill_status,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
