/// 定时任务 -- 服务层
///
/// Java参考: ScheduleJobServiceImpl, ScheduleJobLogServiceImpl
/// 注意: Rust版本不包含Quartz调度器集成，仅管理数据库记录和状态
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::dtos::schedule_job::*;
use crate::dtos::common::PageParamRequest;
use crate::common::pagination::PageResponse;
use crate::models::_entities::{schedule_job, schedule_job_log};

/// 状态常量: 0=正常, 1=暂停
const STATUS_NORMAL: i16 = 0;
const STATUS_PAUSE: i16 = 1;

pub struct ScheduleJobService;

impl ScheduleJobService {
    /// 获取所有定时任务(未删除)
    /// Java: ScheduleJobServiceImpl.getAll()
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<ScheduleJobResponse>> {
        let list = schedule_job::Entity::find()
            .filter(schedule_job::Column::IsDelte.eq(0i16))
            .order_by_desc(schedule_job::Column::JobId)
            .all(db)
            .await?;
        Ok(list.into_iter().map(Self::job_to_response).collect())
    }

    /// 添加定时任务
    /// Java: ScheduleJobServiceImpl.add()
    pub async fn add(db: &DatabaseConnection, request: &ScheduleJobRequest) -> Result<bool> {
        let now = Local::now().naive_local();
        let model = schedule_job::ActiveModel {
            bean_name: Set(Some(request.bean_name.clone())),
            method_name: Set(Some(request.method_name.clone())),
            params: Set(request.params.clone()),
            cron_expression: Set(Some(request.cron_expression.clone())),
            status: Set(Some(STATUS_PAUSE)),
            remark: Set(request.remark.clone()),
            is_delte: Set(Some(0i16)),
            create_time: Set(now),
            ..Default::default()
        };
        model.insert(db).await?;
        Ok(true)
    }

    /// 编辑定时任务
    /// Java: ScheduleJobServiceImpl.edit()
    pub async fn edit(db: &DatabaseConnection, request: &ScheduleJobRequest) -> Result<bool> {
        let job_id = request.job_id
            .ok_or_else(|| Error::BadRequest("定时任务ID不能为空".to_string()))?;
        let job = Self::get_by_id(db, job_id).await?;
        if job.status == Some(STATUS_NORMAL) {
            return Err(Error::BadRequest("请先暂停定时任务".to_string()));
        }
        let mut active: schedule_job::ActiveModel = job.into();
        active.bean_name = Set(Some(request.bean_name.clone()));
        active.method_name = Set(Some(request.method_name.clone()));
        active.params = Set(request.params.clone());
        active.cron_expression = Set(Some(request.cron_expression.clone()));
        active.remark = Set(request.remark.clone());
        active.update(db).await?;
        Ok(true)
    }

    /// 暂停定时任务
    /// Java: ScheduleJobServiceImpl.suspend()
    pub async fn suspend(db: &DatabaseConnection, job_id: i32) -> Result<bool> {
        let job = Self::get_by_id(db, job_id).await?;
        if job.status == Some(STATUS_PAUSE) {
            return Err(Error::BadRequest("定时任务已暂停，请勿重复操作".to_string()));
        }
        let mut active: schedule_job::ActiveModel = job.into();
        active.status = Set(Some(STATUS_PAUSE));
        active.update(db).await?;
        Ok(true)
    }

    /// 启动定时任务
    /// Java: ScheduleJobServiceImpl.start()
    pub async fn start(db: &DatabaseConnection, job_id: i32) -> Result<bool> {
        let job = Self::get_by_id(db, job_id).await?;
        if job.status == Some(STATUS_NORMAL) {
            return Err(Error::BadRequest("定时任务已启动，请勿重复操作".to_string()));
        }
        let mut active: schedule_job::ActiveModel = job.into();
        active.status = Set(Some(STATUS_NORMAL));
        active.update(db).await?;
        Ok(true)
    }

    /// 删除定时任务(软删除)
    /// Java: ScheduleJobServiceImpl.delete()
    pub async fn delete(db: &DatabaseConnection, job_id: i32) -> Result<bool> {
        let job = Self::get_by_id(db, job_id).await?;
        if job.status == Some(STATUS_NORMAL) {
            return Err(Error::BadRequest("请先暂停定时任务".to_string()));
        }
        let mut active: schedule_job::ActiveModel = job.into();
        active.is_delte = Set(Some(1i16));
        active.update(db).await?;
        Ok(true)
    }

    /// 立即执行定时任务(一次)
    /// Java: ScheduleJobServiceImpl.trig()
    /// 注意: Rust版本无Quartz调度器，此处仅验证任务存在
    pub async fn trig(db: &DatabaseConnection, job_id: i32) -> Result<bool> {
        Self::get_by_id(db, job_id).await?;
        // Rust版本无Quartz调度器，返回成功
        Ok(true)
    }

    /// 根据ID获取任务
    async fn get_by_id(db: &DatabaseConnection, job_id: i32) -> Result<schedule_job::Model> {
        let job = schedule_job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("定时任务不存在".to_string()))?;
        if job.is_delte == Some(1i16) {
            return Err(Error::BadRequest("定时任务不存在".to_string()));
        }
        Ok(job)
    }

    fn job_to_response(model: schedule_job::Model) -> ScheduleJobResponse {
        ScheduleJobResponse {
            job_id: model.job_id,
            bean_name: model.bean_name,
            method_name: model.method_name,
            params: model.params,
            cron_expression: model.cron_expression,
            status: model.status.map(|s| s as i32),
            remark: model.remark,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// ==================== 日志服务 ====================

pub struct ScheduleJobLogService;

impl ScheduleJobLogService {
    /// 日志分页列表
    /// Java: ScheduleJobLogServiceImpl.findLogPageList()
    pub async fn find_log_page_list(
        db: &DatabaseConnection,
        request: &ScheduleJobLogSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<PageResponse<ScheduleJobLogResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = schedule_job_log::Entity::find();

        if let Some(job_id) = request.job_id {
            query = query.filter(schedule_job_log::Column::JobId.eq(job_id));
        }
        if let Some(bean_name) = &request.bean_name {
            if !bean_name.trim().is_empty() {
                query = query.filter(schedule_job_log::Column::BeanName.eq(bean_name.trim()));
            }
        }
        if let Some(method_name) = &request.method_name {
            if !method_name.trim().is_empty() {
                query = query.filter(schedule_job_log::Column::MethodName.eq(method_name.trim()));
            }
        }
        if let Some(status) = request.status {
            query = query.filter(schedule_job_log::Column::Status.eq(status as i16));
        }

        query = query.order_by_desc(schedule_job_log::Column::LogId);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list = records.into_iter().map(Self::log_to_response).collect();
        Ok(PageResponse::new(list, total, page as u64, limit as u64))
    }

    fn log_to_response(model: schedule_job_log::Model) -> ScheduleJobLogResponse {
        ScheduleJobLogResponse {
            log_id: model.log_id,
            job_id: model.job_id,
            bean_name: model.bean_name,
            method_name: model.method_name,
            params: model.params,
            status: model.status as i32,
            error: model.error,
            times: model.times,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
