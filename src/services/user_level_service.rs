/// 用户等级服务
///
/// 实现与Java版本一致的用户等级管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;

use crate::models::_entities::system_user_level;
use crate::dtos::user_level::{UserLevelResponse, UserLevelSaveRequest};
use crate::dtos::common::{PageParamRequest, CommonPage};

/// 用户等级服务
pub struct UserLevelService;

impl UserLevelService {
    /// 获取系统用户等级列表（分页）
    ///
    /// Java参考: SystemUserLevelServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserLevelResponse>> {
        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        // 构建查询 - 排除已删除的记录，按grade排序
        let query = system_user_level::Entity::find()
            .filter(system_user_level::Column::IsDel.eq(0))
            .order_by_asc(system_user_level::Column::Grade);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let levels = paginator.fetch_page((page - 1) as u64).await?;

        // 转换为响应格式
        let list: Vec<UserLevelResponse> = levels
            .into_iter()
            .map(|l| Self::model_to_response(l))
            .collect();

        Ok(CommonPage {
            list,
            total: total as i64,
            page_number: page,
            page_size: limit,
        })
    }

    /// 保存用户等级
    pub async fn save(
        db: &DatabaseConnection,
        request: &UserLevelSaveRequest,
    ) -> Result<bool> {
        let now = chrono::Local::now().naive_local();

        let model = system_user_level::ActiveModel {
            name: Set(request.name.clone()),
            grade: Set(request.grade),
            discount: Set(request.discount.unwrap_or(100)),
            experience: Set(request.experience),
            icon: Set(request.icon.clone().unwrap_or_default()),
            is_show: Set(if request.is_show.unwrap_or(true) { 1 } else { 0 }),
            is_del: Set(0),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };

        model.insert(db).await?;
        Ok(true)
    }

    /// 更新用户等级
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &UserLevelSaveRequest,
    ) -> Result<bool> {
        let level = system_user_level::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("等级不存在"))?;

        let mut active: system_user_level::ActiveModel = level.into();
        active.name = Set(request.name.clone());
        active.grade = Set(request.grade);
        if let Some(discount) = request.discount {
            active.discount = Set(discount);
        }
        active.experience = Set(request.experience);
        if let Some(icon) = &request.icon {
            active.icon = Set(icon.clone());
        }
        if let Some(is_show) = request.is_show {
            active.is_show = Set(if is_show { 1 } else { 0 });
        }
        active.update_time = Set(Some(chrono::Local::now().naive_local()));

        active.update(db).await?;
        Ok(true)
    }

    /// 删除用户等级（软删除）
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool> {
        let level = system_user_level::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("等级不存在"))?;

        let mut active: system_user_level::ActiveModel = level.into();
        active.is_del = Set(1);
        active.update_time = Set(Some(chrono::Local::now().naive_local()));

        active.update(db).await?;
        Ok(true)
    }

    /// 更新等级状态（显示/隐藏）
    pub async fn update_show(db: &DatabaseConnection, id: i32, is_show: bool) -> Result<bool> {
        let level = system_user_level::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("等级不存在"))?;

        let mut active: system_user_level::ActiveModel = level.into();
        active.is_show = Set(if is_show { 1 } else { 0 });
        active.update_time = Set(Some(chrono::Local::now().naive_local()));

        active.update(db).await?;
        Ok(true)
    }

    /// 将Model转换为Response
    fn model_to_response(model: system_user_level::Model) -> UserLevelResponse {
        UserLevelResponse {
            id: model.id,
            name: model.name,
            experience: model.experience,
            is_show: model.is_show != 0,
            grade: model.grade,
            discount: model.discount,
            icon: model.icon,
            is_del: model.is_del != 0,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
