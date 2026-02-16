/// 门店核销员 -- 服务层
///
/// Java参考: SystemStoreStaffServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;
use std::collections::HashMap;

use crate::dtos::system_store::*;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::models::_entities::{system_store_staff, user};
use crate::services::system_store_service::SystemStoreService;

pub struct SystemStoreStaffService;

impl SystemStoreStaffService {
    /// 分页列表
    ///
    /// Java: SystemStoreStaffServiceImpl.getList()
    /// 逻辑：
    /// 1. 按 storeId 过滤（>0时）
    /// 2. 查询关联的用户信息和门店信息
    pub async fn get_list(
        db: &DatabaseConnection,
        store_id: i32,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SystemStoreStaffResponse>> {
        let mut query = system_store_staff::Entity::find();

        if store_id > 0 {
            query = query.filter(system_store_staff::Column::StoreId.eq(store_id));
        }

        let page = page_param.get_page() as u64;
        let limit = page_param.get_limit() as u64;
        let paginator = query.paginate(db, limit);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page(page.saturating_sub(1)).await?;

        if records.is_empty() {
            return Ok(CommonPage::new(vec![], total as i64, page_param.get_page(), page_param.get_limit()));
        }

        // 收集用户id和门店id
        let user_ids: Vec<i32> = records.iter().map(|r| r.uid as i32).collect();
        let store_ids: Vec<i32> = records.iter().map(|r| r.store_id).collect();

        // 批量查询用户信息
        let user_map = Self::get_user_map(db, &user_ids).await?;

        // 批量查询门店信息
        let store_map = SystemStoreService::get_map_in_id(db, &store_ids).await?;

        let list = records
            .into_iter()
            .map(|staff| {
                let user_info = user_map.get(&(staff.uid as i32)).map(|u| StaffUserInfo {
                    uid: u.uid,
                    nickname: u.nickname.clone(),
                    avatar: u.avatar.clone(),
                    phone: u.phone.clone(),
                });
                let store_info = store_map.get(&staff.store_id).map(|s| StaffStoreInfo {
                    id: s.id,
                    name: s.name.clone(),
                    phone: s.phone.clone(),
                    address: s.address.clone(),
                });
                Self::model_to_response(staff, user_info, store_info)
            })
            .collect();

        Ok(CommonPage::new(list, total as i64, page_param.get_page(), page_param.get_limit()))
    }

    /// 添加核销员（唯一验证）
    ///
    /// Java: SystemStoreStaffServiceImpl.saveUnique()
    /// 逻辑：检查uid是否已存在，不存在则新增
    pub async fn save_unique(
        db: &DatabaseConnection,
        request: &SystemStoreStaffRequest,
    ) -> Result<bool> {
        // 检查uid是否已存在
        let exists = system_store_staff::Entity::find()
            .filter(system_store_staff::Column::Uid.eq(request.uid as u32))
            .one(db)
            .await?;

        if exists.is_some() {
            return Err(Error::BadRequest("该用户已经是核销员，请勿重复添加".to_string()));
        }

        let now = Local::now().naive_local();
        let model = system_store_staff::ActiveModel {
            uid: Set(request.uid as u32),
            avatar: Set(request.avatar.clone().unwrap_or_default()),
            store_id: Set(request.store_id),
            staff_name: Set(Some(request.staff_name.clone())),
            phone: Set(request.phone.clone()),
            verify_status: Set(if request.verify_status { 1 } else { 0 }),
            status: Set(Some(if request.status { 1 } else { 0 })),
            create_time: Set(now),
            update_time: Set(now),
            ..Default::default()
        };

        model.insert(db).await?;
        Ok(true)
    }

    /// 删除核销员
    ///
    /// Java: removeById
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let result = system_store_staff::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 修改核销员
    ///
    /// Java: SystemStoreStaffServiceImpl.edit()
    pub async fn edit(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemStoreStaffRequest,
    ) -> Result<bool> {
        let staff = Self::get_by_id(db, id).await?;
        let now = Local::now().naive_local();

        let mut active: system_store_staff::ActiveModel = staff.into();
        active.uid = Set(request.uid as u32);
        active.avatar = Set(request.avatar.clone().unwrap_or_default());
        active.store_id = Set(request.store_id);
        active.staff_name = Set(Some(request.staff_name.clone()));
        active.phone = Set(request.phone.clone());
        active.verify_status = Set(if request.verify_status { 1 } else { 0 });
        active.status = Set(Some(if request.status { 1 } else { 0 }));
        active.update_time = Set(now);
        active.update(db).await?;

        Ok(true)
    }

    /// 修改核销员状态
    ///
    /// Java: SystemStoreStaffServiceImpl.updateStatus()
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: i32,
    ) -> Result<bool> {
        let staff = Self::get_by_id(db, id).await?;
        if staff.status == Some(status) {
            return Ok(true);
        }
        let now = Local::now().naive_local();
        let mut active: system_store_staff::ActiveModel = staff.into();
        active.status = Set(Some(status));
        active.update_time = Set(now);
        active.update(db).await?;
        Ok(true)
    }

    /// 核销员详情
    ///
    /// Java: getById
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemStoreStaffInfoResponse> {
        let staff = Self::get_by_id(db, id).await?;
        Ok(SystemStoreStaffInfoResponse {
            id: staff.id,
            uid: staff.uid as i32,
            avatar: staff.avatar.clone(),
            store_id: staff.store_id,
            staff_name: staff.staff_name.clone(),
            phone: staff.phone.clone(),
            verify_status: staff.verify_status,
            status: staff.status,
            create_time: Some(staff.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(staff.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        })
    }

    // ==================== 辅助方法 ====================

    /// 根据id查找核销员
    async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<system_store_staff::Model> {
        system_store_staff::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("核销员不存在".to_string()))
    }

    /// 批量查询用户信息
    async fn get_user_map(
        db: &DatabaseConnection,
        user_ids: &[i32],
    ) -> Result<HashMap<i32, user::Model>> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }
        let users = user::Entity::find()
            .filter(user::Column::Uid.is_in(user_ids.to_vec()))
            .all(db)
            .await?;
        let mut map = HashMap::new();
        for u in users {
            map.insert(u.uid, u);
        }
        Ok(map)
    }

    /// Model 转 Response
    fn model_to_response(
        model: system_store_staff::Model,
        user_info: Option<StaffUserInfo>,
        store_info: Option<StaffStoreInfo>,
    ) -> SystemStoreStaffResponse {
        SystemStoreStaffResponse {
            id: model.id,
            uid: model.uid as i32,
            avatar: model.avatar,
            store_id: model.store_id,
            staff_name: model.staff_name,
            phone: model.phone,
            verify_status: model.verify_status,
            status: model.status,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            user: user_info,
            system_store: store_info,
        }
    }
}
