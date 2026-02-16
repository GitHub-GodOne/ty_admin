/// 用户分组服务
///
/// 实现与Java版本一致的用户分组管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;

use crate::models::_entities::user_group;
use crate::dtos::user_group::{UserGroupRequest, UserGroupResponse};
use crate::dtos::common::{PageParamRequest, CommonPage};

/// 用户分组服务
pub struct UserGroupService;

impl UserGroupService {
    /// 获取用户分组列表（分页）
    ///
    /// Java参考: UserGroupServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserGroupResponse>> {
        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        // 构建查询
        let query = user_group::Entity::find()
            .order_by_desc(user_group::Column::Id);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let groups = paginator.fetch_page((page - 1) as u64).await?;

        // 转换为响应格式
        let list: Vec<UserGroupResponse> = groups
            .into_iter()
            .map(|g| Self::model_to_response(g))
            .collect();

        Ok(CommonPage {
            list,
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 创建用户分组
    ///
    /// Java参考: UserGroupServiceImpl.create()
    pub async fn create(
        db: &DatabaseConnection,
        request: &UserGroupRequest,
    ) -> Result<bool> {
        let group = user_group::ActiveModel {
            group_name: Set(Some(request.group_name.clone())),
            ..Default::default()
        };

        user_group::Entity::insert(group).exec(db).await?;
        Ok(true)
    }

    /// 删除用户分组
    ///
    /// Java参考: UserGroupServiceImpl.removeById()
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let result = user_group::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 更新用户分组
    ///
    /// Java参考: UserGroupServiceImpl.edit()
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &UserGroupRequest,
    ) -> Result<bool> {
        let group = user_group::ActiveModel {
            id: Set(id),
            group_name: Set(Some(request.group_name.clone())),
        };

        group.update(db).await?;
        Ok(true)
    }

    /// 获取用户分组详情
    ///
    /// Java参考: UserGroupServiceImpl.getById()
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<UserGroupResponse>> {
        let group = user_group::Entity::find_by_id(id).one(db).await?;
        Ok(group.map(|g| Self::model_to_response(g)))
    }

    /// 将Model转换为Response
    fn model_to_response(model: user_group::Model) -> UserGroupResponse {
        UserGroupResponse {
            id: model.id,
            group_name: model.group_name,
        }
    }
}
