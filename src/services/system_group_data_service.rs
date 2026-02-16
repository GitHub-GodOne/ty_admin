/// 组合数据详情服务
///
/// Java参考: SystemGroupDataServiceImpl
use sea_orm::*;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::system_group_data::*;
use crate::models::_entities::system_group_data;
use crate::services::system_attachment_service::SystemAttachmentService;

pub struct SystemGroupDataService;

impl SystemGroupDataService {
    /// 分页列表
    ///
    /// Java: SystemGroupDataServiceImpl.getList()
    /// 条件: gid, status; 排序: sort ASC, id ASC
    pub async fn get_list(
        db: &DatabaseConnection,
        search: &SystemGroupDataSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<serde_json::Value>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = system_group_data::Entity::find();

        // gid 过滤
        if let Some(gid) = search.gid {
            query = query.filter(system_group_data::Column::Gid.eq(gid));
        }

        // status 过滤
        if let Some(status) = search.status {
            let status_val: i16 = if status { 1 } else { 0 };
            query = query.filter(system_group_data::Column::Status.eq(status_val));
        }

        // 排序: sort ASC, id ASC
        query = query
            .order_by_asc(system_group_data::Column::Sort)
            .order_by_asc(system_group_data::Column::Id);

        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<serde_json::Value> = records.iter().map(|r| {
            let mut val = serde_json::to_value(r).unwrap_or_default();
            if let Some(obj) = val.as_object_mut() {
                // 格式化时间
                if let Some(ct) = r.create_time {
                    obj.insert("createTime".to_string(),
                        serde_json::Value::String(ct.format("%Y-%m-%d %H:%M:%S").to_string()));
                }
                if let Some(ut) = r.update_time {
                    obj.insert("updateTime".to_string(),
                        serde_json::Value::String(ut.format("%Y-%m-%d %H:%M:%S").to_string()));
                }
            }
            val
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 新增组合数据
    ///
    /// Java: SystemGroupDataServiceImpl.create()
    /// 将form序列化为JSON字符串存入value，clearPrefix处理上传路径
    pub async fn create(
        db: &DatabaseConnection,
        request: &SystemGroupDataRequest,
    ) -> Result<bool, DbErr> {
        let now = chrono::Local::now().naive_local();

        // 序列化form为JSON字符串
        let value_str = serde_json::to_string(&request.form).unwrap_or_default();
        // clearPrefix处理上传路径
        let value_str = SystemAttachmentService::clear_prefix(db, &value_str)
            .await
            .unwrap_or(value_str);

        let sort = request.form.sort.unwrap_or(0);
        let status: i16 = if request.form.status.unwrap_or(false) { 1 } else { 0 };

        let active = system_group_data::ActiveModel {
            gid: Set(request.gid.unwrap_or(0)),
            value: Set(value_str),
            sort: Set(sort),
            status: Set(status),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };
        active.insert(db).await?;
        Ok(true)
    }
    /// 修改组合数据
    ///
    /// Java: SystemGroupDataServiceImpl.update()
    pub async fn update(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemGroupDataRequest,
    ) -> Result<bool, DbErr> {
        let record = system_group_data::Entity::find_by_id(id).one(db).await?;
        let record = match record {
            Some(r) => r,
            None => return Err(DbErr::Custom("组合数据不存在".to_string())),
        };

        // 序列化form为JSON字符串
        let value_str = serde_json::to_string(&request.form).unwrap_or_default();
        let value_str = SystemAttachmentService::clear_prefix(db, &value_str)
            .await
            .unwrap_or(value_str);

        let sort = request.form.sort.unwrap_or(0);
        let status: i16 = if request.form.status.unwrap_or(false) { 1 } else { 0 };

        let mut active: system_group_data::ActiveModel = record.into();
        active.gid = Set(request.gid.unwrap_or(0));
        active.value = Set(value_str);
        active.sort = Set(sort);
        active.status = Set(status);
        active.update_time = Set(Some(chrono::Local::now().naive_local()));
        active.update(db).await?;
        Ok(true)
    }

    /// 删除组合数据
    ///
    /// Java: systemGroupDataService.removeById(id)
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool, DbErr> {
        let result = system_group_data::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 获取组合数据详情
    ///
    /// Java: systemGroupDataService.getById(id)
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<serde_json::Value, DbErr> {
        let record = system_group_data::Entity::find_by_id(id).one(db).await?;
        let record = match record {
            Some(r) => r,
            None => return Err(DbErr::Custom("组合数据不存在".to_string())),
        };

        let mut val = serde_json::to_value(&record).unwrap_or_default();
        if let Some(obj) = val.as_object_mut() {
            if let Some(ct) = record.create_time {
                obj.insert("createTime".to_string(),
                    serde_json::Value::String(ct.format("%Y-%m-%d %H:%M:%S").to_string()));
            }
            if let Some(ut) = record.update_time {
                obj.insert("updateTime".to_string(),
                    serde_json::Value::String(ut.format("%Y-%m-%d %H:%M:%S").to_string()));
            }
        }
        Ok(val)
    }
}
