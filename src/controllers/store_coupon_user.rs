/// 优惠券领取记录 -- 控制器
///
/// Java参考: StoreCouponUserController
/// 路径前缀: /api/admin/marketing/coupon/user
use loco_rs::prelude::*;

use crate::common::response::ApiResponse;
use crate::dtos::common::PageParamRequest;
use crate::dtos::store_coupon_user::*;
use crate::services::store_coupon_user_service::StoreCouponUserService;

/// 分页列表
/// GET /api/admin/marketing/coupon/user/list
#[debug_handler]
async fn coupon_user_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreCouponUserSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    let response = StoreCouponUserService::get_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 管理员发放优惠券
/// POST /api/admin/marketing/coupon/user/receive
#[debug_handler]
async fn coupon_user_receive(
    State(ctx): State<AppContext>,
    Query(request): Query<StoreCouponUserRequest>,
) -> Result<Response> {
    if StoreCouponUserService::receive(&ctx.db, &request).await? {
        format::json(ApiResponse::<()>::success_empty())
    } else {
        format::json(ApiResponse::<()>::failed("发放失败"))
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/marketing/coupon/user")
        .add("/list", get(coupon_user_list))
        .add("/receive", post(coupon_user_receive))
}
