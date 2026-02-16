/// 秒杀管理 -- 控制器
///
/// 包含两个Java控制器:
/// - StoreSeckillController (秒杀商品)
/// - StoreSeckillMangerController (秒杀时间段配置)
use loco_rs::prelude::*;
use serde::Deserialize;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::store_seckill::*;
use crate::services::store_seckill_service::StoreSeckillService;
use crate::services::store_seckill_manger_service::StoreSeckillMangerService;

// ==================== 秒杀商品接口 ====================

/// 分页列表
/// GET /api/admin/store/seckill/list
#[debug_handler]
async fn seckill_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreSeckillSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreSeckillService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增秒杀商品
/// POST /api/admin/store/seckill/save
#[debug_handler]
async fn seckill_save(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreSeckillAddRequest>,
) -> Result<Response> {
    if StoreSeckillService::save_seckill(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

/// 删除秒杀商品
/// GET /api/admin/store/seckill/delete?id=xxx
#[debug_handler]
async fn seckill_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if StoreSeckillService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改秒杀商品
/// POST /api/admin/store/seckill/update
#[debug_handler]
async fn seckill_update(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreSeckillAddRequest>,
) -> Result<Response> {
    if StoreSeckillService::update_seckill(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 修改秒杀商品状态
/// POST /api/admin/store/seckill/update/status?id=xxx&status=xxx
#[debug_handler]
async fn seckill_update_status(
    State(ctx): State<AppContext>,
    Query(params): Query<SeckillStatusRequest>,
) -> Result<Response> {
    let status = params.status != 0;
    if StoreSeckillService::update_status(&ctx.db, params.id, status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 秒杀商品详情（管理端）
/// GET /api/admin/store/seckill/info?id=xxx
#[debug_handler]
async fn seckill_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let info = StoreSeckillService::get_detail_admin(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(info))
}

// ==================== 秒杀时间段配置接口 ====================

/// 分页列表
/// GET /api/admin/store/seckill/manger/list
#[debug_handler]
async fn manger_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreSeckillMangerSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreSeckillMangerService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增秒杀配置
/// POST /api/admin/store/seckill/manger/save
#[debug_handler]
async fn manger_save(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreSeckillMangerRequest>,
) -> Result<Response> {
    if StoreSeckillMangerService::save_manger(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

/// 删除秒杀配置
/// GET /api/admin/store/seckill/manger/delete?id=xxx
#[debug_handler]
async fn manger_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if StoreSeckillMangerService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 修改秒杀配置
/// POST /api/admin/store/seckill/manger/update?id=xxx
#[debug_handler]
async fn manger_update(
    State(ctx): State<AppContext>,
    Query(id_query): Query<IdQuery>,
    Json(request): Json<StoreSeckillMangerRequest>,
) -> Result<Response> {
    if StoreSeckillMangerService::update(&ctx.db, id_query.id, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 秒杀配置详情
/// GET /api/admin/store/seckill/manger/info?id=xxx
#[debug_handler]
async fn manger_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let info = StoreSeckillMangerService::detail(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(info))
}

/// 更新秒杀配置状态
/// POST /api/admin/store/seckill/manger/update/status?id=xxx&status=xxx
#[derive(Debug, Deserialize)]
struct MangerStatusQuery {
    id: i32,
    status: String,
}

#[debug_handler]
async fn manger_update_status(
    State(ctx): State<AppContext>,
    Query(params): Query<MangerStatusQuery>,
) -> Result<Response> {
    if StoreSeckillMangerService::update_status(&ctx.db, params.id, &params.status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

// ==================== 路由注册 ====================

/// 秒杀商品路由
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/seckill")
        // 秒杀商品接口
        .add("/list", get(seckill_list))
        .add("/info", get(seckill_info))
        .add("/delete", get(seckill_delete))
        .add("/save", post(seckill_save))
        .add("/update", post(seckill_update))
        .add("/update/status", post(seckill_update_status))
        // 秒杀时间段配置接口
        .add("/manger/list", get(manger_list))
        .add("/manger/info", get(manger_info))
        .add("/manger/delete", get(manger_delete))
        .add("/manger/save", post(manger_save))
        .add("/manger/update", post(manger_update))
        .add("/manger/update/status", post(manger_update_status))
}
