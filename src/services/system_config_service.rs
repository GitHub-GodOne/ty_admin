/// 系统配置服务层
///
/// 处理系统配置相关的业务逻辑
/// Java代码参考: com.zbkj.service.service.impl.SystemConfigServiceImpl
use sea_orm::*;
use std::collections::HashMap;

use crate::common::constants;
use crate::common::error::{AppError, AppResult};
use crate::dtos::system_config::*;
use crate::models::_entities::system_config;

pub struct SystemConfigService;

impl SystemConfigService {
    /// 根据formId获取表单配置信息
    ///
    /// Java: SystemConfigServiceImpl.info(Integer formId)
    /// 查询所有formId对应的配置项，返回 HashMap<name, value>
    pub async fn info(
        db: &DatabaseConnection,
        form_id: i32,
    ) -> AppResult<HashMap<String, String>> {
        // 查询所有formId对应的配置项
        let configs = system_config::Entity::find()
            .filter(system_config::Column::FormId.eq(form_id))
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // 构建 HashMap<name, value>
        let mut result = HashMap::new();
        for config in configs {
            let value = config.value.unwrap_or_default();
            result.insert(config.name, value);
        }

        Ok(result)
    }

    /// 保存表单配置
    ///
    /// Java: SystemConfigServiceImpl.saveForm(SystemFormCheckRequest request)
    /// 1. 删除该formId下的所有旧配置
    /// 2. 批量插入新配置
    /// 3. 清除Redis缓存
    pub async fn save_form(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        request: &SystemFormCheckRequest,
    ) -> AppResult<()> {
        // 1. 删除该formId下的所有旧配置
        system_config::Entity::delete_many()
            .filter(system_config::Column::FormId.eq(request.id))
            .exec(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // 2. 批量插入新配置
        let now = chrono::Local::now().naive_local();
        for field in &request.fields {
            let new_config = system_config::ActiveModel {
                id: NotSet,
                name: Set(field.name.clone()),
                title: Set(field.title.clone()),
                form_id: Set(Some(request.id)),
                value: Set(field.value.clone()),
                status: Set(Some(1i16)),
                create_time: Set(Some(now)),
                update_time: Set(Some(now)),
            };
            system_config::Entity::insert(new_config)
                .exec(db)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        // 3. 清除Redis缓存
        Self::clear_config_cache(redis).await?;

        Ok(())
    }

    /// 根据配置名称获取配置
    ///
    /// Java: SystemConfigServiceImpl.getValueByName(String name)
    pub async fn get_by_name(
        db: &DatabaseConnection,
        name: &str,
    ) -> AppResult<Option<system_config::Model>> {
        let config = system_config::Entity::find()
            .filter(system_config::Column::Name.eq(name))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(config)
    }

    /// 根据key获取配置值（字符串）
    ///
    /// Java: SystemConfigServiceImpl.getValueByKey(String key)
    /// 返回配置值字符串，如果不存在返回空字符串
    pub async fn get_value_by_key(
        db: &DatabaseConnection,
        key: &str,
    ) -> AppResult<String> {
        let config = Self::get_by_name(db, key).await?;
        Ok(config.and_then(|c| c.value).unwrap_or_default())
    }

    /// 获取文件上传类型
    ///
    /// Java: SystemConfigServiceImpl.getFileUploadType()
    /// 配置名: "uploadType"
    pub async fn get_file_upload_type(
        db: &DatabaseConnection,
    ) -> AppResult<Option<system_config::Model>> {
        Self::get_by_name(db, "uploadType").await
    }

    /// 获取管理端站点Logo
    ///
    /// Java: SystemConfigServiceImpl.getAdminSiteLogo()
    /// 配置名: "siteLogoSquare" 和 "siteLogoLeftTop"
    pub async fn get_site_logo(
        db: &DatabaseConnection,
    ) -> AppResult<AdminSiteLogoResponse> {
        let square = Self::get_by_name(db, "siteLogoSquare").await?;
        let left_top = Self::get_by_name(db, "siteLogoLeftTop").await?;

        Ok(AdminSiteLogoResponse {
            site_logo_square: square.and_then(|c| c.value),
            site_logo_left_top: left_top.and_then(|c| c.value),
        })
    }

    /// 获取腾讯地图Key
    ///
    /// Java: SystemConfigServiceImpl.getTxMapKey()
    /// 配置名: "txMapKey"
    pub async fn get_tx_map_key(
        db: &DatabaseConnection,
    ) -> AppResult<Option<system_config::Model>> {
        Self::get_by_name(db, "txMapKey").await
    }

    /// 获取移动端首页列表样式
    ///
    /// Java: SystemConfigServiceImpl.getHomePageSaleListStyle()
    /// 配置名: "homePageSaleListStyle"
    pub async fn get_home_page_sale_list_style(
        db: &DatabaseConnection,
    ) -> AppResult<Option<system_config::Model>> {
        Self::get_by_name(db, "homePageSaleListStyle").await
    }

    /// 保存移动端首页列表样式
    ///
    /// Java: SystemConfigServiceImpl.saveHomePageSaleListStyle(SaveConfigRequest request)
    /// 配置名: "homePageSaleListStyle"
    pub async fn save_home_page_sale_list_style(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        request: &SaveConfigRequest,
    ) -> AppResult<()> {
        Self::update_or_save_value_by_name(
            db,
            redis,
            "homePageSaleListStyle",
            &request.value,
        )
        .await
    }

    /// 获取授权Host
    ///
    /// Java: SystemConfigServiceImpl.getAuthHost()
    /// 配置名: "authHost"
    pub async fn get_auth_host(
        db: &DatabaseConnection,
    ) -> AppResult<Option<system_config::Model>> {
        Self::get_by_name(db, "authHost").await
    }

    /// 获取主题颜色
    ///
    /// Java: SystemConfigServiceImpl.getChangeColor()
    /// 配置名: "changeColor"
    pub async fn get_change_color(
        db: &DatabaseConnection,
    ) -> AppResult<Option<system_config::Model>> {
        Self::get_by_name(db, "changeColor").await
    }

    /// 保存主题颜色
    ///
    /// Java: SystemConfigServiceImpl.saveChangeColor(SaveConfigRequest request)
    /// 配置名: "changeColor"
    pub async fn save_change_color(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        request: &SaveConfigRequest,
    ) -> AppResult<()> {
        Self::update_or_save_value_by_name(
            db,
            redis,
            "changeColor",
            &request.value,
        )
        .await
    }

    /// 清除配置缓存
    ///
    /// Java: SystemConfigServiceImpl.clearCache()
    pub async fn clear_cache(
        redis: &crate::utils::redis_client::RedisClient,
    ) -> AppResult<()> {
        Self::clear_config_cache(redis).await
    }

    /// 根据名称更新或新增配置值
    ///
    /// Java: SystemConfigServiceImpl.updateOrSaveValueByName(String name, String value)
    /// 核心方法：如果配置存在则更新，不存在则新增
    pub async fn update_or_save_value_by_name(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        name: &str,
        value: &str,
    ) -> AppResult<()> {
        let now = chrono::Local::now().naive_local();

        // 查找是否已存在
        let existing = system_config::Entity::find()
            .filter(system_config::Column::Name.eq(name))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match existing {
            Some(config) => {
                // 更新已有配置
                let mut active: system_config::ActiveModel = config.into();
                active.value = Set(Some(value.to_string()));
                active.update_time = Set(Some(now));
                active.update(db)
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            }
            None => {
                // 新增配置
                let new_config = system_config::ActiveModel {
                    id: NotSet,
                    name: Set(name.to_string()),
                    title: Set(name.to_string()),
                    form_id: Set(None),
                    value: Set(Some(value.to_string())),
                    status: Set(Some(1i16)),
                    create_time: Set(Some(now)),
                    update_time: Set(Some(now)),
                };
                system_config::Entity::insert(new_config)
                    .exec(db)
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            }
        }

        // 清除Redis缓存
        Self::clear_config_cache(redis).await?;

        Ok(())
    }

    /// 清除Redis中的配置缓存
    ///
    /// Java: asyncService.deleteCache(Constants.CONFIG_LIST)
    async fn clear_config_cache(
        redis: &crate::utils::redis_client::RedisClient,
    ) -> AppResult<()> {
        let _ = redis.del(constants::CONFIG_LIST_KEY).await;
        Ok(())
    }
}
