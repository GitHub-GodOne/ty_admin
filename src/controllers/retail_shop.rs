/// 分销管理 -- 控制器
///
/// Java参考: RetailShopController
/// 路径前缀: /api/admin/store/retail
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::retail_shop::*;
use crate::services::retail_shop_service::RetailShopService;

/// 分销员列表
/// GET /api/admin/store/retail/list
#[debug_handler]
async fn retail_list(
    State(ctx): State<AppContext>,
    Query(search): Query<RetailShopListRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = RetailShopService::get_spread_people_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 根据条件获取推广人列表
/// POST /api/admin/store/retail/spread/userlist
#[debug_handler]
async fn spread_user_list(
    State(ctx): State<AppContext>,
    Query(page): Query<PageParamRequest>,
    Json(request): Json<RetailShopStairUserRequest>,
) -> Result<Response> {
    let response = RetailShopService::get_user_list_by_spread_level(&ctx.db, &request, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 根据条件获取推广订单列表
/// POST /api/admin/store/retail/spread/orderlist
#[debug_handler]
async fn spread_order_list(
    State(ctx): State<AppContext>,
    Query(page): Query<PageParamRequest>,
    Json(request): Json<RetailShopStairUserRequest>,
) -> Result<Response> {
    let response = RetailShopService::get_order_list_by_spread_level(&ctx.db, &request, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 清除上级推广人
/// GET /api/admin/store/retail/spread/clean?id=xxx
#[debug_handler]
async fn spread_clean(
    State(ctx): State<AppContext>,
    Query(params): Query<CleanSpreadRequest>,
) -> Result<Response> {
    if RetailShopService::clear_spread(&ctx.db, params.id).await? {
        format::json(ApiResponse::<bool>::success(true))
    } else {
        format::json(ApiResponse::<()>::failed("清除推广关系失败"))
    }
}

/// 分销配置信息获取
/// GET /api/admin/store/retail/spread/manage/get
#[debug_handler]
async fn spread_manage_get(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let info = RetailShopService::get_manage_info(&ctx.db).await?;
    format::json(ApiResponse::success(info))
}

/// 分销管理信息保存
/// POST /api/admin/store/retail/spread/manage/set
#[debug_handler]
async fn spread_manage_set(
    State(ctx): State<AppContext>,
    Json(request): Json<RetailShopConfigRequest>,
) -> Result<Response> {
    // 校验返佣比例
    let ratio = request.store_brokerage_two + request.store_brokerage_ratio;
    if ratio > 100 || ratio < 0 {
        return format::json(ApiResponse::<()>::failed("返佣比例加起来不能超过100%"));
    }

    let redis = crate::initializers::redis::get_redis()
        .await
        .map_err(|e| loco_rs::Error::string(&e.to_string()))?;

    if RetailShopService::set_manage_info(&ctx.db, &redis, &request).await? {
        format::json(ApiResponse::<bool>::success(true))
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/retail")
        .add("/list", get(retail_list))
        .add("/spread/userlist", post(spread_user_list))
        .add("/spread/orderlist", post(spread_order_list))
        .add("/spread/clean", get(spread_clean))
        .add("/spread/manage/get", get(spread_manage_get))
        .add("/spread/manage/set", post(spread_manage_set))
}
