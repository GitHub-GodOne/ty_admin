/// 系统菜单 -- 服务层
///
/// 实现与Java版本一致的菜单管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.SystemMenuServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::dtos::system_menu::*;
use crate::models::_entities::system_menu;

/// Redis缓存key
const CACHE_LIST_KEY: &str = "menuList";

pub struct SystemMenuService;

impl SystemMenuService {
    /// 获取菜单列表
    ///
    /// Java: SystemMenuServiceImpl.getAdminList()
    /// 逻辑：
    /// 1. 过滤 is_delte = 0
    /// 2. 如果有 name，按 name LIKE 过滤
    /// 3. 如果有 menuType，按 menu_type 精确匹配
    /// 4. 排序: sort DESC, id ASC
    pub async fn get_admin_list(
        db: &DatabaseConnection,
        request: &SystemMenuSearchRequest,
    ) -> Result<Vec<SystemMenuListResponse>> {
        let mut query = system_menu::Entity::find()
            .filter(system_menu::Column::IsDelte.eq(0i16));

        // 按名称模糊搜索
        if let Some(name) = &request.name {
            let name = name.trim();
            if !name.is_empty() {
                query = query.filter(system_menu::Column::Name.contains(name));
            }
        }

        // 按菜单类型精确匹配
        if let Some(menu_type) = &request.menu_type {
            let mt = menu_type.trim();
            if !mt.is_empty() {
                query = query.filter(system_menu::Column::MenuType.eq(mt));
            }
        }

        // 排序: sort DESC, id ASC
        query = query
            .order_by_desc(system_menu::Column::Sort)
            .order_by_asc(system_menu::Column::Id);

        let records = query.all(db).await?;

        let list = records
            .into_iter()
            .map(Self::model_to_list_response)
            .collect();

        Ok(list)
    }

    /// 新增菜单
    ///
    /// Java: SystemMenuServiceImpl.add()
    /// 逻辑：
    /// 1. 菜单类型为C时，component不能为空
    /// 2. 菜单类型为A时，perms不能为空
    /// 3. 保存到数据库
    /// 4. 清除Redis缓存
    pub async fn add(
        db: &DatabaseConnection,
        request: &SystemMenuRequest,
    ) -> Result<bool> {
        // 验证
        Self::validate_menu(request)?;

        let now = Local::now().naive_local();

        let is_show: i16 = match request.is_show {
            Some(true) => 1,
            _ => 0,
        };

        let model = system_menu::ActiveModel {
            id: NotSet,
            pid: Set(request.pid),
            name: Set(Some(request.name.clone())),
            icon: Set(request.icon.clone()),
            perms: Set(request.perms.clone()),
            component: Set(request.component.clone()),
            menu_type: Set(Some(request.menu_type.clone())),
            sort: Set(request.sort),
            is_show: Set(is_show),
            is_delte: Set(0i16),
            create_time: Set(now),
            update_time: Set(now),
        };

        system_menu::Entity::insert(model).exec(db).await?;

        // 清除Redis缓存
        Self::clear_cache().await;

        Ok(true)
    }

    /// 删除菜单（软删除）
    ///
    /// Java: SystemMenuServiceImpl.deleteById()
    /// 逻辑：
    /// 1. 查找菜单
    /// 2. 设置 is_delte = 1
    /// 3. 递归删除所有子菜单（按pid查找）
    /// 4. 清除Redis缓存
    pub async fn delete_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        // 查找菜单
        let menu = system_menu::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        // 软删除当前菜单
        let mut active: system_menu::ActiveModel = menu.into();
        active.is_delte = Set(1i16);
        active.update(db).await?;

        // 递归删除子菜单
        Self::delete_children(db, id).await?;

        // 清除Redis缓存
        Self::clear_cache().await;

        Ok(true)
    }

    /// 递归删除子菜单
    async fn delete_children(db: &DatabaseConnection, pid: i32) -> Result<()> {
        let children = system_menu::Entity::find()
            .filter(system_menu::Column::Pid.eq(pid))
            .filter(system_menu::Column::IsDelte.eq(0i16))
            .all(db)
            .await?;

        for child in children {
            let child_id = child.id;
            let mut active: system_menu::ActiveModel = child.into();
            active.is_delte = Set(1i16);
            active.update(db).await?;

            // 递归删除子菜单的子菜单
            Box::pin(Self::delete_children(db, child_id)).await?;
        }

        Ok(())
    }

    /// 修改菜单
    ///
    /// Java: SystemMenuServiceImpl.edit()
    /// 逻辑：
    /// 1. 验证id不为空
    /// 2. 菜单类型为C时，component不能为空
    /// 3. 菜单类型为A时，perms不能为空
    /// 4. 更新数据库
    /// 5. 清除Redis缓存
    pub async fn edit(
        db: &DatabaseConnection,
        request: &SystemMenuRequest,
    ) -> Result<bool> {
        let id = request.id.ok_or_else(|| Error::BadRequest("菜单id不能为空".to_string()))?;

        // 验证
        Self::validate_menu(request)?;

        // 查找菜单
        let existing = system_menu::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let now = Local::now().naive_local();

        let is_show: i16 = match request.is_show {
            Some(true) => 1,
            _ => 0,
        };

        let mut active: system_menu::ActiveModel = existing.into();
        active.pid = Set(request.pid);
        active.name = Set(Some(request.name.clone()));
        active.icon = Set(request.icon.clone());
        active.perms = Set(request.perms.clone());
        active.component = Set(request.component.clone());
        active.menu_type = Set(Some(request.menu_type.clone()));
        active.sort = Set(request.sort);
        active.is_show = Set(is_show);
        active.update_time = Set(now);

        active.update(db).await?;

        // 清除Redis缓存
        Self::clear_cache().await;

        Ok(true)
    }

    /// 获取菜单详情
    ///
    /// Java: SystemMenuServiceImpl.getInfo()
    /// 逻辑：
    /// 1. 根据id查找
    /// 2. 检查未被删除
    /// 3. 返回不含isDelte/createTime/updateTime的响应
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemMenuResponse> {
        let menu = system_menu::Entity::find_by_id(id)
            .filter(system_menu::Column::IsDelte.eq(0i16))
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        Ok(Self::model_to_response(menu))
    }

    /// 切换显示状态
    ///
    /// Java: SystemMenuServiceImpl.updateShowStatus()
    /// 逻辑：
    /// 1. 查找菜单
    /// 2. 切换 is_show 状态
    /// 3. 清除Redis缓存
    pub async fn update_show_status(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let menu = system_menu::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let new_show: i16 = if menu.is_show == 0 { 1 } else { 0 };

        let mut active: system_menu::ActiveModel = menu.into();
        active.is_show = Set(new_show);
        active.update(db).await?;

        // 清除Redis缓存
        Self::clear_cache().await;

        Ok(true)
    }

    /// 获取缓存菜单树
    ///
    /// Java: SystemMenuServiceImpl.getCacheTree()
    /// 逻辑：
    /// 1. 从缓存/数据库获取所有未删除菜单
    /// 2. 转换为MenuCheckVo
    /// 3. 构建树结构（pid=0为根节点）
    pub async fn get_cache_tree(
        db: &DatabaseConnection,
    ) -> Result<Vec<MenuCheckVo>> {
        let menus = Self::get_cache_list(db).await?;

        // 转换为MenuCheckVo
        let check_list: Vec<MenuCheckVo> = menus
            .into_iter()
            .map(|m| MenuCheckVo {
                id: m.id,
                pid: m.pid,
                name: m.name.clone(),
                icon: m.icon.clone(),
                checked: false,
                sort: m.sort,
                child_list: vec![],
            })
            .collect();

        // 构建树
        Ok(Self::build_tree(check_list))
    }

    // ==================== 辅助方法 ====================

    /// 验证菜单请求
    fn validate_menu(request: &SystemMenuRequest) -> Result<()> {
        // 菜单类型为C时，component不能为空
        if request.menu_type == "C" {
            if request.component.as_ref().map_or(true, |c| c.trim().is_empty()) {
                return Err(Error::BadRequest("菜单类型为菜单时，组件路径不能为空".to_string()));
            }
        }

        // 菜单类型为A时，perms不能为空
        if request.menu_type == "A" {
            if request.perms.as_ref().map_or(true, |p| p.trim().is_empty()) {
                return Err(Error::BadRequest("菜单类型为按钮时，权限标识不能为空".to_string()));
            }
        }

        Ok(())
    }

    /// 从缓存/数据库获取所有未删除菜单
    ///
    /// Java: SystemMenuServiceImpl.getCacheList()
    pub async fn get_cache_list(
        db: &DatabaseConnection,
    ) -> Result<Vec<system_menu::Model>> {
        // 尝试从Redis获取
        if let Ok(redis) = crate::initializers::redis::get_redis().await {
            if let Ok(Some(cached)) = redis.get_json::<Vec<system_menu::Model>>(CACHE_LIST_KEY).await {
                return Ok(cached);
            }
        }

        // 从数据库查询
        let menus = system_menu::Entity::find()
            .filter(system_menu::Column::IsDelte.eq(0i16))
            .order_by_desc(system_menu::Column::Sort)
            .order_by_asc(system_menu::Column::Id)
            .all(db)
            .await?;

        // 存入Redis缓存（3600秒）
        if let Ok(redis) = crate::initializers::redis::get_redis().await {
            let _ = redis.set_json(CACHE_LIST_KEY, &menus, 3600).await;
        }

        Ok(menus)
    }

    /// 构建菜单树
    ///
    /// Java: MenuCheckTree.menuList()
    pub fn build_tree(list: Vec<MenuCheckVo>) -> Vec<MenuCheckVo> {
        // 找出根节点（pid=0）
        let mut roots: Vec<MenuCheckVo> = list
            .iter()
            .filter(|m| m.pid == 0)
            .cloned()
            .collect();

        // 递归填充子节点
        for root in &mut roots {
            Self::fill_children(root, &list);
        }

        roots
    }

    /// 递归填充子节点
    pub fn fill_children(parent: &mut MenuCheckVo, all: &[MenuCheckVo]) {
        let children: Vec<MenuCheckVo> = all
            .iter()
            .filter(|m| m.pid == parent.id)
            .cloned()
            .collect();

        for mut child in children {
            Self::fill_children(&mut child, all);
            parent.child_list.push(child);
        }
    }

    /// 清除Redis菜单缓存
    async fn clear_cache() {
        if let Ok(redis) = crate::initializers::redis::get_redis().await {
            let _ = redis.del(CACHE_LIST_KEY).await;
        }
    }

    /// Model 转 ListResponse（含全部字段）
    fn model_to_list_response(model: system_menu::Model) -> SystemMenuListResponse {
        SystemMenuListResponse {
            id: model.id,
            pid: model.pid,
            name: model.name,
            icon: model.icon,
            perms: model.perms,
            component: model.component,
            menu_type: model.menu_type,
            sort: model.sort,
            is_show: model.is_show != 0,
            is_delte: model.is_delte != 0,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }

    /// Model 转 Response（不含isDelte/createTime/updateTime）
    fn model_to_response(model: system_menu::Model) -> SystemMenuResponse {
        SystemMenuResponse {
            id: model.id,
            pid: model.pid,
            name: model.name,
            icon: model.icon,
            perms: model.perms,
            component: model.component,
            menu_type: model.menu_type,
            sort: model.sort,
            is_show: model.is_show != 0,
        }
    }
}
