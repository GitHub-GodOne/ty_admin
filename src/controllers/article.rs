/// 文章管理 -- 控制器
///
/// 实现与Java版本一致的文章管理接口
/// Java代码参考: com.zbkj.admin.controller.ArticleController
/// 路由前缀: /api/admin/article
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::article::*;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::services::article_service::ArticleService;
use crate::utils::auth;

/// 1. 管理端文章分页列表
///
/// 权限: admin:article:list
/// 路径: GET /api/admin/article/list
/// Java: ArticleController.getList(ArticleSearchRequest, PageParamRequest)
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(request): Query<ArticleSearchRequest>,
    Query(page_param): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:article:list").await?;

    let response = ArticleService::get_admin_list(&ctx.db, &request, &page_param).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增文章
///
/// 权限: admin:article:save
/// 路径: POST /api/admin/article/save
/// Java: ArticleController.save(@Validated @RequestBody ArticleRequest)
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(request): axum::Json<ArticleRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:article:save").await?;

    if ArticleService::create(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增失败"))
    }
}

/// 3. 删除文章
///
/// 权限: admin:article:delete
/// 路径: GET /api/admin/article/delete
/// Java: ArticleController.delete(@RequestParam Integer id)
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:article:delete").await?;

    if ArticleService::delete_by_id(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 4. 修改文章
///
/// 权限: admin:article:update
/// 路径: POST /api/admin/article/update
/// Java: ArticleController.update(@RequestParam Integer id, @Validated @RequestBody ArticleRequest)
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
    axum::Json(request): axum::Json<ArticleRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:article:update").await?;

    if ArticleService::update_article(&ctx.db, params.id, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 5. 文章详情
///
/// 权限: admin:article:info
/// 路径: GET /api/admin/article/info
/// Java: ArticleController.info(@RequestParam Integer id)
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:article:info").await?;

    let response = ArticleService::get_detail(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/article")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
}
