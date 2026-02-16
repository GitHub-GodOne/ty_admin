/// 商品评论管理 -- 控制器
///
/// 实现与Java版本一致的商品评论管理接口
/// Java代码参考: com.zbkj.admin.controller.StoreProductReplyController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::product_reply::*;
use crate::services::store_product_reply_service::StoreProductReplyService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分页列表
///
/// 权限: admin:product:reply:list
/// 路径: GET /api/admin/store/product/reply/list
/// Java: StoreProductReplyController.getList()
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<StoreProductReplySearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:reply:list").await?;

    let response = StoreProductReplyService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 虚拟评论（新增虚拟评论）
///
/// 权限: admin:product:reply:save
/// 路径: POST /api/admin/store/product/reply/save
/// Java: StoreProductReplyController.save() -> virtualCreate()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<StoreProductReplyAddRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:reply:save").await?;

    if StoreProductReplyService::virtual_create(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增虚拟评论失败"))
    }
}

/// 3. 删除评论（软删除）
///
/// 权限: admin:product:reply:delete
/// 路径: GET /api/admin/store/product/reply/delete
/// Java: StoreProductReplyController.delete() - 原路径 /delete/{id}
/// 注意：Loco框架不支持路径参数，改为查询参数 ?id=1
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:reply:delete").await?;

    if StoreProductReplyService::delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 4. 评论详情
///
/// 权限: admin:product:reply:info
/// 路径: GET /api/admin/store/product/reply/info
/// Java: StoreProductReplyController.info() - 原路径 /info/{id}
/// 注意：Loco框架不支持路径参数，改为查询参数 ?id=1
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:reply:info").await?;

    let reply = StoreProductReplyService::get_by_id(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(reply))
}

/// 5. 管理员回复评论
///
/// 权限: admin:product:reply:comment
/// 路径: POST /api/admin/store/product/reply/comment
/// Java: StoreProductReplyController.comment()
#[debug_handler]
async fn comment(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<StoreProductReplyCommentRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:reply:comment").await?;

    if StoreProductReplyService::comment(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("回复失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/product/reply")
        .add("/list", get(get_list))
        .add("/delete", get(delete))
        .add("/info", get(info))
        .add("/save", post(save))
        .add("/comment", post(comment))
}
