/// 用户标签服务
///
/// 实现与Java版本一致的用户标签管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;

use crate::models::_entities::user_tag;
use crate::dtos::user_tag::{UserTagRequest, UserTagResponse};
use crate::dtos::common::{PageParamRequest, CommonPage};

/// 用户标签服务
pub struct UserTagService;

impl UserTagService {
    /// 获取用户标签列表（分页）
    ///
    /// Java参考: UserTagServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserTagResponse>> {
        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        // 构建查询
        let query = user_tag::Entity::find()
            .order_by_desc(user_tag::Column::Id);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let tags = paginator.fetch_page((page - 1) as u64).await?;

        // 转换为响应格式
        let list: Vec<UserTagResponse> = tags
            .into_iter()
            .map(|t| Self::model_to_response(t))
            .collect();

        Ok(CommonPage {
            list,
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 创建用户标签
    ///
    /// Java参考: UserTagServiceImpl.create()
    pub async fn create(
        db: &DatabaseConnection,
        request: &UserTagRequest,
    ) -> Result<bool> {
        let tag = user_tag::ActiveModel {
            name: Set(Some(request.name.clone())),
            ..Default::default()
        };

        user_tag::Entity::insert(tag).exec(db).await?;
        Ok(true)
    }

    /// 删除用户标签
    ///
    /// Java参考: UserTagServiceImpl.delete()
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let result = user_tag::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 更新用户标签
    ///
    /// Java参考: UserTagServiceImpl.updateTag()
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &UserTagRequest,
    ) -> Result<bool> {
        let tag = user_tag::ActiveModel {
            id: Set(id),
            name: Set(Some(request.name.clone())),
        };

        tag.update(db).await?;
        Ok(true)
    }

    /// 获取用户标签详情
    ///
    /// Java参考: UserTagServiceImpl.getById()
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<UserTagResponse>> {
        let tag = user_tag::Entity::find_by_id(id).one(db).await?;
        Ok(tag.map(|t| Self::model_to_response(t)))
    }

    /// 将Model转换为Response
    fn model_to_response(model: user_tag::Model) -> UserTagResponse {
        UserTagResponse {
            id: model.id,
            name: model.name,
        }
    }
}
