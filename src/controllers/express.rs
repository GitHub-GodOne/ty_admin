/// 快递公司 -- 控制器
///
/// 实现与Java版本一致的快递公司管理接口
/// Java代码参考: com.zbkj.admin.controller.ExpressController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::express::*;
use crate::services::express_service::ExpressService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分页列表
///
/// 权限: admin:express:list
/// 路径: GET /api/admin/express/list
/// Java: ExpressController.getList()
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<ExpressSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:list").await?;

    let response = ExpressService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 编辑快递公司
///
/// 权限: admin:express:update
/// 路径: POST /api/admin/express/update
/// Java: ExpressController.update(ExpressUpdateRequest)
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<ExpressUpdateRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:update").await?;

    if ExpressService::update_express(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("编辑失败"))
    }
}

/// 3. 修改显示状态
///
/// 权限: admin:express:update:show
/// 路径: POST /api/admin/express/update/show
/// Java: ExpressController.update(ExpressUpdateShowRequest)
#[debug_handler]
async fn update_show(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<ExpressUpdateShowRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:update:show").await?;

    if ExpressService::update_express_show(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改显示状态失败"))
    }
}

/// 4. 同步物流公司
///
/// 权限: admin:express:sync
/// 路径: POST /api/admin/express/sync/express
/// Java: ExpressController.syncExpress()
#[debug_handler]
async fn sync_express(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:sync").await?;

    if ExpressService::sync_express(&ctx.db).await? {
        format::json(ApiResponse::success_with_message(
            (),
            "同步物流公司成功",
        ))
    } else {
        format::json(ApiResponse::<()>::failed("同步物流公司失败"))
    }
}

/// 5. 快递公司详情
///
/// 权限: admin:express:info
/// 路径: GET /api/admin/express/info
/// Java: ExpressController.info(Integer id)
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:info").await?;

    let response = ExpressService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(response))
}

/// 6. 查询全部物流公司
///
/// 权限: admin:express:all
/// 路径: GET /api/admin/express/all
/// Java: ExpressController.all(String type)
/// 参数: type - "normal" 普通, "elec" 电子面单
#[debug_handler]
async fn all(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<ExpressAllQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:all").await?;

    let response = ExpressService::find_all(&ctx.db, &params.express_type).await?;
    format::json(ApiResponse::success(response))
}

/// 7. 查询物流公司面单模板
///
/// 权限: admin:express:template
/// 路径: GET /api/admin/express/template
/// Java: ExpressController.template(String com)
#[debug_handler]
async fn template(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<ExpressTemplateQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:express:template").await?;

    let response = ExpressService::template(&ctx.db, &params.com).await?;
    format::json(ApiResponse::success(response))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/express")
        .add("/list", get(get_list))
        .add("/update", post(update))
        .add("/update/show", post(update_show))
        .add("/sync/express", post(sync_express))
        .add("/info", get(info))
        .add("/all", get(all))
        .add("/template", get(template))
}
