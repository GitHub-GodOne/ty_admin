/// 城市管理 -- 服务层
///
/// 实现与Java版本一致的城市管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.SystemCityServiceImpl
use loco_rs::prelude::*;

use crate::dtos::system_city::*;
use crate::initializers::redis::get_redis;
use crate::models::_entities::system_city;

/// Redis 缓存 key
const CITY_LIST: &str = "city_list";
const CITY_LIST_TREE: &str = "city_list_tree";

/// 缓存过期时间（秒）
const CITY_CACHE_EXPIRE: usize = 3600;

pub struct SystemCityService;

impl SystemCityService {
    /// 城市列表（按父级id查询）
    ///
    /// Java: SystemCityServiceImpl.getList(SystemCitySearchRequest)
    /// 1. 先从 Redis 缓存获取
    /// 2. 缓存不存在则从数据库查询（parentId + isShow=true）
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &SystemCitySearchRequest,
    ) -> Result<Vec<SystemCityResponse>> {
        // 尝试从 Redis 获取缓存
        if let Ok(redis) = get_redis().await {
            let cache_key = format!("{}:{}", CITY_LIST, request.parent_id);
            if let Ok(Some(cached)) = redis.get(&cache_key).await {
                if let Ok(list) = serde_json::from_str::<Vec<SystemCityResponse>>(&cached) {
                    return Ok(list);
                }
            }
        }

        // 缓存不存在，从数据库查询
        let list = Self::get_list_by_parent_id(db, request.parent_id).await?;

        // 异步写入缓存（忽略错误）
        if let Ok(redis) = get_redis().await {
            let cache_key = format!("{}:{}", CITY_LIST, request.parent_id);
            if let Ok(json) = serde_json::to_string(&list) {
                let _ = redis.set(&cache_key, &json, CITY_CACHE_EXPIRE).await;
            }
        }

        Ok(list)
    }

    /// 修改城市显示状态
    ///
    /// Java: SystemCityServiceImpl.updateStatus(Integer id, Boolean status)
    /// 更新后刷新 Redis 缓存
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let existing = system_city::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("城市不存在".to_string()))?;

        let parent_id = existing.parent_id;
        let new_is_show: i16 = if status { 1 } else { 0 };

        let mut model: system_city::ActiveModel = existing.into();
        model.is_show = Set(new_is_show);
        model.update_time = Set(chrono::Utc::now().naive_utc());
        model.update(db).await?;

        // 刷新 Redis 缓存
        Self::async_redis(parent_id).await;

        Ok(true)
    }

    /// 修改城市信息
    ///
    /// Java: SystemCityServiceImpl.update(Integer id, SystemCityRequest request)
    /// 更新 name 和 parentId，更新后刷新 Redis 缓存
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemCityRequest,
    ) -> Result<bool> {
        // 验证参数
        if request.name.trim().is_empty() {
            return Err(Error::BadRequest("城市名称不能为空".to_string()));
        }
        if request.name.len() > 100 {
            return Err(Error::BadRequest("城市名称不能超过100个字符".to_string()));
        }
        if request.parent_id < 0 {
            return Err(Error::BadRequest("父级id必须大于等于0".to_string()));
        }

        let existing = system_city::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("城市不存在".to_string()))?;

        let mut model: system_city::ActiveModel = existing.into();
        model.parent_id = Set(request.parent_id);
        model.name = Set(request.name.clone());
        model.update_time = Set(chrono::Utc::now().naive_utc());
        model.update(db).await?;

        // 刷新 Redis 缓存
        Self::async_redis(request.parent_id).await;

        Ok(true)
    }

    /// 获取城市详情
    ///
    /// Java: getById(id) - MyBatis Plus 内置方法
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemCityResponse> {
        let record = system_city::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("城市不存在".to_string()))?;

        Ok(Self::model_to_response(record))
    }

    /// 获取城市树形结构
    ///
    /// Java: SystemCityServiceImpl.getListTree()
    /// 1. 先从 Redis 缓存获取
    /// 2. 缓存不存在则从数据库构建树并缓存
    pub async fn get_list_tree(
        db: &DatabaseConnection,
    ) -> Result<Vec<SystemCityTreeVo>> {
        // 尝试从 Redis 获取缓存
        if let Ok(redis) = get_redis().await {
            if let Ok(Some(cached)) = redis.get(CITY_LIST_TREE).await {
                if let Ok(tree) = serde_json::from_str::<Vec<SystemCityTreeVo>>(&cached) {
                    if !tree.is_empty() {
                        return Ok(tree);
                    }
                }
            }
        }

        // 缓存不存在，从数据库构建树
        let tree = Self::build_city_tree(db).await?;

        // 写入 Redis 缓存
        if let Ok(redis) = get_redis().await {
            if let Ok(json) = serde_json::to_string(&tree) {
                let _ = redis.set(CITY_LIST_TREE, &json, CITY_CACHE_EXPIRE).await;
            }
        }

        Ok(tree)
    }

    // ==================== 内部方法 ====================

    /// 根据父级id查询城市列表（数据库）
    ///
    /// Java: getList(Integer parentId)
    /// 条件: parentId + isShow=true
    async fn get_list_by_parent_id(
        db: &DatabaseConnection,
        parent_id: i32,
    ) -> Result<Vec<SystemCityResponse>> {
        let records = system_city::Entity::find()
            .filter(system_city::Column::ParentId.eq(parent_id))
            .filter(system_city::Column::IsShow.eq(1i16))
            .all(db)
            .await?;

        Ok(records.into_iter().map(Self::model_to_response).collect())
    }

    /// 构建城市树形结构
    ///
    /// Java: SystemCityAsyncService.setListTree()
    /// 1. 查询所有 isShow=true 的城市
    /// 2. 按 parentId 分组
    /// 3. 递归构建树
    async fn build_city_tree(
        db: &DatabaseConnection,
    ) -> Result<Vec<SystemCityTreeVo>> {
        // 查询所有显示的城市
        let all_cities = system_city::Entity::find()
            .filter(system_city::Column::IsShow.eq(1i16))
            .all(db)
            .await?;

        // 按 parentId 分组
        let mut children_map: std::collections::HashMap<i32, Vec<system_city::Model>> =
            std::collections::HashMap::new();
        for city in &all_cities {
            children_map
                .entry(city.parent_id)
                .or_default()
                .push(city.clone());
        }

        // 从顶级节点（parentId=0）开始构建树
        let root_cities = children_map.get(&0).cloned().unwrap_or_default();
        let tree: Vec<SystemCityTreeVo> = root_cities
            .into_iter()
            .map(|city| Self::build_tree_node(city, &children_map))
            .collect();

        Ok(tree)
    }

    /// 递归构建树节点
    fn build_tree_node(
        model: system_city::Model,
        children_map: &std::collections::HashMap<i32, Vec<system_city::Model>>,
    ) -> SystemCityTreeVo {
        let children = children_map
            .get(&model.city_id)
            .cloned()
            .unwrap_or_default();

        let child: Vec<SystemCityTreeVo> = children
            .into_iter()
            .map(|c| Self::build_tree_node(c, children_map))
            .collect();

        SystemCityTreeVo {
            id: model.id,
            city_id: model.city_id,
            level: model.level,
            parent_id: model.parent_id,
            area_code: model.area_code,
            name: model.name,
            merger_name: model.merger_name,
            lng: model.lng,
            lat: model.lat,
            is_show: model.is_show != 0,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            child,
        }
    }

    /// 刷新 Redis 缓存
    ///
    /// Java: asyncRedis(Integer pid)
    /// 删除相关缓存，下次请求时重新加载
    async fn async_redis(parent_id: i32) {
        if let Ok(redis) = get_redis().await {
            // 删除对应 parentId 的列表缓存
            let cache_key = format!("{}:{}", CITY_LIST, parent_id);
            let _ = redis.del(&cache_key).await;
            // 删除树形缓存
            let _ = redis.del(CITY_LIST_TREE).await;
        }
    }

    /// Model 转 Response DTO
    fn model_to_response(model: system_city::Model) -> SystemCityResponse {
        SystemCityResponse {
            id: model.id,
            city_id: model.city_id,
            level: model.level,
            parent_id: model.parent_id,
            area_code: model.area_code,
            name: model.name,
            merger_name: model.merger_name,
            lng: model.lng,
            lat: model.lat,
            is_show: model.is_show != 0,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
