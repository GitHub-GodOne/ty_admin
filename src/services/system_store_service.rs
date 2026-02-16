/// 门店自提 -- 服务层
///
/// Java参考: SystemStoreServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::dtos::system_store::*;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::models::_entities::system_store;

pub struct SystemStoreService;

impl SystemStoreService {
    /// 分页列表
    ///
    /// Java: SystemStoreServiceImpl.getList()
    /// 逻辑：
    /// - status=1: is_show=1 AND is_del=0 (显示中)
    /// - status=2: is_del=1 (回收站)
    /// - 其他: is_show=0 AND is_del=0 (隐藏中)
    /// - keywords: name LIKE OR phone LIKE
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &SystemStoreSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SystemStoreResponse>> {
        let mut query = system_store::Entity::find();

        match request.status {
            1 => {
                query = query
                    .filter(system_store::Column::IsShow.eq(1))
                    .filter(system_store::Column::IsDel.eq(0));
            }
            2 => {
                query = query.filter(system_store::Column::IsDel.eq(1));
            }
            _ => {
                query = query
                    .filter(system_store::Column::IsShow.eq(0))
                    .filter(system_store::Column::IsDel.eq(0));
            }
        }

        if let Some(keywords) = &request.keywords {
            if !keywords.trim().is_empty() {
                let kw = keywords.trim().to_string();
                query = query.filter(
                    Condition::any()
                        .add(system_store::Column::Name.contains(&kw))
                        .add(system_store::Column::Phone.contains(&kw)),
                );
            }
        }

        query = query
            .order_by_desc(system_store::Column::UpdateTime)
            .order_by_desc(system_store::Column::Id);

        let page = page_param.get_page() as u64;
        let limit = page_param.get_limit() as u64;
        let paginator = query.paginate(db, limit);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page(page.saturating_sub(1)).await?;

        let list = records.into_iter().map(Self::model_to_response).collect();

        Ok(CommonPage::new(list, total as i64, page_param.get_page(), page_param.get_limit()))
    }

    /// 数量统计
    ///
    /// Java: SystemStoreServiceImpl.getCount()
    pub async fn get_count(
        db: &DatabaseConnection,
        keywords: &Option<String>,
    ) -> Result<SystemStoreCountResponse> {
        let show = Self::count_by_status(db, 1, keywords).await?;
        let hide = Self::count_by_status(db, 0, keywords).await?;
        let recycle = Self::count_by_status(db, 2, keywords).await?;

        Ok(SystemStoreCountResponse { show, hide, recycle })
    }

    /// 根据状态统计数量
    async fn count_by_status(
        db: &DatabaseConnection,
        status: i32,
        keywords: &Option<String>,
    ) -> Result<i64> {
        let mut query = system_store::Entity::find();

        if status == 2 {
            query = query.filter(system_store::Column::IsDel.eq(1));
        } else {
            query = query
                .filter(system_store::Column::IsShow.eq(status))
                .filter(system_store::Column::IsDel.eq(0));
        }

        if let Some(kw) = keywords {
            if !kw.trim().is_empty() {
                let kw = kw.trim().to_string();
                query = query.filter(
                    Condition::any()
                        .add(system_store::Column::Name.contains(&kw))
                        .add(system_store::Column::Phone.contains(&kw)),
                );
            }
        }

        let count = query.count(db).await?;
        Ok(count as i64)
    }

    /// 新增门店
    ///
    /// Java: SystemStoreServiceImpl.create()
    /// 逻辑：拷贝属性，分解经纬度，去掉图片前缀
    pub async fn create(
        db: &DatabaseConnection,
        request: &SystemStoreRequest,
    ) -> Result<bool> {
        let now = Local::now().naive_local();
        let (lat, lng) = Self::split_lat(&request.latitude, &request.longitude);

        let model = system_store::ActiveModel {
            name: Set(request.name.clone()),
            introduction: Set(request.introduction.clone().unwrap_or_default()),
            phone: Set(request.phone.clone().unwrap_or_default()),
            address: Set(request.address.clone()),
            detailed_address: Set(request.detailed_address.clone()),
            image: Set(Self::clear_prefix(&request.image)),
            latitude: Set(lat),
            longitude: Set(lng),
            valid_time: Set(request.valid_time.clone().unwrap_or_default()),
            day_time: Set(request.day_time.clone().unwrap_or_default()),
            is_show: Set(0),
            is_del: Set(0),
            create_time: Set(now),
            update_time: Set(now),
            ..Default::default()
        };

        model.insert(db).await?;
        Ok(true)
    }

    /// 修改门店
    ///
    /// Java: SystemStoreServiceImpl.update()
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemStoreRequest,
    ) -> Result<bool> {
        let store = Self::get_by_id(db, id).await?;
        let now = Local::now().naive_local();
        let (lat, lng) = Self::split_lat(&request.latitude, &request.longitude);

        let mut active: system_store::ActiveModel = store.into();
        active.name = Set(request.name.clone());
        active.introduction = Set(request.introduction.clone().unwrap_or_default());
        active.phone = Set(request.phone.clone().unwrap_or_default());
        active.address = Set(request.address.clone());
        active.detailed_address = Set(request.detailed_address.clone());
        active.image = Set(Self::clear_prefix(&request.image));
        active.latitude = Set(lat);
        active.longitude = Set(lng);
        active.valid_time = Set(request.valid_time.clone().unwrap_or_default());
        active.day_time = Set(request.day_time.clone().unwrap_or_default());
        active.update_time = Set(now);
        active.update(db).await?;

        Ok(true)
    }

    /// 软删除门店
    ///
    /// Java: SystemStoreServiceImpl.delete()
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let store = Self::get_by_id(db, id).await?;
        let now = Local::now().naive_local();
        let mut active: system_store::ActiveModel = store.into();
        active.is_del = Set(1);
        active.update_time = Set(now);
        active.update(db).await?;
        Ok(true)
    }

    /// 修改门店显示状态
    ///
    /// Java: SystemStoreServiceImpl.updateStatus()
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let store = Self::get_by_id(db, id).await?;
        let is_show_val: i16 = if status { 1 } else { 0 };
        if store.is_show == is_show_val {
            return Ok(true);
        }
        let now = Local::now().naive_local();
        let mut active: system_store::ActiveModel = store.into();
        active.is_show = Set(is_show_val);
        active.update_time = Set(now);
        active.update(db).await?;
        Ok(true)
    }

    /// 门店详情
    ///
    /// Java: SystemStoreServiceImpl.getInfo()
    /// 逻辑：返回时 latitude = "latitude,longitude"
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemStoreResponse> {
        let store = Self::get_by_id(db, id).await?;
        let mut resp = Self::model_to_response(store.clone());
        // Java: systemStore.setLatitude(systemStore.getLatitude() + "," + systemStore.getLongitude())
        resp.latitude = format!("{},{}", store.latitude, store.longitude);
        Ok(resp)
    }

    /// 彻底删除
    ///
    /// Java: SystemStoreServiceImpl.completeLyDelete()
    pub async fn completely_delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let _store = Self::get_by_id(db, id).await?;
        system_store::Entity::delete_by_id(id).exec(db).await?;
        Ok(true)
    }

    /// 恢复
    ///
    /// Java: SystemStoreServiceImpl.recovery()
    pub async fn recovery(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let store = Self::get_by_id(db, id).await?;
        if store.is_del == 0 {
            return Ok(true);
        }
        let now = Local::now().naive_local();
        let mut active: system_store::ActiveModel = store.into();
        active.is_del = Set(0);
        active.update_time = Set(now);
        active.update(db).await?;
        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 根据id查找门店
    async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<system_store::Model> {
        system_store::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("门店自提点不存在".to_string()))
    }

    /// 根据id集合查询门店，返回 HashMap
    ///
    /// Java: SystemStoreServiceImpl.getMapInId()
    pub async fn get_map_in_id(
        db: &DatabaseConnection,
        store_ids: &[i32],
    ) -> Result<std::collections::HashMap<i32, system_store::Model>> {
        use std::collections::HashMap;
        if store_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let stores = system_store::Entity::find()
            .filter(system_store::Column::Id.is_in(store_ids.to_vec()))
            .all(db)
            .await?;
        let mut map = HashMap::new();
        for s in stores {
            map.insert(s.id, s);
        }
        Ok(map)
    }

    /// 分解经纬度
    ///
    /// Java: SystemStoreServiceImpl.splitLat()
    /// 前端传 latitude="经度,纬度" 格式，需要拆分
    fn split_lat(latitude: &Option<String>, longitude: &Option<String>) -> (String, String) {
        if let Some(lat_str) = latitude {
            if lat_str.contains(',') {
                let parts: Vec<&str> = lat_str.split(',').collect();
                if parts.len() >= 2 {
                    // Java: longitude = list.get(0), latitude = list.get(1)
                    return (parts[1].trim().to_string(), parts[0].trim().to_string());
                }
            }
            return (lat_str.clone(), longitude.clone().unwrap_or_default());
        }
        (String::new(), longitude.clone().unwrap_or_default())
    }

    /// 去掉图片前缀
    fn clear_prefix(image: &str) -> String {
        image.to_string()
    }

    /// Model 转 Response
    fn model_to_response(model: system_store::Model) -> SystemStoreResponse {
        SystemStoreResponse {
            id: model.id,
            name: model.name,
            introduction: model.introduction,
            phone: model.phone,
            address: model.address,
            detailed_address: model.detailed_address,
            image: model.image,
            latitude: model.latitude,
            longitude: model.longitude,
            valid_time: model.valid_time,
            day_time: model.day_time,
            is_show: model.is_show != 0,
            is_del: model.is_del != 0,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
