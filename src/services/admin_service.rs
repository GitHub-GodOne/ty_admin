/// 管理员服务层
///
/// 处理管理员相关的业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;

use crate::common::error::{AppError, AppResult};
use crate::dtos::admin::*;
use crate::models::_entities::{system_admin, system_menu, system_role, system_role_menu};
use crate::utils::{auth, crypto};

pub struct AdminService;

impl AdminService {
    /// 管理员登录
    pub async fn login(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        request: &AdminLoginRequest,
    ) -> AppResult<AdminLoginResponse> {
        // 1. 查找管理员
        let admin = system_admin::Entity::find()
            .filter(system_admin::Column::Account.eq(&request.account))
            .filter(system_admin::Column::IsDel.eq(0))
            .one(db)
            .await?
            .ok_or_else(|| AppError::Unauthorized("用户不存在或密码错误".to_string()))?;

        // 2. 检查账号状态
        if admin.status == 0 {
            return Err(AppError::BusinessError("账号已被禁用".to_string()));
        }

        // 3. 验证密码
        let encrypted_pwd = crypto::encrypt_password(&request.pwd, &admin.account);
        if admin.pwd.trim() != encrypted_pwd {
            return Err(AppError::Unauthorized("用户不存在或密码错误".to_string()));
        }

        // 4. 生成权限列表
        let permissions = if admin.roles.split(',').any(|r| r == "1") {
            vec!["*:*:*".to_string()] // 超级管理员拥有所有权限
        } else {
            // TODO: 从数据库查询用户的实际权限列表
            vec![]
        };

        // 5. 生成 Token
        let token = auth::create_token(
            redis,
            admin.id,
            admin.account.clone(),
            admin.roles.clone(),
            permissions,
        )
        .await
        .map_err(|e| AppError::InternalError(format!("创建Token失败: {}", e)))?;

        // 6. 更新登录信息
        let mut admin_active: system_admin::ActiveModel = admin.clone().into();
        admin_active.login_count = Set(admin.login_count + 1);
        admin_active.update_time = Set(chrono::Utc::now().naive_utc());
        admin_active.update(db).await?;

        // 7. 返回响应
        Ok(AdminLoginResponse {
            id: admin.id,
            account: admin.account,
            real_name: admin.real_name,
            token,
            is_sms: admin.is_sms.unwrap_or(0) == 1,
        })
    }

    /// 获取管理员信息（通过Token）
    ///
    /// Java: AdminLoginServiceImpl.getInfoByToken()
    /// 流程：
    /// 1. 从Redis获取LoginUserVo
    /// 2. 从数据库查询完整的SystemAdmin记录
    /// 3. 查询角色名称
    /// 4. 构建权限列表
    /// 5. 组装SystemAdminResponse返回
    pub async fn get_admin_info_by_token(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        token: &str,
    ) -> AppResult<AdminInfoResponse> {
        // 1. 从Redis获取登录用户信息
        let login_user = auth::get_login_user(redis, token)
            .await
            .ok_or_else(|| AppError::Unauthorized("未登录或Token已过期".to_string()))?;

        // 2. 验证Token是否有效
        if !auth::verify_token(&login_user) {
            return Err(AppError::Unauthorized("Token已过期".to_string()));
        }

        // 3. 从数据库查询完整的管理员记录（Java: loginUserVo.getUser()）
        let admin = system_admin::Entity::find_by_id(login_user.user_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("管理员不存在".to_string()))?;

        // 4. 查询角色名称
        let role_ids: Vec<i32> = admin
            .roles
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let role_names = if !role_ids.is_empty() {
            let roles = system_role::Entity::find()
                .filter(system_role::Column::Id.is_in(role_ids))
                .all(db)
                .await?;
            let names: Vec<String> = roles.into_iter().map(|r| r.role_name).collect();
            Some(names.join(","))
        } else {
            None
        };

        // 5. 构建权限列表（Java: getInfoByToken中的权限逻辑）
        let permissions_list = if admin.roles.split(',').any(|r| r == "1") {
            // 超级管理员拥有所有权限
            vec!["*:*:*".to_string()]
        } else {
            // 从LoginUserVo中获取权限（登录时已存储）
            login_user.permissions
        };

        // 6. 组装响应（Java: BeanUtils.copyProperties(systemAdmin, systemAdminResponse)）
        Ok(AdminInfoResponse {
            id: admin.id,
            account: admin.account,
            real_name: admin.real_name,
            roles: admin.roles,
            role_names,
            last_ip: admin.last_ip,
            last_time: Some(admin.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            add_time: Some(admin.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            login_count: admin.login_count,
            level: admin.level as i32,
            status: admin.status != 0,
            phone: admin.phone,
            is_sms: admin.is_sms.unwrap_or(0) != 0,
            permissions_list,
            token: None,
        })
    }

    /// 获取管理员菜单
    pub async fn get_menus(
        db: &DatabaseConnection,
        redis: &crate::utils::redis_client::RedisClient,
        token: &str,
    ) -> AppResult<Vec<MenuResponse>> {
        // 1. 从Redis获取登录用户信息
        let login_user = auth::get_login_user(redis, token)
            .await
            .ok_or_else(|| AppError::Unauthorized("未登录或Token已过期".to_string()))?;

        // 2. 判断是否超级管理员
        let is_super_admin = login_user.roles.split(',').any(|r| r == "1");

        // 3. 查询菜单
        let menus = if is_super_admin {
            Self::get_all_menus(db).await?
        } else {
            Self::get_role_menus(db, &login_user.roles).await?
        };

        // 4. 构建菜单树
        Ok(Self::build_menu_tree(menus))
    }

    /// 获取所有菜单
    async fn get_all_menus(db: &DatabaseConnection) -> AppResult<Vec<system_menu::Model>> {
        system_menu::Entity::find()
            .filter(system_menu::Column::IsDelte.eq(0))
            .filter(system_menu::Column::IsShow.eq(1))
            .filter(system_menu::Column::MenuType.ne("A"))
            .order_by_desc(system_menu::Column::Sort)
            .order_by_asc(system_menu::Column::Id)
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// 获取角色菜单
    async fn get_role_menus(
        db: &DatabaseConnection,
        roles: &str,
    ) -> AppResult<Vec<system_menu::Model>> {
        // 解析角色ID列表
        let role_ids: Vec<i32> = roles
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        // 查询角色对应的菜单ID
        let role_menus = system_role_menu::Entity::find()
            .filter(system_role_menu::Column::Rid.is_in(role_ids))
            .all(db)
            .await?;

        let menu_ids: Vec<i32> = role_menus.iter().map(|rm| rm.menu_id).collect();

        if menu_ids.is_empty() {
            return Ok(vec![]);
        }

        // 查询菜单
        system_menu::Entity::find()
            .filter(system_menu::Column::Id.is_in(menu_ids))
            .filter(system_menu::Column::IsDelte.eq(0))
            .filter(system_menu::Column::IsShow.eq(1))
            .filter(system_menu::Column::MenuType.ne("A"))
            .order_by_desc(system_menu::Column::Sort)
            .order_by_asc(system_menu::Column::Id)
            .all(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// 构建菜单树
    fn build_menu_tree(menus: Vec<system_menu::Model>) -> Vec<MenuResponse> {
        // 转换为 MenuResponse
        let menu_responses: Vec<MenuResponse> = menus
            .into_iter()
            .map(|m| MenuResponse {
                id: m.id,
                name: m.name.unwrap_or_default(),
                pid: m.pid,
                path: "".to_string(), // system_menu 表没有 path 字段，使用空字符串
                icon: m.icon,
                menu_type: m.menu_type.unwrap_or_default(),
                component: m.component,
                perms: m.perms,
                is_show: m.is_show == 1,
                sort: m.sort,
                child_list: vec![],
            })
            .collect();

        // 构建树形结构
        Self::build_tree_recursive(&menu_responses, 0)
    }

    /// 递归构建树形结构
    fn build_tree_recursive(all_menus: &[MenuResponse], parent_id: i32) -> Vec<MenuResponse> {
        all_menus
            .iter()
            .filter(|m| m.pid == parent_id)
            .map(|m| {
                let mut menu = m.clone();
                menu.child_list = Self::build_tree_recursive(all_menus, m.id);
                menu
            })
            .collect()
    }

    /// 账号检测（检查账号是否存在）
    pub async fn account_detection(
        db: &DatabaseConnection,
        account: &str,
        exclude_id: Option<i32>,
    ) -> AppResult<bool> {
        let mut query = system_admin::Entity::find()
            .filter(system_admin::Column::Account.eq(account))
            .filter(system_admin::Column::IsDel.eq(0));

        if let Some(id) = exclude_id {
            query = query.filter(system_admin::Column::Id.ne(id));
        }

        let exists = query.one(db).await?.is_some();
        Ok(exists)
    }

    /// 修改登录用户信息
    pub async fn update_login_admin(
        db: &DatabaseConnection,
        admin_id: i32,
        real_name: &str,
    ) -> AppResult<()> {
        let admin = system_admin::Entity::find_by_id(admin_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("管理员不存在".to_string()))?;

        let mut admin_active: system_admin::ActiveModel = admin.into();
        admin_active.real_name = Set(real_name.to_string());
        admin_active.update_time = Set(chrono::Utc::now().naive_utc());
        admin_active.update(db).await?;

        Ok(())
    }

    /// 修改登录用户密码
    pub async fn update_password(
        db: &DatabaseConnection,
        admin_id: i32,
        old_password: &str,
        new_password: &str,
    ) -> AppResult<()> {
        // 1. 查找管理员
        let admin = system_admin::Entity::find_by_id(admin_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound("管理员不存在".to_string()))?;

        // 2. 验证旧密码
        let encrypted_old_pwd = crypto::encrypt_password(old_password, &admin.account);
        if admin.pwd.trim() != encrypted_old_pwd {
            return Err(AppError::BusinessError("原密码错误".to_string()));
        }

        // 3. 加密新密码
        let encrypted_new_pwd = crypto::encrypt_password(new_password, &admin.account);

        // 4. 更新密码
        let mut admin_active: system_admin::ActiveModel = admin.into();
        admin_active.pwd = Set(encrypted_new_pwd);
        admin_active.update_time = Set(chrono::Utc::now().naive_utc());
        admin_active.update(db).await?;

        Ok(())
    }
}
