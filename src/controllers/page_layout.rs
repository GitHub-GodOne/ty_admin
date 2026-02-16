/// 页面布局管理 -- 控制器
///
/// Java代码参考: com.zbkj.admin.controller.PageLayoutController
/// 路由前缀: /api/admin/page/layout
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::services::page_layout_service::PageLayoutService;
use crate::utils::auth;

/// 1. 页面首页
///
/// 权限: admin:page:layout:index
/// 路径: GET /api/admin/page/layout/index
#[debug_handler]
async fn index(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:index").await?;

    let response = PageLayoutService::index(&ctx.db)
        .await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 2. 页面首页保存（全量）
///
/// 权限: admin:page:layout:save
/// 路径: POST /api/admin/page/layout/save
#[debug_handler]
async fn save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:save").await?;

    if PageLayoutService::save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}
/// 3. 页面首页banner保存
///
/// 权限: admin:page:layout:index:banner:save
/// 路径: POST /api/admin/page/layout/index/banner/save
#[debug_handler]
async fn index_banner_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:index:banner:save").await?;

    if PageLayoutService::index_banner_save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}

/// 4. 页面首页menu保存
///
/// 权限: admin:page:layout:index:menu:save
/// 路径: POST /api/admin/page/layout/index/menu/save
#[debug_handler]
async fn index_menu_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:index:menu:save").await?;

    if PageLayoutService::index_menu_save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}

/// 5. 页面首页新闻保存
///
/// 权限: admin:page:layout:index:news:save
/// 路径: POST /api/admin/page/layout/index/news/save
#[debug_handler]
async fn index_news_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:index:news:save").await?;

    if PageLayoutService::index_news_save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}
/// 6. 页面用户中心banner保存
///
/// 权限: admin:page:layout:index:banner:save
/// 路径: POST /api/admin/page/layout/user/banner/save
#[debug_handler]
async fn user_banner_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:index:banner:save").await?;

    if PageLayoutService::user_banner_save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}

/// 7. 页面用户中心导航保存
///
/// 权限: admin:page:layout:user:menu:save
/// 路径: POST /api/admin/page/layout/user/menu/save
#[debug_handler]
async fn user_menu_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:user:menu:save").await?;

    if PageLayoutService::user_menu_save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}

/// 8. 页面用户中心商品table保存
///
/// 权限: admin:page:layout:index:table:save
/// 路径: POST /api/admin/page/layout/index/table/save
#[debug_handler]
async fn index_table_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:index:table:save").await?;

    if PageLayoutService::index_table_save(&ctx.db, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}
/// 9. 页面底部导航
///
/// 权限: admin:page:layout:bottom:navigation
/// 路径: GET /api/admin/page/layout/bottom/navigation/get
#[debug_handler]
async fn get_bottom_navigation(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:bottom:navigation").await?;

    let response = PageLayoutService::get_bottom_navigation(&ctx.db)
        .await.map_err(|e| Error::string(&e.to_string()))?;
    format::json(ApiResponse::success(response))
}

/// 10. 底部导航保存
///
/// 权限: admin:page:layout:bottom:navigation:save
/// 路径: POST /api/admin/page/layout/bottom/navigation/save
#[debug_handler]
async fn bottom_navigation_save(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    axum::Json(json): axum::Json<serde_json::Value>,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:page:layout:bottom:navigation:save").await?;

    let redis = crate::initializers::redis::get_redis().await
        .map_err(|e| Error::string(&e.to_string()))?;
    if PageLayoutService::bottom_navigation_save(&ctx.db, &redis, &json)
        .await.map_err(|e| Error::string(&e.to_string()))? {
        format::json(ApiResponse::<String>::success_empty())
    } else {
        format::json(ApiResponse::<String>::failed("保存失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/page/layout")
        .add("/index", get(index))
        .add("/save", post(save))
        .add("/index/banner/save", post(index_banner_save))
        .add("/index/menu/save", post(index_menu_save))
        .add("/index/news/save", post(index_news_save))
        .add("/user/banner/save", post(user_banner_save))
        .add("/user/menu/save", post(user_menu_save))
        .add("/index/table/save", post(index_table_save))
        .add("/bottom/navigation/get", get(get_bottom_navigation))
        .add("/bottom/navigation/save", post(bottom_navigation_save))
}
