/// 定时任务 -- 控制器
///
/// Java参考: ScheduleJobController
/// 路径前缀: /api/admin/schedule/job
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::schedule_job::*;
use crate::services::schedule_job_service::{ScheduleJobService, ScheduleJobLogService};

/// 定时任务列表
/// GET /api/admin/schedule/job/list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let response = ScheduleJobService::get_all(&ctx.db).await?;
    format::json(ApiResponse::success(response))
}

/// 添加定时任务
/// POST /api/admin/schedule/job/add
#[debug_handler]
async fn add(
    State(ctx): State<AppContext>,
    Json(request): Json<ScheduleJobRequest>,
) -> Result<Response> {
    if ScheduleJobService::add(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("添加失败"))
    }
}

/// 编辑定时任务
/// POST /api/admin/schedule/job/update
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    Json(request): Json<ScheduleJobRequest>,
) -> Result<Response> {
    if ScheduleJobService::edit(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("编辑失败"))
    }
}

/// 暂停定时任务
/// POST /api/admin/schedule/job/suspend?jobId=
#[debug_handler]
async fn suspend(
    State(ctx): State<AppContext>,
    Query(params): Query<JobIdQuery>,
) -> Result<Response> {
    if ScheduleJobService::suspend(&ctx.db, params.job_id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("暂停失败"))
    }
}

/// 启动定时任务
/// POST /api/admin/schedule/job/start?jobId=
#[debug_handler]
async fn start(
    State(ctx): State<AppContext>,
    Query(params): Query<JobIdQuery>,
) -> Result<Response> {
    if ScheduleJobService::start(&ctx.db, params.job_id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("启动失败"))
    }
}

/// 删除定时任务
/// POST /api/admin/schedule/job/delete?jobId=
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    Query(params): Query<JobIdQuery>,
) -> Result<Response> {
    if ScheduleJobService::delete(&ctx.db, params.job_id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 立即执行定时任务(一次)
/// POST /api/admin/schedule/job/trig?jobId=
#[debug_handler]
async fn trig(
    State(ctx): State<AppContext>,
    Query(params): Query<JobIdQuery>,
) -> Result<Response> {
    if ScheduleJobService::trig(&ctx.db, params.job_id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("执行失败"))
    }
}

/// 定时任务日志分页列表
/// GET /api/admin/schedule/job/log/list
#[debug_handler]
async fn log_list(
    State(ctx): State<AppContext>,
    Query(search): Query<ScheduleJobLogSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = ScheduleJobLogService::find_log_page_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/schedule/job")
        .add("/list", get(get_list))
        .add("/add", post(add))
        .add("/update", post(update))
        .add("/suspend", post(suspend))
        .add("/start", post(start))
        .add("/delete", post(delete))
        .add("/trig", post(trig))
        .add("/log/list", get(log_list))
}
