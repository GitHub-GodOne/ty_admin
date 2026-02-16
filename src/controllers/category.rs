/// 分类管理 -- 控制器
///
/// 实现与Java版本一致的分类管理接口
/// Java代码参考: com.zbkj.admin.controller.CategoryController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::category::*;
use crate::dtos::common::{IdQuery, IdsQuery, PageParamRequest};
use crate::services::category_service::CategoryService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分类列表
///
/// 权限: admin:category:list
/// 路径: GET /api/admin/category/list
/// Java: CategoryController.getList()
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<CategorySearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:list").await?;

    let list = CategoryService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(list))
}

/// 2. 新增分类
///
/// 权限: admin:category:save
/// 路径: POST /api/admin/category/save
/// Java: CategoryController.save()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<CategoryRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:save").await?;

    CategoryService::create(&ctx.db, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 3. 删除分类
///
/// 权限: admin:category:delete
/// 路径: GET /api/admin/category/delete
/// Java: CategoryController.delete()
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:delete").await?;

    CategoryService::delete(&ctx.db, params.id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 4. 修改分类
///
/// 权限: admin:category:update
/// 路径: POST /api/admin/category/update
/// Java: CategoryController.update()
/// 注意：Java版本通过URL参数传递id
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(id_query): Query<IdQuery>,
    Json(request): Json<CategoryRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:update").await?;

    CategoryService::update(&ctx.db, id_query.id, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 5. 分类详情
///
/// 权限: admin:category:info
/// 路径: GET /api/admin/category/info
/// Java: CategoryController.info()
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:info").await?;

    let category = CategoryService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(category))
}

/// 6. 分类树形列表
///
/// 权限: admin:category:list:tree
/// 路径: GET /api/admin/category/list/tree
/// Java: CategoryController.getListTree()
#[debug_handler]
async fn get_list_tree(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<CategorySearchRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:list:tree").await?;

    let tree = CategoryService::get_list_tree(
        &ctx.db,
        search.category_type,
        search.status,
        search.name.as_deref(),
    ).await?;
    format::json(ApiResponse::success(tree))
}

/// 7. 根据ID列表获取分类
///
/// 权限: admin:category:list:ids
/// 路径: GET /api/admin/category/list/ids
/// Java: CategoryController.getByIds()
#[debug_handler]
async fn get_by_ids(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdsQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:list:ids").await?;

    let ids: Vec<i32> = params.ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let list = CategoryService::get_by_ids(&ctx.db, ids).await?;
    format::json(ApiResponse::success(list))
}

/// 8. 更新分类状态
///
/// 权限: admin:category:update:status
/// 路径: GET /api/admin/category/updateStatus
/// Java: CategoryController.updateStatus()
/// 注意：Java版本使用路径参数 /updateStatus/{id}，
/// 但Loco框架不支持路径参数，改为查询参数
#[debug_handler]
async fn update_status(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<UpdateStatusQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:category:update:status").await?;

    CategoryService::update_status(&ctx.db, params.id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/category")
        // GET 接口
        .add("/list", get(get_list))
        .add("/delete", get(delete))
        .add("/info", get(info))
        .add("/list/tree", get(get_list_tree))
        .add("/list/ids", get(get_by_ids))
        .add("/updateStatus", get(update_status))
        // POST 接口
        .add("/save", post(save))
        .add("/update", post(update))
}
