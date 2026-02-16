/// 用户等级服务
///
/// 实现与Java版本一致的用户等级管理业务逻辑
use loco_rs::prelude::*;
use sea_orm::*;

use crate::models::_entities::user_level;
use crate::dtos::user_level::UserLevelResponse;
use crate::dtos::common::{PageParamRequest, CommonPage};

/// 用户等级服务
pub struct UserLevelService;

impl UserLevelService {
    /// 获取用户等级列表（分页）
    ///
    /// Java参考: UserLevelServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<UserLevelResponse>> {
        let page = page_param.page.unwrap_or(1);
        let limit = page_param.limit.unwrap_or(10);

        // 构建查询 - 排除已删除的记录
        let query = user_level::Entity::find()
            .filter(user_level::Column::IsDel.eq(0i16))
            .order_by_desc(user_level::Column::Id);

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

    /// 将Model转换为Response
    fn model_to_response(model: user_level::Model) -> UserLevelResponse {
        UserLevelResponse {
            id: model.id,
            uid: model.uid,
            level_id: model.level_id,
            grade: model.grade,
            status: model.status as i32,
            mark: model.mark,
            remind: model.remind as i32,
            is_del: model.is_del != 0,
            discount: model.discount,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            expired_time: model.expired_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
