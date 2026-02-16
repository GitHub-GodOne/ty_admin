/// 附件管理 -- 控制器
///
/// 实现与Java版本一致的附件管理接口
/// Java代码参考: com.zbkj.admin.controller.SystemAttachmentController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, IdsQuery, PageParamRequest};
use crate::dtos::system_attachment::*;
use crate::services::system_attachment_service::SystemAttachmentService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分页列表
///
/// 权限: admin:system:attachment:list
/// 路径: GET /api/admin/system/attachment/list
/// Java: SystemAttachmentController.getList()
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<SystemAttachmentListQuery>,
) -> Result<Response> {
    tracing::info!("=== SystemAttachment get_list called, pid={}, att_type={:?}", params.pid, params.att_type);

    auth::check_permission(&headers, "admin:system:attachment:list").await?;

    let page_param = PageParamRequest {
        page: params.page,
        limit: params.limit,
    };
    let response = SystemAttachmentService::get_list(
        &ctx.db, params.pid, &params.att_type, &page_param,
    ).await
    .map_err(|e| {
        tracing::error!("SystemAttachment get_list DB error: {:?}", e);
        e
    })?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增附件
///
/// 权限: admin:system:attachment:save
/// 路径: POST /api/admin/system/attachment/save
/// Java: SystemAttachmentController.save()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemAttachmentRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:attachment:save").await?;

    if SystemAttachmentService::add(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增失败"))
    }
}

/// 3. 删除附件
///
/// 权限: admin:system:attachment:delete
/// 路径: GET /api/admin/system/attachment/delete
/// Java: SystemAttachmentController.delete() - 原路径 /delete/{ids}
/// 注意：Loco框架不支持路径参数，改为查询参数 ?ids=1,2,3
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdsQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:attachment:delete").await?;

    let ids: Vec<i32> = params.ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if SystemAttachmentService::delete_by_ids(&ctx.db, ids).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 4. 修改附件
///
/// 权限: admin:system:attachment:update
/// 路径: POST /api/admin/system/attachment/update
/// Java: SystemAttachmentController.update()
/// 注意：Java中id通过@RequestParam传递，这里通过查询参数
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(id_query): Query<IdQuery>,
    Json(request): Json<SystemAttachmentRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:attachment:update").await?;

    if SystemAttachmentService::edit(&ctx.db, id_query.id, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 5. 更改图片目录（移动附件）
///
/// 权限: admin:system:attachment:move
/// 路径: POST /api/admin/system/attachment/move
/// Java: SystemAttachmentController.updateAttrId()
#[debug_handler]
async fn move_attachment(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<SystemAttachmentMoveRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:attachment:move").await?;

    if SystemAttachmentService::update_attr_id(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("移动失败"))
    }
}

/// 6. 附件详情
///
/// 权限: admin:system:attachment:info
/// 路径: GET /api/admin/system/attachment/info
/// Java: SystemAttachmentController.info() - 原路径 /info/{id}
/// 注意：Loco框架不支持路径参数，改为查询参数 ?id=1
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:system:attachment:info").await?;

    let attachment = SystemAttachmentService::get_by_id(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(attachment))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/system/attachment")
        .add("/list", get(get_list))
        .add("/delete", get(delete))
        .add("/info", get(info))
        .add("/save", post(save))
        .add("/update", post(update))
        .add("/move", post(move_attachment))
}
