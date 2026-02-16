/// 页面布局服务
///
/// Java参考: PageLayoutServiceImpl
use sea_orm::*;
use std::collections::HashMap;

use crate::common::constants;
use crate::dtos::page_layout::PageLayoutBottomNavigationResponse;
use crate::models::_entities::system_group_data;
use crate::services::system_attachment_service::SystemAttachmentService;
use crate::services::system_config_service::SystemConfigService;

pub struct PageLayoutService;

impl PageLayoutService {
    /// 页面首页 — 获取所有布局数据
    ///
    /// Java: PageLayoutServiceImpl.index()
    pub async fn index(
        db: &DatabaseConnection,
    ) -> Result<HashMap<String, serde_json::Value>, DbErr> {
        let mut map = HashMap::new();

        // 首页banner
        let banner_list = Self::find_list_by_gid(db, constants::GROUP_DATA_ID_INDEX_BANNER).await?;
        map.insert("indexBanner".to_string(), serde_json::to_value(Self::convert_data(&banner_list)).unwrap_or_default());

        // 首页金刚区
        let menu_list = Self::find_list_by_gid(db, constants::GROUP_DATA_ID_INDEX_MENU).await?;
        map.insert("indexMenu".to_string(), serde_json::to_value(Self::convert_data(&menu_list)).unwrap_or_default());

        // 首页新闻
        let news_list = Self::find_list_by_gid(db, constants::GROUP_DATA_ID_INDEX_NEWS_BANNER).await?;
        map.insert("indexNews".to_string(), serde_json::to_value(Self::convert_data(&news_list)).unwrap_or_default());

        // 我的页服务
        let user_menu_list = Self::find_list_by_gid(db, constants::GROUP_DATA_ID_USER_CENTER_MENU).await?;
        map.insert("userMenu".to_string(), serde_json::to_value(Self::convert_data(&user_menu_list)).unwrap_or_default());

        // 我的页banner
        let user_banner_list = Self::find_list_by_gid(db, constants::GROUP_DATA_ID_USER_CENTER_BANNER).await?;
        map.insert("userBanner".to_string(), serde_json::to_value(Self::convert_data(&user_banner_list)).unwrap_or_default());

        Ok(map)
    }
    /// 首页保存（全量）
    ///
    /// Java: PageLayoutServiceImpl.save()
    /// 删除所有5个分组的旧数据，然后批量插入新数据
    pub async fn save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let mut data_list: Vec<system_group_data::ActiveModel> = Vec::new();

        if let Some(arr) = json.get("indexBanner").and_then(|v| v.as_array()) {
            data_list.extend(Self::convert_group_data(db, arr, constants::GROUP_DATA_ID_INDEX_BANNER).await);
        }
        if let Some(arr) = json.get("indexMenu").and_then(|v| v.as_array()) {
            data_list.extend(Self::convert_group_data(db, arr, constants::GROUP_DATA_ID_INDEX_MENU).await);
        }
        if let Some(arr) = json.get("indexNews").and_then(|v| v.as_array()) {
            data_list.extend(Self::convert_group_data(db, arr, constants::GROUP_DATA_ID_INDEX_NEWS_BANNER).await);
        }
        if let Some(arr) = json.get("userMenu").and_then(|v| v.as_array()) {
            data_list.extend(Self::convert_group_data(db, arr, constants::GROUP_DATA_ID_USER_CENTER_MENU).await);
        }
        if let Some(arr) = json.get("userBanner").and_then(|v| v.as_array()) {
            data_list.extend(Self::convert_group_data(db, arr, constants::GROUP_DATA_ID_USER_CENTER_BANNER).await);
        }

        // 删除旧数据
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_BANNER).await?;
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_MENU).await?;
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_NEWS_BANNER).await?;
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_USER_CENTER_MENU).await?;
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_USER_CENTER_BANNER).await?;

        // 批量插入
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }
    /// 页面首页banner保存
    ///
    /// Java: PageLayoutServiceImpl.indexBannerSave()
    pub async fn index_banner_save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let arr = json.get("indexBanner").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_INDEX_BANNER).await,
            None => vec![],
        };
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_BANNER).await?;
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }

    /// 页面首页menu保存
    ///
    /// Java: PageLayoutServiceImpl.indexMenuSave()
    pub async fn index_menu_save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let arr = json.get("indexMenu").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_INDEX_MENU).await,
            None => vec![],
        };
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_MENU).await?;
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }

    /// 页面首页新闻保存
    ///
    /// Java: PageLayoutServiceImpl.indexNewsSave()
    pub async fn index_news_save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let arr = json.get("indexNews").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_INDEX_NEWS_BANNER).await,
            None => vec![],
        };
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_NEWS_BANNER).await?;
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }
    /// 页面用户中心banner保存
    ///
    /// Java: PageLayoutServiceImpl.userBannerSave()
    pub async fn user_banner_save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let arr = json.get("userBanner").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_USER_CENTER_BANNER).await,
            None => vec![],
        };
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_USER_CENTER_BANNER).await?;
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }

    /// 页面用户中心导航保存
    ///
    /// Java: PageLayoutServiceImpl.userMenuSave()
    pub async fn user_menu_save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let arr = json.get("userMenu").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_USER_CENTER_MENU).await,
            None => vec![],
        };
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_USER_CENTER_MENU).await?;
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }

    /// 页面用户中心商品table保存
    ///
    /// Java: PageLayoutServiceImpl.indexTableSave()
    pub async fn index_table_save(
        db: &DatabaseConnection,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let arr = json.get("indexTable").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_INDEX_EX_BANNER).await,
            None => vec![],
        };
        Self::delete_by_gid(db, constants::GROUP_DATA_ID_INDEX_EX_BANNER).await?;
        Self::save_batch(db, data_list).await?;
        Ok(true)
    }
    /// 获取页面底部导航信息
    ///
    /// Java: PageLayoutServiceImpl.getBottomNavigation()
    pub async fn get_bottom_navigation(
        db: &DatabaseConnection,
    ) -> Result<PageLayoutBottomNavigationResponse, DbErr> {
        let data_list = Self::find_list_by_gid(db, constants::GROUP_DATA_ID_BOTTOM_NAVIGATION).await?;
        let bottom_navigation_list = Self::convert_data(&data_list);

        // 是否自定义
        let is_custom = SystemConfigService::get_value_by_key(db, constants::CONFIG_BOTTOM_NAVIGATION_IS_CUSTOM)
            .await
            .unwrap_or_default();

        Ok(PageLayoutBottomNavigationResponse {
            bottom_navigation_list,
            is_custom,
        })
    }

    /// 页面底部导航信息保存
    ///
    /// Java: PageLayoutServiceImpl.bottomNavigationSave()
    pub async fn bottom_navigation_save(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        json: &serde_json::Value,
    ) -> Result<bool, DbErr> {
        let is_custom = json.get("isCustom").and_then(|v| v.as_str()).unwrap_or("");
        if is_custom.is_empty() {
            return Err(DbErr::Custom("请选择是否自定义".to_string()));
        }

        let arr = json.get("bottomNavigationList").and_then(|v| v.as_array());
        let data_list = match arr {
            Some(a) if !a.is_empty() => Self::convert_group_data(db, a, constants::GROUP_DATA_ID_BOTTOM_NAVIGATION).await,
            _ => return Err(DbErr::Custom("请传入底部导航数据".to_string())),
        };

        Self::delete_by_gid(db, constants::GROUP_DATA_ID_BOTTOM_NAVIGATION).await?;
        Self::save_batch(db, data_list).await?;

        // 保存是否自定义配置
        SystemConfigService::update_or_save_value_by_name(
            db, redis,
            constants::CONFIG_BOTTOM_NAVIGATION_IS_CUSTOM,
            is_custom,
        ).await.map_err(|e| DbErr::Custom(e.to_string()))?;

        Ok(true)
    }
    // ==================== 私有辅助方法 ====================

    /// 根据gid查询组合数据列表
    async fn find_list_by_gid(
        db: &DatabaseConnection,
        gid: i32,
    ) -> Result<Vec<system_group_data::Model>, DbErr> {
        system_group_data::Entity::find()
            .filter(system_group_data::Column::Gid.eq(gid))
            .order_by_asc(system_group_data::Column::Sort)
            .all(db)
            .await
    }

    /// 根据gid删除组合数据
    async fn delete_by_gid(
        db: &DatabaseConnection,
        gid: i32,
    ) -> Result<(), DbErr> {
        system_group_data::Entity::delete_many()
            .filter(system_group_data::Column::Gid.eq(gid))
            .exec(db)
            .await?;
        Ok(())
    }

    /// 批量保存组合数据
    async fn save_batch(
        db: &DatabaseConnection,
        data_list: Vec<system_group_data::ActiveModel>,
    ) -> Result<(), DbErr> {
        for item in data_list {
            item.insert(db).await?;
        }
        Ok(())
    }
    /// 转换JSON数组为组合数据实体列表
    ///
    /// Java: PageLayoutServiceImpl.convertGroupData()
    /// 将前端传入的JSON对象列表转换为SystemGroupData实体
    async fn convert_group_data(
        db: &DatabaseConnection,
        json_list: &[serde_json::Value],
        gid: i32,
    ) -> Vec<system_group_data::ActiveModel> {
        let now = chrono::Local::now().naive_local();
        let mut result = Vec::new();

        for e in json_list {
            let obj = match e.as_object() {
                Some(o) => o,
                None => continue,
            };

            let sort = obj.get("sort").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let status_val = obj.get("status").map(|v| {
                match v {
                    serde_json::Value::Bool(b) => if *b { 1i16 } else { 0i16 },
                    serde_json::Value::Number(n) => n.as_i64().unwrap_or(0) as i16,
                    _ => 0i16,
                }
            }).unwrap_or(0i16);
            let tempid = obj.get("tempid").and_then(|v| v.as_i64()).map(|v| v as i32);

            // 组装value JSON
            let mut json_map = serde_json::Map::new();
            if let Some(tid) = tempid {
                json_map.insert("id".to_string(), serde_json::Value::Number(tid.into()));
            }
            json_map.insert("sort".to_string(), serde_json::Value::Number(sort.into()));
            json_map.insert("status".to_string(), serde_json::Value::Number(status_val.into()));

            // 构建fields数组：排除id/gid/sort/status/tempid，其余字段作为fields
            let excluded = ["id", "gid", "sort", "status", "tempid"];
            let mut fields: Vec<serde_json::Value> = Vec::new();
            for (key, value) in obj {
                if excluded.contains(&key.as_str()) {
                    continue;
                }
                let mut field_map = serde_json::Map::new();
                field_map.insert("name".to_string(), serde_json::Value::String(key.clone()));
                field_map.insert("title".to_string(), serde_json::Value::String(key.clone()));

                // 如果值包含上传文件关键字，清除前缀
                let val_str = value.as_str().unwrap_or("");
                if val_str.contains(constants::UPLOAD_FILE_KEYWORD) {
                    let cleared = SystemAttachmentService::clear_prefix(db, val_str)
                        .await
                        .unwrap_or_else(|_| val_str.to_string());
                    field_map.insert("value".to_string(), serde_json::Value::String(cleared));
                } else {
                    field_map.insert("value".to_string(), value.clone());
                }
                fields.push(serde_json::Value::Object(field_map));
            }
            json_map.insert("fields".to_string(), serde_json::Value::Array(fields));

            let value_str = serde_json::to_string(&serde_json::Value::Object(json_map))
                .unwrap_or_default();

            // 构建ActiveModel
            let mut active = system_group_data::ActiveModel {
                gid: Set(gid),
                value: Set(value_str),
                sort: Set(sort),
                status: Set(status_val),
                create_time: Set(Some(now)),
                update_time: Set(Some(now)),
                ..Default::default()
            };

            // 如果有id则设置（更新场景）
            if let Some(id) = obj.get("id").and_then(|v| v.as_i64()) {
                if id > 0 {
                    active.id = Set(id as i32);
                }
            }

            result.push(active);
        }
        result
    }
    /// 转换组合数据为前端响应格式
    ///
    /// Java: PageLayoutServiceImpl.convertData()
    /// 将SystemGroupData列表转换为HashMap列表
    /// value JSON中的fields数组展开为顶层字段
    fn convert_data(data_list: &[system_group_data::Model]) -> Vec<HashMap<String, serde_json::Value>> {
        data_list.iter().map(|data| {
            let mut map = HashMap::new();
            map.insert("id".to_string(), serde_json::Value::Number(data.id.into()));
            map.insert("gid".to_string(), serde_json::Value::Number(data.gid.into()));
            map.insert("sort".to_string(), serde_json::Value::Number(data.sort.into()));
            map.insert("status".to_string(), serde_json::Value::Number(data.status.into()));

            // 解析value JSON
            if let Ok(json_obj) = serde_json::from_str::<serde_json::Value>(&data.value) {
                // 展开fields数组为顶层字段
                if let Some(fields) = json_obj.get("fields").and_then(|v| v.as_array()) {
                    for field in fields {
                        if let (Some(name), Some(value)) = (
                            field.get("name").and_then(|v| v.as_str()),
                            field.get("value"),
                        ) {
                            map.insert(name.to_string(), value.clone());
                        }
                    }
                }
                // tempid = json中的id
                if let Some(tid) = json_obj.get("id") {
                    map.insert("tempid".to_string(), tid.clone());
                }
            }

            map
        }).collect()
    }
}
