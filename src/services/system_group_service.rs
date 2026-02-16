/// 组合数据 -- 服务层
///
/// Java参考: SystemGroupServiceImpl
use sea_orm::*;
use chrono::Local;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::system_group::*;
use crate::models::_entities::{system_group, user};

pub struct SystemGroupService;

impl SystemGroupService {
    /// 分页列表
    /// Java: SystemGroupServiceImpl.getList()
    /// 条件: keywords模糊匹配name; 排序: id DESC
    pub async fn get_list(
        db: &DatabaseConnection,
        search: &SystemGroupSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SystemGroupResponse>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = system_group::Entity::find();

        // keywords 模糊搜索 name
        if let Some(keywords) = &search.keywords {
            if !keywords.trim().is_empty() {
                query = query.filter(system_group::Column::Name.contains(keywords.trim()));
            }
        }

        // 排序: id DESC
        query = query.order_by_desc(system_group::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list = records.into_iter().map(Self::model_to_response).collect();
        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 新增组合数据
    /// Java: SystemGroupServiceImpl.add()
    pub async fn add(
        db: &DatabaseConnection,
        request: &SystemGroupRequest,
    ) -> Result<bool, DbErr> {
        let now = Local::now().naive_local();
        let active = system_group::ActiveModel {
            name: Set(request.name.clone().unwrap_or_default()),
            info: Set(request.info.clone().unwrap_or_default()),
            form_id: Set(request.form_id.unwrap_or(0)),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };
        active.insert(db).await?;
        Ok(true)
    }

    /// 删除组合数据
    /// Java: SystemGroupServiceImpl.delete()
    /// 删除后清除用户对应的group_id
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, DbErr> {
        let result = system_group::Entity::delete_by_id(id).exec(db).await?;
        if result.rows_affected > 0 {
            // 清除用户对应已经存在的分组标签
            // Java: userService.clearGroupByGroupId(id.toString())
            let id_str = id.to_string();
            let users = user::Entity::find()
                .filter(user::Column::GroupId.eq(&id_str))
                .all(db)
                .await?;
            for u in users {
                let mut active: user::ActiveModel = u.into();
                active.group_id = Set(Some(String::new()));
                active.update(db).await?;
            }
        }
        Ok(result.rows_affected > 0)
    }

    /// 修改组合数据
    /// Java: SystemGroupServiceImpl.edit()
    pub async fn edit(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemGroupRequest,
    ) -> Result<bool, DbErr> {
        let record = system_group::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("组合数据不存在".to_string()))?;

        let mut active: system_group::ActiveModel = record.into();
        active.name = Set(request.name.clone().unwrap_or_default());
        active.info = Set(request.info.clone().unwrap_or_default());
        active.form_id = Set(request.form_id.unwrap_or(0));
        active.update_time = Set(Some(Local::now().naive_local()));
        active.update(db).await?;
        Ok(true)
    }

    /// 查询组合数据详情
    /// Java: systemGroupService.getById(id)
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemGroupResponse, DbErr> {
        let record = system_group::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::Custom("组合数据不存在".to_string()))?;
        Ok(Self::model_to_response(record))
    }

    fn model_to_response(model: system_group::Model) -> SystemGroupResponse {
        SystemGroupResponse {
            id: model.id,
            name: model.name,
            info: model.info,
            form_id: model.form_id,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
