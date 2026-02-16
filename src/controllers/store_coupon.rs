/// 优惠券管理 -- 控制器
///
/// Java参考: StoreCouponController
/// 路径前缀: /api/admin/marketing/coupon
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::{IdQuery, PageParamRequest};
use crate::dtos::store_coupon::*;
use crate::services::store_coupon_service::StoreCouponService;

/// 分页列表
/// GET /api/admin/marketing/coupon/list
#[debug_handler]
async fn coupon_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreCouponSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreCouponService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 新增优惠券
/// POST /api/admin/marketing/coupon/save
#[debug_handler]
async fn coupon_save(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreCouponRequest>,
) -> Result<Response> {
    if StoreCouponService::create(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("保存失败"))
    }
}

/// 修改优惠券状态
/// POST /api/admin/marketing/coupon/update/status
#[debug_handler]
async fn coupon_update_status(
    State(ctx): State<AppContext>,
    Query(params): Query<CouponStatusRequest>,
) -> Result<Response> {
    if StoreCouponService::update_status(&ctx.db, params.id, params.status).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("修改状态失败"))
    }
}

/// 优惠券详情
/// POST /api/admin/marketing/coupon/info
#[debug_handler]
async fn coupon_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    let info = StoreCouponService::info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(info))
}

/// 发送优惠券列表
/// GET /api/admin/marketing/coupon/send/list
#[debug_handler]
async fn coupon_send_list(
    State(ctx): State<AppContext>,
    Query(search): Query<CouponSendListRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreCouponService::get_send_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 删除优惠券
/// POST /api/admin/marketing/coupon/delete
#[debug_handler]
async fn coupon_delete(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    if StoreCouponService::delete(&ctx.db, params.id).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("删除失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/marketing/coupon")
        .add("/list", get(coupon_list))
        .add("/save", post(coupon_save))
        .add("/update/status", post(coupon_update_status))
        .add("/info", post(coupon_info))
        .add("/send/list", get(coupon_send_list))
        .add("/delete", post(coupon_delete))
}
