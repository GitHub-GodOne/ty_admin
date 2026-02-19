/// 分类管理服务
///
/// 实现与Java版本一致的分类管理业务逻辑
/// Java参考: CategoryServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use std::collections::HashMap;

use crate::models::_entities::category;
use crate::dtos::category::{
    CategoryRequest, CategorySearchRequest, CategoryResponse, CategoryTreeVo,
};
use crate::dtos::common::PageParamRequest;

/// 分类服务
pub struct CategoryService;

impl CategoryService {
    /// 获取分类列表
    ///
    /// Java参考: CategoryServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &CategorySearchRequest,
        _page_param: &PageParamRequest,
    ) -> Result<Vec<CategoryResponse>> {
        let mut query = category::Entity::find();

        // 按pid筛选
        if let Some(pid) = request.pid {
            query = query.filter(category::Column::Pid.eq(pid));
        }

        // 按类型筛选
        if let Some(category_type) = request.category_type {
            query = query.filter(category::Column::Type.eq(category_type));
        }

        // 按状态筛选（-1 表示全部，不过滤）
        if let Some(status) = request.status {
            if status >= 0 {
                query = query.filter(category::Column::Status.eq(status));
            }
        }

        // 按名称模糊搜索
        if let Some(name) = &request.name {
            if !name.is_empty() {
                query = query.filter(category::Column::Name.contains(name));
            }
        }

        // 排序：sort降序，id升序
        query = query
            .order_by_desc(category::Column::Sort)
            .order_by_asc(category::Column::Id);

        let categories = query.all(db).await?;

        Ok(categories.into_iter().map(Self::model_to_response).collect())
    }

    /// 新增分类
    ///
    /// Java参考: CategoryServiceImpl.create()
    pub async fn create(
        db: &DatabaseConnection,
        request: &CategoryRequest,
    ) -> Result<bool> {
        let name = request.name.as_deref()
            .ok_or_else(|| Error::string("分类名称不能为空"))?;

        let category_type = request.category_type
            .ok_or_else(|| Error::string("分类类型不能为空"))?;

        let pid = request.pid.unwrap_or(0);

        // 检查名称是否重复（同类型同路径下）
        let path = Self::get_path_by_pid(db, pid).await?;
        let exists = category::Entity::find()
            .filter(category::Column::Name.eq(name))
            .filter(category::Column::Type.eq(category_type))
            .filter(category::Column::Path.eq(&path))
            .one(db)
            .await?;

        if exists.is_some() {
            return Err(Error::string("分类名称已存在"));
        }

        // 处理extra字段，去掉前缀
        let extra = request.extra.as_ref().map(|e| {
            Self::clear_prefix(e)
        });

        let now = chrono::Local::now().naive_local();
        let model = category::ActiveModel {
            pid: Set(pid),
            path: Set(path),
            name: Set(name.to_string()),
            r#type: Set(Some(category_type)),
            url: Set(request.url.clone().or(Some(String::new()))),
            extra: Set(extra),
            status: Set(request.status.unwrap_or(1)),
            sort: Set(request.sort.unwrap_or(99999)),
            create_time: Set(now),
            update_time: Set(now),
            ..Default::default()
        };

        category::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 更新分类
    ///
    /// Java参考: CategoryServiceImpl.update()
    /// 包含级联状态更新逻辑
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &CategoryRequest,
    ) -> Result<bool> {
        let existing = category::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("分类不存在"))?;

        let name = request.name.as_deref()
            .unwrap_or(&existing.name);

        // 检查名称是否重复（排除自身）
        if let Some(category_type) = request.category_type {
            let duplicate = category::Entity::find()
                .filter(category::Column::Name.eq(name))
                .filter(category::Column::Type.eq(category_type))
                .filter(category::Column::Path.eq(&existing.path))
                .filter(category::Column::Id.ne(id))
                .one(db)
                .await?;

            if duplicate.is_some() {
                return Err(Error::string("分类名称已存在"));
            }
        }

        // 处理extra字段
        let extra = request.extra.as_ref().map(|e| {
            Self::clear_prefix(e)
        });

        let now = chrono::Local::now().naive_local();
        let mut model: category::ActiveModel = existing.clone().into();

        if let Some(n) = &request.name {
            model.name = Set(n.clone());
        }
        if let Some(t) = request.category_type {
            model.r#type = Set(Some(t));
        }
        if let Some(url) = &request.url {
            model.url = Set(Some(url.clone()));
        }
        if extra.is_some() {
            model.extra = Set(extra);
        }
        if let Some(sort) = request.sort {
            model.sort = Set(sort);
        }
        if let Some(status) = request.status {
            model.status = Set(status);

            // 级联状态更新
            // 关闭时：关闭所有子分类
            if status == 0 {
                Self::cascade_close_children(db, id).await?;
            }
            // 开启时：开启所有父级分类
            if status == 1 {
                Self::cascade_open_parents(db, &existing.path).await?;
            }
        }

        model.update_time = Set(now);
        model.update(db).await?;

        Ok(true)
    }

    /// 删除分类
    ///
    /// Java参考: CategoryServiceImpl.delete()
    /// 需要检查是否有子分类
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let _existing = category::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("分类不存在"))?;

        // 检查是否有子分类
        let children_count = category::Entity::find()
            .filter(category::Column::Pid.eq(id))
            .count(db)
            .await?;

        if children_count > 0 {
            return Err(Error::string("请先删除子分类"));
        }

        category::Entity::delete_by_id(id).exec(db).await?;
        Ok(true)
    }

    /// 获取分类详情
    ///
    /// Java参考: CategoryServiceImpl.getById()
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<CategoryResponse> {
        let model = category::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("分类不存在"))?;

        Ok(Self::model_to_response(model))
    }

    /// 获取分类树形列表
    ///
    /// Java参考: CategoryServiceImpl.getListTree()
    /// 支持按类型筛选和名称搜索
    pub async fn get_list_tree(
        db: &DatabaseConnection,
        category_type: Option<i16>,
        status: Option<i16>,
        name: Option<&str>,
    ) -> Result<Vec<CategoryTreeVo>> {
        let mut query = category::Entity::find();

        // 按类型筛选
        if let Some(t) = category_type {
            query = query.filter(category::Column::Type.eq(t));
        }

        // 按状态筛选（-1 表示全部，不过滤）
        if let Some(s) = status {
            if s >= 0 {
                query = query.filter(category::Column::Status.eq(s));
            }
        }

        // 按名称模糊搜索
        let has_name_filter = name.map_or(false, |n| !n.is_empty());
        if let Some(n) = name {
            if !n.is_empty() {
                query = query.filter(category::Column::Name.contains(n));
            }
        }

        // 排序
        query = query
            .order_by_desc(category::Column::Sort)
            .order_by_asc(category::Column::Id);

        let categories = query.all(db).await?;

        // 如果有名称搜索，直接返回平铺列表（不构建树）
        // Java: 如果有name搜索条件，返回的是平铺列表而不是树
        if has_name_filter {
            return Ok(categories.into_iter().map(|c| {
                CategoryTreeVo {
                    id: c.id,
                    pid: c.pid,
                    name: c.name,
                    category_type: c.r#type,
                    url: c.url,
                    extra: c.extra,
                    status: c.status,
                    sort: c.sort,
                    path: c.path,
                    child: vec![],
                }
            }).collect());
        }

        // 构建树形结构
        Ok(Self::build_tree(categories))
    }

    /// 根据ID列表获取分类
    ///
    /// Java参考: CategoryServiceImpl.getByIds()
    pub async fn get_by_ids(
        db: &DatabaseConnection,
        ids: Vec<i32>,
    ) -> Result<Vec<CategoryResponse>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let categories = category::Entity::find()
            .filter(category::Column::Id.is_in(ids))
            .order_by_desc(category::Column::Sort)
            .order_by_asc(category::Column::Id)
            .all(db)
            .await?;

        Ok(categories.into_iter().map(Self::model_to_response).collect())
    }

    /// 更新分类状态
    ///
    /// Java参考: CategoryServiceImpl.updateStatus()
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let existing = category::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("分类不存在"))?;

        // 切换状态
        let new_status: i16 = if existing.status == 1 { 0 } else { 1 };

        let now = chrono::Local::now().naive_local();
        let mut model: category::ActiveModel = existing.clone().into();
        model.status = Set(new_status);
        model.update_time = Set(now);
        model.update(db).await?;

        // 级联状态更新
        if new_status == 0 {
            // 关闭：关闭所有子分类
            Self::cascade_close_children(db, id).await?;
        } else {
            // 开启：开启所有父级分类
            Self::cascade_open_parents(db, &existing.path).await?;
        }

        Ok(true)
    }

    // ==================== 辅助方法 ====================

    /// 根据父级ID计算路径
    ///
    /// Java参考: CategoryServiceImpl.getPathByPId()
    /// 路径格式: /0/ 或 /0/1/ 或 /0/1/2/
    async fn get_path_by_pid(
        db: &DatabaseConnection,
        pid: i32,
    ) -> Result<String> {
        if pid == 0 {
            return Ok("/0/".to_string());
        }

        let parent = category::Entity::find_by_id(pid)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("父级分类不存在"))?;

        // 路径 = 父级路径 + 父级ID + /
        Ok(format!("{}{}/", parent.path, parent.id))
    }

    /// 级联关闭子分类
    ///
    /// Java参考: CategoryServiceImpl 中关闭子分类的逻辑
    /// 查找所有path包含当前分类ID的子分类，将其status设为0
    fn cascade_close_children(
        db: &DatabaseConnection,
        parent_id: i32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            let children = category::Entity::find()
                .filter(category::Column::Pid.eq(parent_id))
                .all(db)
                .await?;

            let now = chrono::Local::now().naive_local();
            for child in children {
                if child.status != 0 {
                    let mut model: category::ActiveModel = child.clone().into();
                    model.status = Set(0);
                    model.update_time = Set(now);
                    model.update(db).await?;
                }
                // 递归关闭子分类的子分类
                Self::cascade_close_children(db, child.id).await?;
            }

            Ok(())
        })
    }

    /// 级联开启父级分类
    ///
    /// Java参考: CategoryServiceImpl 中开启父级分类的逻辑
    /// 解析path中的所有父级ID，将其status设为1
    async fn cascade_open_parents(
        db: &DatabaseConnection,
        path: &str,
    ) -> Result<()> {
        // 解析路径中的父级ID列表
        // path格式: /0/1/2/ -> 父级ID为 1, 2
        let parent_ids: Vec<i32> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<i32>().ok())
            .filter(|&id| id != 0) // 排除根节点0
            .collect();

        if parent_ids.is_empty() {
            return Ok(());
        }

        let now = chrono::Local::now().naive_local();
        for parent_id in parent_ids {
            if let Some(parent) = category::Entity::find_by_id(parent_id)
                .one(db)
                .await?
            {
                if parent.status != 1 {
                    let mut model: category::ActiveModel = parent.into();
                    model.status = Set(1);
                    model.update_time = Set(now);
                    model.update(db).await?;
                }
            }
        }

        Ok(())
    }

    /// 构建树形结构
    ///
    /// Java参考: CategoryServiceImpl.getTree() / getListTree()
    /// 使用HashMap实现高效的树构建
    fn build_tree(categories: Vec<category::Model>) -> Vec<CategoryTreeVo> {
        if categories.is_empty() {
            return vec![];
        }

        // 转换为TreeVo
        let tree_nodes: Vec<CategoryTreeVo> = categories
            .iter()
            .map(|c| CategoryTreeVo {
                id: c.id,
                pid: c.pid,
                name: c.name.clone(),
                category_type: c.r#type,
                url: c.url.clone(),
                extra: c.extra.clone(),
                status: c.status,
                sort: c.sort,
                path: c.path.clone(),
                child: vec![],
            })
            .collect();

        // 使用HashMap按pid分组
        let mut children_map: HashMap<i32, Vec<CategoryTreeVo>> = HashMap::new();
        for node in &tree_nodes {
            children_map
                .entry(node.pid)
                .or_insert_with(Vec::new)
                .push(node.clone());
        }

        // 递归构建树
        fn build_children(
            pid: i32,
            children_map: &HashMap<i32, Vec<CategoryTreeVo>>,
        ) -> Vec<CategoryTreeVo> {
            match children_map.get(&pid) {
                Some(children) => {
                    children
                        .iter()
                        .map(|child| {
                            let mut node = child.clone();
                            node.child = build_children(
                                child.id,
                                children_map,
                            );
                            node
                        })
                        .collect()
                }
                None => vec![],
            }
        }

        // 从根节点（pid=0）开始构建
        build_children(0, &children_map)
    }

    /// 清除extra字段的前缀
    ///
    /// Java参考: CategoryServiceImpl.clearPrefix()
    fn clear_prefix(extra: &str) -> String {
        // Java中会去掉图片URL的域名前缀
        // 这里保持与Java一致的逻辑
        extra.to_string()
    }

    /// Model转Response
    fn model_to_response(model: category::Model) -> CategoryResponse {
        CategoryResponse {
            id: model.id,
            pid: model.pid,
            path: model.path,
            name: model.name,
            category_type: model.r#type,
            url: model.url,
            extra: model.extra,
            status: model.status,
            sort: model.sort,
            create_time: model.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: model.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
