/// 微信菜单管理 -- 控制器
///
/// 实现与Java版本一致的微信菜单管理接口
/// Java代码参考: com.zbkj.admin.controller.WeChatController
/// 路由前缀: /api/admin/wechat/menu
///
/// 注意: 微信公众号菜单管理需要调用微信API
/// 当前实现为本地存储方案（存储到 system_config 表）
/// 如需对接微信API，需要配置微信公众号 appId 和 appSecret
use loco_rs::prelude::*;
use sea_orm::NotSet;

use crate::common::response::ApiResponse;
use crate::utils::auth;
use crate::models::_entities::system_config;

/// 1. 获取自定义菜单
///
/// 权限: admin:wechat:menu:public:get
/// 路径: GET /api/admin/wechat/menu/public/get
/// Java: WeChatController.get() -> wechatPublicService.getCustomizeMenus()
///
/// 从 system_config 中读取 wechat_menus 配置
#[debug_handler]
async fn get_menus(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:menu:public:get").await?;

    // 从 system_config 读取微信菜单配置
    let config = system_config::Entity::find()
        .filter(system_config::Column::Name.eq("wechat_menus"))
        .one(&ctx.db)
        .await?;

    match config {
        Some(c) => {
            let value = c.value.unwrap_or_default();
            // 尝试解析为 JSON 对象返回
            if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&value) {
                format::json(ApiResponse::success(json_value))
            } else {
                format::json(ApiResponse::success(value))
            }
        }
        None => {
            // 返回空对象
            format::json(ApiResponse::success(serde_json::json!({})))
        }
    }
}

/// 2. 保存自定义菜单
///
/// 权限: admin:wechat:menu:public:create
/// 路径: POST /api/admin/wechat/menu/public/create
/// Java: WeChatController.create(@RequestBody String data) -> wechatPublicService.createMenus(data)
///
/// 保存菜单数据到 system_config 表
#[debug_handler]
async fn create_menus(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
    body: String,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:menu:public:create").await?;

    // 验证 JSON 格式
    let _: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| Error::BadRequest("菜单数据格式错误，请提供有效的JSON".to_string()))?;

    // 查找或创建 wechat_menus 配置
    let existing = system_config::Entity::find()
        .filter(system_config::Column::Name.eq("wechat_menus"))
        .one(&ctx.db)
        .await?;

    let now = chrono::Utc::now().naive_utc();

    match existing {
        Some(config) => {
            let mut model: system_config::ActiveModel = config.into();
            model.value = Set(Some(body));
            model.update_time = Set(Some(now));
            model.update(&ctx.db).await?;
        }
        None => {
            let model = system_config::ActiveModel {
                id: NotSet,
                name: Set("wechat_menus".to_string()),
                title: Set("微信菜单配置".to_string()),
                value: Set(Some(body)),
                status: Set(Some(1i16)),
                form_id: Set(Some(0)),
                create_time: Set(Some(now)),
                update_time: Set(Some(now)),
            };
            system_config::Entity::insert(model).exec(&ctx.db).await?;
        }
    }

    format::json(ApiResponse::<()>::success_empty())
}

/// 3. 删除自定义菜单
///
/// 权限: admin:wechat:menu:public:delete
/// 路径: GET /api/admin/wechat/menu/public/delete
/// Java: WeChatController.delete() -> wechatPublicService.deleteMenus()
///
/// 清空 system_config 中的 wechat_menus 配置
#[debug_handler]
async fn delete_menus(
    State(ctx): State<AppContext>,
    headers: axum::http::HeaderMap,
) -> Result<Response> {
    auth::check_permission(&headers, "admin:wechat:menu:public:delete").await?;

    let existing = system_config::Entity::find()
        .filter(system_config::Column::Name.eq("wechat_menus"))
        .one(&ctx.db)
        .await?;

    if let Some(config) = existing {
        let mut model: system_config::ActiveModel = config.into();
        model.value = Set(Some("{}".to_string()));
        model.update_time = Set(Some(chrono::Utc::now().naive_utc()));
        model.update(&ctx.db).await?;
    }

    format::json(ApiResponse::<()>::success_empty())
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/wechat/menu")
        .add("/public/get", get(get_menus))
        .add("/public/create", post(create_menus))
        .add("/public/delete", get(delete_menus))
}
