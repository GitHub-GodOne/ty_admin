/// DIY页面服务
///
/// Java参考: PageDiyServiceImpl
use sea_orm::*;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::page_diy::*;
use crate::models::_entities::page_diy;

pub struct PageDiyService;

impl PageDiyService {
    /// 分页列表
    ///
    /// Java: PageDiyServiceImpl.getList()
    /// 列表查询时忽略value字段，按is_default DESC, add_time DESC排序
    /// 固定条件: mer_id=0, is_del=0
    pub async fn get_list(
        db: &DatabaseConnection,
        name: &str,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<serde_json::Value>, DbErr> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        let mut query = page_diy::Entity::find();

        // 固定条件
        query = query
            .filter(page_diy::Column::MerId.eq(0))
            .filter(page_diy::Column::IsDel.eq(0));

        // 名称模糊搜索
        if !name.is_empty() {
            query = query.filter(page_diy::Column::Name.contains(name));
        }

        // 排序: is_default DESC, add_time DESC
        query = query
            .order_by_desc(page_diy::Column::IsDefault)
            .order_by_desc(page_diy::Column::AddTime);

        // 分页
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let records = paginator.fetch_page((page - 1) as u64).await?;

        // 列表中排除value字段（设为null）
        let list: Vec<serde_json::Value> = records.iter().map(|r| {
            let mut val = serde_json::to_value(r).unwrap_or_default();
            if let Some(obj) = val.as_object_mut() {
                obj.insert("value".to_string(), serde_json::Value::Null);
                // 格式化时间
                obj.insert("addTime".to_string(),
                    serde_json::Value::String(r.add_time.format("%Y-%m-%d %H:%M:%S").to_string()));
                obj.insert("updateTime".to_string(),
                    serde_json::Value::String(r.update_time.format("%Y-%m-%d %H:%M:%S").to_string()));
            }
            val
        }).collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 新增DIY页面
    ///
    /// Java: PageDiyServiceImpl.savePageDiy()
    /// 检查名称唯一 → 保存
    pub async fn save_page_diy(
        db: &DatabaseConnection,
        request: &PageDiyRequest,
    ) -> Result<page_diy::Model, DbErr> {
        // 检查名称唯一
        if let Some(ref name) = request.name {
            Self::check_name_unique(db, name, None).await?;
        }

        let now = chrono::Local::now().naive_local();
        // value: JSONObject → String
        let value_str = request.value.as_ref()
            .map(|v| serde_json::to_string(v).unwrap_or_default());

        let active = page_diy::ActiveModel {
            version: Set(request.version.clone().unwrap_or_default()),
            name: Set(request.name.clone().unwrap_or_default()),
            title: Set(request.title.clone().unwrap_or_default()),
            cover_image: Set(request.cover_image.clone().unwrap_or_default()),
            template_name: Set(request.template_name.clone().unwrap_or_default()),
            value: Set(value_str),
            add_time: Set(now),
            update_time: Set(now),
            status: Set(request.status.unwrap_or(0)),
            r#type: Set(request.r#type.unwrap_or(0)),
            is_show: Set(request.is_show.unwrap_or(0)),
            is_bg_color: Set(request.is_bg_color.unwrap_or(0)),
            is_bg_pic: Set(request.is_bg_pic.unwrap_or(0)),
            is_diy: Set(request.is_diy.unwrap_or(0)),
            color_picker: Set(request.color_picker.clone().unwrap_or_default()),
            bg_pic: Set(request.bg_pic.clone().unwrap_or_default()),
            bg_tab_val: Set(request.bg_tab_val.unwrap_or(0)),
            is_del: Set(0),
            return_address: Set(request.return_address.clone().unwrap_or_default()),
            title_bg_color: Set(request.title_bg_color.clone().unwrap_or_default()),
            title_color: Set(request.title_color.clone().unwrap_or_default()),
            service_status: Set(request.service_status.unwrap_or(0)),
            mer_id: Set(request.mer_id.unwrap_or(0)),
            is_default: Set(request.is_default.unwrap_or(0)),
            text_position: Set(request.text_position.unwrap_or(0)),
            ..Default::default()
        };
        let result = active.insert(db).await?;
        Ok(result)
    }

    /// 修改DIY页面
    ///
    /// Java: PageDiyServiceImpl.editPageDiy()
    pub async fn edit_page_diy(
        db: &DatabaseConnection,
        request: &PageDiyRequest,
    ) -> Result<bool, DbErr> {
        let id = request.id.ok_or(DbErr::Custom("id不能为空".to_string()))?;

        // 检查名称唯一
        if let Some(ref name) = request.name {
            Self::check_name_unique(db, name, Some(id)).await?;
        }

        let record = page_diy::Entity::find_by_id(id).one(db).await?;
        let record = match record {
            Some(r) => r,
            None => return Err(DbErr::Custom("DIY模版不存在".to_string())),
        };

        let mut active: page_diy::ActiveModel = record.into();
        if let Some(ref v) = request.version { active.version = Set(v.clone()); }
        if let Some(ref v) = request.name { active.name = Set(v.clone()); }
        if let Some(ref v) = request.title { active.title = Set(v.clone()); }
        if let Some(ref v) = request.cover_image { active.cover_image = Set(v.clone()); }
        if let Some(ref v) = request.template_name { active.template_name = Set(v.clone()); }
        if let Some(ref v) = request.value {
            active.value = Set(Some(serde_json::to_string(v).unwrap_or_default()));
        }
        if let Some(v) = request.status { active.status = Set(v); }
        if let Some(v) = request.r#type { active.r#type = Set(v); }
        if let Some(v) = request.is_show { active.is_show = Set(v); }
        if let Some(v) = request.is_bg_color { active.is_bg_color = Set(v); }
        if let Some(v) = request.is_bg_pic { active.is_bg_pic = Set(v); }
        if let Some(v) = request.is_diy { active.is_diy = Set(v); }
        if let Some(ref v) = request.color_picker { active.color_picker = Set(v.clone()); }
        if let Some(ref v) = request.bg_pic { active.bg_pic = Set(v.clone()); }
        if let Some(v) = request.bg_tab_val { active.bg_tab_val = Set(v); }
        if let Some(ref v) = request.return_address { active.return_address = Set(v.clone()); }
        if let Some(ref v) = request.title_bg_color { active.title_bg_color = Set(v.clone()); }
        if let Some(ref v) = request.title_color { active.title_color = Set(v.clone()); }
        if let Some(v) = request.service_status { active.service_status = Set(v); }
        if let Some(v) = request.mer_id { active.mer_id = Set(v); }
        if let Some(v) = request.is_default { active.is_default = Set(v); }
        if let Some(v) = request.text_position { active.text_position = Set(v); }
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;
        Ok(true)
    }

    /// 修改DIY模版名称
    ///
    /// Java: PageDiyServiceImpl.editPageDiyName()
    pub async fn edit_page_diy_name(
        db: &DatabaseConnection,
        request: &PageDiyEditNameRequest,
    ) -> Result<bool, DbErr> {
        Self::check_name_unique(db, &request.name, Some(request.id)).await?;

        let record = page_diy::Entity::find_by_id(request.id).one(db).await?;
        let record = match record {
            Some(r) => r,
            None => return Err(DbErr::Custom("DIY模版不存在".to_string())),
        };
        let mut active: page_diy::ActiveModel = record.into();
        active.name = Set(request.name.clone());
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;
        Ok(true)
    }

    /// 设置DIY首页模版
    ///
    /// Java: PageDiyServiceImpl.setDiyPageHome()
    /// 先取消所有is_default=1的，再设置指定id为is_default=1
    pub async fn set_diy_page_home(
        db: &DatabaseConnection,
        diy_id: i32,
    ) -> Result<bool, DbErr> {
        let target = page_diy::Entity::find_by_id(diy_id).one(db).await?;
        if target.is_none() {
            return Err(DbErr::Custom("当前DIY模版不存在".to_string()));
        }

        // 取消现有首页设置
        let current_defaults = page_diy::Entity::find()
            .filter(page_diy::Column::IsDefault.eq(1))
            .all(db)
            .await?;
        for item in current_defaults {
            let mut active: page_diy::ActiveModel = item.into();
            active.is_default = Set(0);
            active.update(db).await?;
        }

        // 设置新首页
        let target = target.unwrap();
        let mut active: page_diy::ActiveModel = target.into();
        active.is_default = Set(1);
        active.update_time = Set(chrono::Local::now().naive_local());
        active.update(db).await?;
        Ok(true)
    }

    /// 获取DIY首页模版ID
    ///
    /// Java: PageDiyServiceImpl.getDiyPageHome()
    pub async fn get_diy_page_home_id(
        db: &DatabaseConnection,
    ) -> Result<i32, DbErr> {
        let record = page_diy::Entity::find()
            .filter(page_diy::Column::IsDefault.eq(1))
            .one(db)
            .await?;
        match record {
            Some(r) => Ok(r.id),
            None => Err(DbErr::Custom("首页模版设置不正确！".to_string())),
        }
    }

    /// 获取DIY详情（Admin端）
    ///
    /// Java: PageDiyServiceImpl.getDiyPageByPageIdForAdmin()
    /// id=0时加载首页模版，id>0时加载对应模版
    /// value字段解析为JSON对象返回
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<PageDiyResponse, DbErr> {
        let record = if id == 0 {
            page_diy::Entity::find()
                .filter(page_diy::Column::IsDefault.eq(1))
                .one(db)
                .await?
        } else {
            page_diy::Entity::find_by_id(id).one(db).await?
        };

        let record = match record {
            Some(r) => r,
            None => return Err(DbErr::Custom("未找到对应模版信息".to_string())),
        };

        // value: String → JSONObject
        let value_json = record.value.as_ref()
            .and_then(|v| serde_json::from_str(v).ok());

        Ok(PageDiyResponse {
            id: record.id,
            version: record.version.clone(),
            name: record.name.clone(),
            title: record.title.clone(),
            cover_image: record.cover_image.clone(),
            template_name: record.template_name.clone(),
            value: value_json,
            add_time: record.add_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            update_time: record.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            status: record.status,
            r#type: record.r#type,
            is_show: record.is_show,
            is_bg_color: record.is_bg_color,
            is_bg_pic: record.is_bg_pic,
            is_diy: record.is_diy,
            color_picker: record.color_picker.clone(),
            bg_pic: record.bg_pic.clone(),
            bg_tab_val: record.bg_tab_val,
            is_del: record.is_del,
            return_address: record.return_address.clone(),
            title_bg_color: record.title_bg_color.clone(),
            title_color: record.title_color.clone(),
            service_status: record.service_status,
            mer_id: record.mer_id,
            is_default: record.is_default,
            text_position: record.text_position,
        })
    }

    /// 删除DIY页面
    ///
    /// Java: pageDiyService.removeById(id)
    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool, DbErr> {
        let result = page_diy::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 检查DIY模版名称唯一
    ///
    /// Java: PageDiyServiceImpl.checkPageDiyNameUnique()
    async fn check_name_unique(
        db: &DatabaseConnection,
        name: &str,
        exclude_id: Option<i32>,
    ) -> Result<(), DbErr> {
        let mut query = page_diy::Entity::find()
            .filter(page_diy::Column::Name.eq(name));
        if let Some(id) = exclude_id {
            query = query.filter(page_diy::Column::Id.ne(id));
        }
        let exists = query.one(db).await?;
        if exists.is_some() {
            return Err(DbErr::Custom("当前模版名称已经存在，请修改后再保存！".to_string()));
        }
        Ok(())
    }
}
