/// 微信关键字回复 -- 控制器
///
/// 实现与Java版本一致的微信关键字回复接口
/// Java代码参考: com.zbkj.admin.controller.WechatReplyController
/// 路由前缀: /api/admin/wechat/keywords/reply
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::wechat_reply::*;
use crate::services::wechat_reply_service::WechatReplyService;
use crate::utils::auth;

/// 关键字查询参数
#[derive(Debug, serde::Deserialize)]
struct KeywordsQuery {
    keywords: String,
}

/// 1. 分页列表
///
/// 权限: admin:wechat:keywords:reply:list
/// 路径: GET /api/admin/wechat/keywords/reply/list
/// Java: WechatReplyController.getList(WechatReplySearchRequest, PageParamRequest)
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<WechatReplySearchRequest>,
    Query(page_param): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:list").await?;

    let response = WechatReplyService::get_list(&ctx.db, &request, &page_param).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增
///
/// 权限: admin:wechat:keywords:reply:save
/// 路径: POST /api/admin/wechat/keywords/reply/save
/// Java: WechatReplyController.save(@RequestBody @Validated WechatReplyRequest)
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<WechatReplyRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:save").await?;

    if WechatReplyService::create(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增失败"))
    }
}

/// 3. 删除
///
/// 权限: admin:wechat:keywords:reply:delete
/// 路径: GET /api/admin/wechat/keywords/reply/delete
/// Java: WechatReplyController.delete(@RequestParam Integer id)
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:delete").await?;

    if WechatReplyService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 4. 修改
///
/// 权限: admin:wechat:keywords:reply:update
/// 路径: POST /api/admin/wechat/keywords/reply/update
/// Java: WechatReplyController.update(@RequestBody @Validated WechatReplyRequest)
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<WechatReplyRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:update").await?;

    if WechatReplyService::update_reply(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 5. 修改状态
///
/// 权限: admin:wechat:keywords:reply:status
/// 路径: POST /api/admin/wechat/keywords/reply/status
/// Java: WechatReplyController.update(@RequestParam Integer id, @RequestParam Boolean status)
#[debug_handler]
async fn update_status(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<WechatReplyStatusRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:status").await?;

    if WechatReplyService::update_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 6. 详情
///
/// 权限: admin:wechat:keywords:reply:info
/// 路径: GET /api/admin/wechat/keywords/reply/info
/// Java: WechatReplyController.info(@RequestParam Integer id)
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:info").await?;

    let response = WechatReplyService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

/// 7. 根据关键字查询数据
///
/// 权限: admin:wechat:keywords:reply:info:keywords
/// 路径: GET /api/admin/wechat/keywords/reply/info/keywords
/// Java: WechatReplyController.info(@RequestParam String keywords)
#[debug_handler]
async fn info_by_keywords(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<KeywordsQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:keywords:reply:info:keywords").await?;

    let response = WechatReplyService::get_by_keywords_response(&ctx.db, &params.keywords).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/wechat/keywords/reply")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/status", post(update_status))
        .add("/info", get(info))
        .add("/info/keywords", get(info_by_keywords))
}
