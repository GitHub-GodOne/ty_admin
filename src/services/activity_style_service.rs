/// 活动样式服务
///
/// Java参考: ActivityStyleServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::models::_entities::activity_style;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::activity_style::*;
use crate::services::system_attachment_service::SystemAttachmentService;

pub struct ActivityStyleService;

impl ActivityStyleService {
    /// 分页列表
    ///
    /// Java参考: ActivityStyleServiceImpl.getList()
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &ActivityStyleSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<ActivityStyleResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = activity_style::Entity::find();

        // 类型 (必填)
        if let Some(style_type) = request.style_type {
            query = query.filter(activity_style::Column::Type.eq(style_type));
        }

        // 名称模糊搜索
        if let Some(name) = &request.name {
            if !name.is_empty() {
                query = query.filter(activity_style::Column::Name.contains(name));
            }
        }

        // 状态
        if let Some(status) = request.status {
            query = query.filter(activity_style::Column::Status.eq(status));
        }

        // 方式
        if let Some(method) = request.method {
            query = query.filter(activity_style::Column::Method.eq(method));
        }

        // 运行状态筛选
        let now = Local::now().naive_local();
        if let Some(running_status) = request.running_status {
            use sea_orm::sea_query::Expr;
            let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
            match running_status {
                -1 => {
                    // 已结束: endtime < now
                    query = query.filter(Expr::cust(&format!(
                        "endtime < '{}'::timestamp", now_str
                    )));
                }
                0 => {
                    // 未开始: starttime > now
                    query = query.filter(Expr::cust(&format!(
                        "starttime > '{}'::timestamp", now_str
                    )));
                }
                1 => {
                    // 进行中: starttime <= now AND endtime >= now
                    query = query.filter(Expr::cust(&format!(
                        "starttime <= '{0}'::timestamp AND endtime >= '{0}'::timestamp", now_str
                    )));
                }
                _ => {}
            }
        }

        // 排序: createtime desc
        query = query.order_by_desc(activity_style::Column::Createtime);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<ActivityStyleResponse> = records.iter()
            .map(|r| Self::model_to_response(r, &now))
            .collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }
// __PLACEHOLDER2__

    /// 新增活动样式
    ///
    /// Java参考: ActivityStyleController.save()
    pub async fn save(
        db: &DatabaseConnection,
        request: &ActivityStyleRequest,
    ) -> Result<bool> {
        let now = Local::now().naive_local();

        let starttime = chrono::NaiveDateTime::parse_from_str(&request.starttime, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| Error::string("开始时间格式错误"))?;
        let endtime = chrono::NaiveDateTime::parse_from_str(&request.endtime, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| Error::string("结束时间格式错误"))?;

        // clearPrefix 处理素材路径
        let style = SystemAttachmentService::clear_prefix(db, &request.style)
            .await
            .unwrap_or_else(|_| request.style.clone());

        let record = activity_style::ActiveModel {
            name: Set(request.name.clone()),
            r#type: Set(request.style_type),
            starttime: Set(starttime),
            endtime: Set(endtime),
            style: Set(style),
            status: Set(request.status),
            method: Set(request.method),
            products: Set(request.products.clone()),
            createtime: Set(now),
            updatetime: Set(now),
            ..Default::default()
        };

        record.insert(db).await?;
        Ok(true)
    }

    /// 修改活动样式
    ///
    /// Java参考: ActivityStyleController.update()
    pub async fn update(
        db: &DatabaseConnection,
        request: &ActivityStyleRequest,
    ) -> Result<bool> {
        let id = request.id.ok_or_else(|| Error::string("ID不能为空"))?;

        let existing = activity_style::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("活动样式不存在"))?;
// __PLACEHOLDER3__

        let now = Local::now().naive_local();

        let starttime = chrono::NaiveDateTime::parse_from_str(&request.starttime, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| Error::string("开始时间格式错误"))?;
        let endtime = chrono::NaiveDateTime::parse_from_str(&request.endtime, "%Y-%m-%d %H:%M:%S")
            .map_err(|_| Error::string("结束时间格式错误"))?;

        // clearPrefix 处理素材路径
        let style = SystemAttachmentService::clear_prefix(db, &request.style)
            .await
            .unwrap_or_else(|_| request.style.clone());

        let mut active: activity_style::ActiveModel = existing.into();
        active.name = Set(request.name.clone());
        active.r#type = Set(request.style_type);
        active.starttime = Set(starttime);
        active.endtime = Set(endtime);
        active.style = Set(style);
        active.status = Set(request.status);
        active.method = Set(request.method);
        active.products = Set(request.products.clone());
        active.updatetime = Set(now);
        active.update(db).await?;

        Ok(true)
    }

    /// 删除活动样式
    ///
    /// Java参考: ActivityStyleController.delete() → removeById
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let result = activity_style::Entity::delete_by_id(id)
            .exec(db)
            .await?;
        Ok(result.rows_affected > 0)
    }

    /// 更新状态
    ///
    /// Java参考: ActivityStyleServiceImpl.updateStatus()
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let existing = activity_style::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("活动样式不存在"))?;

        let now = Local::now().naive_local();
        let mut active: activity_style::ActiveModel = existing.into();
        active.status = Set(status);
        active.updatetime = Set(now);
        active.update(db).await?;

        Ok(true)
    }

    // ==================== 私有辅助方法 ====================

    /// 计算运行状态: -1=已结束, 0=未开始, 1=进行中
    fn calc_running_status(
        starttime: &chrono::NaiveDateTime,
        endtime: &chrono::NaiveDateTime,
        now: &chrono::NaiveDateTime,
    ) -> i32 {
        if endtime < now {
            -1 // 已结束
        } else if starttime > now {
            0 // 未开始
        } else {
            1 // 进行中
        }
    }

    fn model_to_response(
        r: &activity_style::Model,
        now: &chrono::NaiveDateTime,
    ) -> ActivityStyleResponse {
        ActivityStyleResponse {
            id: r.id,
            name: r.name.clone(),
            style_type: r.r#type,
            starttime: r.starttime.format("%Y-%m-%d %H:%M:%S").to_string(),
            endtime: r.endtime.format("%Y-%m-%d %H:%M:%S").to_string(),
            style: r.style.clone(),
            running_status: Self::calc_running_status(&r.starttime, &r.endtime, now),
            status: r.status,
            method: r.method,
            products: r.products.clone(),
            createtime: r.createtime.format("%Y-%m-%d %H:%M:%S").to_string(),
            updatetime: r.updatetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
