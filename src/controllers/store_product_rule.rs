/// 商品规格管理 -- 控制器
///
/// 实现与Java版本一致的商品规格管理接口
/// Java代码参考: com.zbkj.admin.controller.StoreProductRuleController
use loco_rs::prelude::*;
use axum::Json;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, IdsQuery, PageParamRequest};
use crate::dtos::product_rule::*;
use crate::services::store_product_rule_service::StoreProductRuleService;
use crate::utils::auth;

// ==================== 接口实现 ====================

/// 1. 分页列表
///
/// 权限: admin:product:rule:list
/// 路径: GET /api/admin/store/product/rule/list
/// Java: StoreProductRuleController.getList()
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(search): Query<StoreProductRuleSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:rule:list").await?;

    let response = StoreProductRuleService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 2. 新增规格
///
/// 权限: admin:product:rule:save
/// 路径: POST /api/admin/store/product/rule/save
/// Java: StoreProductRuleController.save()
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<StoreProductRuleRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:rule:save").await?;

    if StoreProductRuleService::save(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("新增失败"))
    }
}

/// 3. 删除规格
///
/// 权限: admin:product:rule:delete
/// 路径: GET /api/admin/store/product/rule/delete
/// Java: StoreProductRuleController.delete() - 原路径 /delete/{ids}
/// 注意：Loco框架不支持路径参数，改为查询参数 ?ids=1,2,3
#[debug_handler]
async fn delete(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdsQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:rule:delete").await?;

    let ids: Vec<i32> = params.ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if StoreProductRuleService::delete_by_ids(&ctx.db, ids).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

/// 4. 修改规格
///
/// 权限: admin:product:rule:update
/// 路径: POST /api/admin/store/product/rule/update
/// Java: StoreProductRuleController.update()
#[debug_handler]
async fn update(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Json(request): Json<StoreProductRuleRequest>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:rule:update").await?;

    if StoreProductRuleService::update_rule(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改失败"))
    }
}

/// 5. 规格详情
///
/// 权限: admin:product:rule:info
/// 路径: GET /api/admin/store/product/rule/info
/// Java: StoreProductRuleController.info() - 原路径 /info/{id}
/// 注意：Loco框架不支持路径参数，改为查询参数 ?id=1
#[debug_handler]
async fn info(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:product:rule:info").await?;

    let rule = StoreProductRuleService::get_by_id(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(rule))
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/product/rule")
        .add("/list", get(get_list))
        .add("/delete", get(delete))
        .add("/info", get(info))
        .add("/save", post(save))
        .add("/update", post(update))
}
