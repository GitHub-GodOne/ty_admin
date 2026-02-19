/// 商品管理 -- 控制器
///
/// 实现与Java版本一致的商品管理接口
/// Java代码参考: com.zbkj.admin.controller.StoreProductController
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::response::ApiResponse;
use crate::dtos::common::{CommonPage, IdQuery, IdsQuery, PageParamRequest};
use crate::dtos::product::*;
use crate::services::store_product_service::StoreProductService;

// ==================== 接口实现 ====================

/// 分页列表
///
/// 权限: admin:product:list
/// 路径: GET /api/admin/store/product/list
#[debug_handler]
async fn get_list(
    State(ctx): State<AppContext>,
    Query(search): Query<StoreProductSearchRequest>,
    Query(page): Query<PageParamRequest>,
) -> Result<Response> {
    tracing::info!("商品列表查询: type={}, page={:?}", search.product_type, page.page);

    let response = StoreProductService::get_admin_list(&ctx.db, &search, &page).await?;
    format::json(ApiResponse::success(response))
}

/// 根据ID集合获取商品列表
///
/// 权限: admin:product:listbyids
/// 路径: GET /api/admin/store/product/listids
#[debug_handler]
async fn get_list_by_ids(
    State(ctx): State<AppContext>,
    Query(params): Query<IdsQuery>,
) -> Result<Response> {
    tracing::info!("根据ID获取商品列表: ids={}", params.ids);

    // 解析ID字符串为Vec<i32>
    let ids: Vec<i32> = params.ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let list = StoreProductService::get_list_in_ids(&ctx.db, ids).await?;

    let response: CommonPage<StoreProductResponse> = CommonPage {
        list,
        total: 0,
        page_number: 1,
        page_size: 100,
    };

    format::json(ApiResponse::success(response))
}

/// 商品详情
///
/// 权限: admin:product:info
/// 路径: GET /api/admin/store/product/info
#[debug_handler]
async fn get_info(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    tracing::info!("获取商品详情: id={}", params.id);

    let info = StoreProductService::get_info(&ctx.db, params.id).await?;
    format::json(ApiResponse::success(info))
}

/// 商品表头数量统计
///
/// 权限: admin:product:tabs:headers
/// 路径: GET /api/admin/store/product/tabs/headers
#[debug_handler]
async fn get_tabs_header(
    State(ctx): State<AppContext>,
) -> Result<Response> {
    tracing::info!("获取商品表头统计");

    let headers = StoreProductService::get_tabs_header(&ctx.db).await?;
    format::json(ApiResponse::success(headers))
}

/// 上架商品
///
/// 权限: admin:product:up
/// 路径: GET /api/admin/store/product/putOnShell
#[debug_handler]
async fn put_on_shell(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    tracing::info!("上架商品: id={}", params.id);

    StoreProductService::put_on_shelf(&ctx.db, params.id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 下架商品
///
/// 权限: admin:product:down
/// 路径: GET /api/admin/store/product/offShell
#[debug_handler]
async fn off_shell(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    tracing::info!("下架商品: id={}", params.id);

    StoreProductService::off_shelf(&ctx.db, params.id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 删除商品
///
/// 权限: admin:product:delete
/// 路径: GET /api/admin/store/product/delete
#[debug_handler]
async fn delete_product(
    State(ctx): State<AppContext>,
    Query(params): Query<DeleteProductQuery>,
) -> Result<Response> {
    let delete_type = params.delete_type.unwrap_or_else(|| "recycle".to_string());
    tracing::info!("删除商品: id={}, type={}", params.id, delete_type);

    StoreProductService::delete_product(&ctx.db, params.id, &delete_type).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 恢复商品
///
/// 权限: admin:product:restore
/// 路径: GET /api/admin/store/product/restore
#[debug_handler]
async fn restore_product(
    State(ctx): State<AppContext>,
    Query(params): Query<IdQuery>,
) -> Result<Response> {
    tracing::info!("恢复商品: id={}", params.id);

    StoreProductService::restore_product(&ctx.db, params.id).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 新增商品
///
/// 权限: admin:product:save
/// 路径: POST /api/admin/store/product/save
#[debug_handler]
async fn save_product(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreProductAddRequest>,
) -> Result<Response> {
    tracing::info!("新增商品: name={:?}", request.store_name);

    StoreProductService::save(&ctx.db, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 修改商品
///
/// 权限: admin:product:update
/// 路径: POST /api/admin/store/product/update
#[debug_handler]
async fn update_product(
    State(ctx): State<AppContext>,
    Json(request): Json<StoreProductAddRequest>,
) -> Result<Response> {
    tracing::info!("修改商品: id={:?}, name={:?}", request.id, request.store_name);

    StoreProductService::update(&ctx.db, &request).await?;
    format::json(ApiResponse::<()>::success_empty())
}

/// 导入99Api商品
///
/// 权限: admin:product:import:product
/// 路径: POST /api/admin/store/product/importProduct
#[debug_handler]
async fn import_product(
    State(_ctx): State<AppContext>,
    Query(params): Query<ImportProductQuery>,
) -> Result<Response> {
    tracing::info!("导入商品: form={}, url={}", params.form, params.url);

    // TODO: 实现实际的导入逻辑
    format::json(ApiResponse::<()>::success_empty())
}

/// 获取复制商品配置
///
/// 权限: admin:product:copy:config
/// 路径: POST /api/admin/store/product/copy/config
#[debug_handler]
async fn copy_config(
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    tracing::info!("获取复制商品配置");

    // TODO: 实现实际的配置获取逻辑
    let config = serde_json::json!({});
    format::json(ApiResponse::success(config))
}

/// 复制平台商品
///
/// 权限: admin:product:copy:product
/// 路径: POST /api/admin/store/product/copy/product
#[debug_handler]
async fn copy_product(
    State(_ctx): State<AppContext>,
    Json(request): Json<CopyProductRequest>,
) -> Result<Response> {
    tracing::info!("复制商品: url={}", request.url);

    // TODO: 实现实际的复制逻辑
    let result = serde_json::json!({});
    format::json(ApiResponse::success(result))
}

/// 快捷添加库存
///
/// 权限: admin:product:quick:stock:add
/// 路径: POST /api/admin/store/product/quick/stock/add
#[debug_handler]
async fn quick_add_stock(
    State(ctx): State<AppContext>,
    Json(request): Json<ProductAddStockRequest>,
) -> Result<Response> {
    tracing::info!("快捷添加库存: id={}, stock={}", request.id, request.stock);

    let attr_stocks = request.attr_stock.map(|items| {
        items.into_iter().map(|item| (item.attr_value_id, item.stock)).collect()
    });

    StoreProductService::quick_add_stock(&ctx.db, request.id, request.stock, attr_stocks).await?;
    format::json(ApiResponse::<()>::success_empty())
}

// ==================== 请求结构体 ====================

#[derive(Debug, Deserialize)]
struct DeleteProductQuery {
    id: i32,
    #[serde(rename = "type")]
    delete_type: Option<String>,
}

/// 导入商品请求
#[derive(Debug, Deserialize)]
struct ImportProductQuery {
    /// 导入平台: 1=淘宝，2=京东，3=苏宁，4=拼多多, 5=天猫
    form: i32,
    /// 商品URL
    url: String,
}

/// 复制商品请求
#[derive(Debug, Deserialize, Serialize)]
struct CopyProductRequest {
    /// 商品URL
    url: String,
}

/// 快捷添加库存请求
#[derive(Debug, Deserialize, Serialize)]
struct ProductAddStockRequest {
    /// 商品ID
    id: i32,
    /// 添加的库存数量（无SKU时使用）
    #[serde(default)]
    stock: i32,
    /// 按SKU添加库存
    #[serde(rename = "attrStock")]
    attr_stock: Option<Vec<AttrStockItem>>,
}

/// SKU库存项
#[derive(Debug, Deserialize, Serialize)]
struct AttrStockItem {
    /// attrValue记录ID
    #[serde(rename = "attrValueId")]
    attr_value_id: i32,
    /// 增加的库存数量
    stock: i32,
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/store/product")
        // GET 接口
        .add("/list", get(get_list))
        .add("/tabs/headers", get(get_tabs_header))
        .add("/listids", get(get_list_by_ids))
        .add("/info", get(get_info))
        .add("/putOnShell", get(put_on_shell))
        .add("/offShell", get(off_shell))
        .add("/delete", get(delete_product))
        .add("/restore", get(restore_product))
        // POST 接口
        .add("/save", post(save_product))
        .add("/update", post(update_product))
        .add("/importProduct", post(import_product))
        .add("/copy/config", post(copy_config))
        .add("/copy/product", post(copy_product))
        .add("/quick/stock/add", post(quick_add_stock))
}
