/// 系统角色 -- 服务层
///
/// 实现与Java版本一致的角色管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.SystemRoleServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::system_menu::MenuCheckVo;
use crate::dtos::system_role::*;
use crate::models::_entities::{system_role, system_role_menu};
use crate::services::system_menu_service::SystemMenuService;

pub struct SystemRoleService;

impl SystemRoleService {
    /// 分页列表
    ///
    /// Java: SystemRoleServiceImpl.getList()
    /// 逻辑：
    /// 1. 按 status 过滤（可选）
    /// 2. 按 roleName LIKE 过滤（可选）
    /// 3. 按 id ASC 排序
    /// 4. 分页返回（只返回 id, roleName, status, createTime, updateTime）
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &SystemRoleSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SystemRoleListResponse>> {
        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        let mut query = system_role::Entity::find();

        // 按状态过滤
        if let Some(status) = request.status {
            let status_val: i16 = if status { 1 } else { 0 };
            query = query.filter(system_role::Column::Status.eq(status_val));
        }

        // 按角色名称模糊搜索
        if let Some(role_name) = &request.role_name {
            let rn = role_name.trim();
            if !rn.is_empty() {
                query = query.filter(system_role::Column::RoleName.contains(rn));
            }
        }

        // 按 id ASC 排序
        query = query.order_by_asc(system_role::Column::Id);

        // 查询总数
        let total = query.clone().count(db).await? as i64;

        // 分页查询
        let records = query
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        let list: Vec<SystemRoleListResponse> = records
            .into_iter()
            .map(Self::model_to_list_response)
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }

    /// 新增角色
    ///
    /// Java: SystemRoleServiceImpl.add()
    /// 逻辑：
    /// 1. 检查角色名称是否重复
    /// 2. 解析 rules 为菜单id列表
    /// 3. 保存角色
    /// 4. 批量保存角色-菜单关联
    pub async fn add(
        db: &DatabaseConnection,
        request: &SystemRoleRequest,
    ) -> Result<bool> {
        // 检查角色名称是否重复
        if Self::exist_name(db, &request.role_name, None).await? {
            return Err(Error::BadRequest("角色名称重复".to_string()));
        }

        // 解析 rules
        let rule_list = Self::parse_rules(&request.rules)?;

        let now = Local::now().naive_local();
        let status_val: i16 = if request.status { 1 } else { 0 };

        let role_model = system_role::ActiveModel {
            id: NotSet,
            role_name: Set(request.role_name.clone()),
            rules: Set(String::new()),
            level: Set(0i16),
            status: Set(status_val),
            create_time: Set(now),
            update_time: Set(now),
        };

        // 插入角色并获取id
        let result = system_role::Entity::insert(role_model).exec(db).await?;
        let role_id = result.last_insert_id;

        // 批量保存角色-菜单关联
        Self::save_role_menus(db, role_id, &rule_list).await?;

        Ok(true)
    }

    /// 修改角色
    ///
    /// Java: SystemRoleServiceImpl.edit()
    /// 逻辑：
    /// 1. 检查角色是否存在
    /// 2. 如果名称变了，检查是否重复
    /// 3. 更新角色
    /// 4. 删除旧的角色-菜单关联
    /// 5. 批量保存新的角色-菜单关联
    pub async fn edit(
        db: &DatabaseConnection,
        request: &SystemRoleRequest,
    ) -> Result<bool> {
        let id = request.id.ok_or_else(|| Error::BadRequest("角色id不能为空".to_string()))?;

        let existing = system_role::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("角色不存在".to_string()))?;

        // 如果名称变了，检查是否重复
        if existing.role_name != request.role_name {
            if Self::exist_name(db, &request.role_name, Some(id)).await? {
                return Err(Error::BadRequest("角色名称重复".to_string()));
            }
        }

        // 解析 rules
        let rule_list = Self::parse_rules(&request.rules)?;

        let now = Local::now().naive_local();
        let status_val: i16 = if request.status { 1 } else { 0 };

        let mut active: system_role::ActiveModel = existing.into();
        active.role_name = Set(request.role_name.clone());
        active.rules = Set(String::new());
        active.status = Set(status_val);
        active.update_time = Set(now);
        active.update(db).await?;

        // 删除旧的角色-菜单关联
        Self::delete_role_menus(db, id).await?;

        // 批量保存新的角色-菜单关联
        Self::save_role_menus(db, id, &rule_list).await?;

        Ok(true)
    }
    /// 删除角色
    ///
    /// Java: SystemRoleServiceImpl.delete()
    /// 逻辑：
    /// 1. 检查角色是否存在
    /// 2. 删除角色（物理删除）
    /// 3. 删除角色-菜单关联
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let _existing = system_role::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("角色已删除".to_string()))?;

        // 物理删除角色
        system_role::Entity::delete_by_id(id).exec(db).await?;

        // 删除角色-菜单关联
        Self::delete_role_menus(db, id).await?;

        Ok(true)
    }

    /// 获取角色详情
    ///
    /// Java: SystemRoleServiceImpl.getInfo()
    /// 逻辑：
    /// 1. 查找角色
    /// 2. 获取所有菜单列表（从缓存）
    /// 3. 获取该角色关联的菜单id列表
    /// 4. 构建菜单树，标记已选中的菜单
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<RoleInfoResponse> {
        let role = system_role::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("角色不存在".to_string()))?;

        // 获取所有菜单列表
        let menu_list = SystemMenuService::get_cache_list(db).await?;

        // 获取该角色关联的菜单id列表
        let menu_id_list = Self::get_menu_list_by_rid(db, id).await?;

        // 转换为MenuCheckVo，标记已选中
        let check_list: Vec<MenuCheckVo> = menu_list
            .into_iter()
            .map(|menu| {
                let checked = menu_id_list.contains(&menu.id);
                MenuCheckVo {
                    id: menu.id,
                    pid: menu.pid,
                    name: menu.name.clone(),
                    icon: menu.icon.clone(),
                    checked,
                    sort: menu.sort,
                    child_list: vec![],
                }
            })
            .collect();

        // 构建菜单树
        let menu_tree = SystemMenuService::build_tree(check_list);

        Ok(RoleInfoResponse {
            id: role.id,
            role_name: role.role_name.clone(),
            status: role.status != 0,
            create_time: Some(role.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(role.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            menu_list: menu_tree,
        })
    }

    /// 修改角色状态
    ///
    /// Java: SystemRoleServiceImpl.updateStatus()
    /// 逻辑：
    /// 1. 查找角色
    /// 2. 如果状态相同则直接返回
    /// 3. 更新状态
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let role = system_role::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("身份不存在".to_string()))?;

        let status_val: i16 = if status { 1 } else { 0 };

        // 如果状态相同则直接返回
        if role.status == status_val {
            return Ok(true);
        }

        let now = Local::now().naive_local();
        let mut active: system_role::ActiveModel = role.into();
        active.status = Set(status_val);
        active.update_time = Set(now);
        active.update(db).await?;

        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 检查角色名称是否存在
    async fn exist_name(db: &DatabaseConnection, role_name: &str, exclude_id: Option<i32>) -> Result<bool> {
        let mut query = system_role::Entity::find()
            .filter(system_role::Column::RoleName.eq(role_name));

        if let Some(id) = exclude_id {
            query = query.filter(system_role::Column::Id.ne(id));
        }

        let count = query.count(db).await?;
        Ok(count > 0)
    }

    /// 解析 rules 字符串为菜单id列表（去重）
    fn parse_rules(rules: &str) -> Result<Vec<i32>> {
        let list: Vec<i32> = rules
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::BadRequest(format!("权限格式错误: {}", e)))?;

        // 去重
        let mut unique = list;
        unique.sort();
        unique.dedup();
        Ok(unique)
    }

    /// 获取角色关联的菜单id列表
    async fn get_menu_list_by_rid(db: &DatabaseConnection, rid: i32) -> Result<Vec<i32>> {
        let records = system_role_menu::Entity::find()
            .filter(system_role_menu::Column::Rid.eq(rid))
            .all(db)
            .await?;

        Ok(records.into_iter().map(|r| r.menu_id).collect())
    }

    /// 删除角色的所有菜单关联
    async fn delete_role_menus(db: &DatabaseConnection, rid: i32) -> Result<()> {
        system_role_menu::Entity::delete_many()
            .filter(system_role_menu::Column::Rid.eq(rid))
            .exec(db)
            .await?;
        Ok(())
    }

    /// 批量保存角色-菜单关联
    async fn save_role_menus(db: &DatabaseConnection, rid: i32, menu_ids: &[i32]) -> Result<()> {
        if menu_ids.is_empty() {
            return Ok(());
        }

        let models: Vec<system_role_menu::ActiveModel> = menu_ids
            .iter()
            .map(|&menu_id| system_role_menu::ActiveModel {
                rid: Set(rid),
                menu_id: Set(menu_id),
            })
            .collect();

        system_role_menu::Entity::insert_many(models)
            .exec(db)
            .await?;

        Ok(())
    }

    /// Model 转 ListResponse
    fn model_to_list_response(model: system_role::Model) -> SystemRoleListResponse {
        SystemRoleListResponse {
            id: model.id,
            role_name: model.role_name,
            status: model.status != 0,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
