/// 表单模板 -- 服务层
///
/// 实现与Java版本一致的表单模板管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.SystemFormTempServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::system_form_temp::*;
use crate::models::_entities::system_form_temp;

pub struct SystemFormTempService;

impl SystemFormTempService {
    /// 分页列表
    ///
    /// Java: SystemFormTempServiceImpl.getList()
    /// 逻辑：
    /// 1. 如果有 keywords，按 id 精确匹配 OR name LIKE OR info LIKE
    /// 2. 按 id 倒序排列
    /// 3. 分页返回
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &SystemFormTempSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SystemFormTempResponse>> {
        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        let mut query = system_form_temp::Entity::find();

        // 多条件搜索：按 id 精确匹配 OR name LIKE OR info LIKE
        if let Some(keywords) = &request.keywords {
            let kw = keywords.trim();
            if !kw.is_empty() {
                let mut condition = Condition::any();

                // 尝试按 id 精确匹配
                if let Ok(id_val) = kw.parse::<i32>() {
                    condition = condition.add(system_form_temp::Column::Id.eq(id_val));
                }

                // name LIKE 和 info LIKE
                condition = condition
                    .add(system_form_temp::Column::Name.contains(kw))
                    .add(system_form_temp::Column::Info.contains(kw));

                query = query.filter(condition);
            }
        }

        // 按 id 倒序
        query = query.order_by_desc(system_form_temp::Column::Id);

        // 查询总数
        let total = query.clone().count(db).await? as i64;

        // 分页查询
        let records = query
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        let list: Vec<SystemFormTempResponse> = records
            .into_iter()
            .map(|r| Self::model_to_response(r))
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }

    /// 新增表单模板
    ///
    /// Java: SystemFormTempServiceImpl.add()
    /// 逻辑：
    /// 1. 验证 name 不为空，长度不超过500
    /// 2. 验证 info 不为空，长度不超过500
    /// 3. 验证 content 不为空，且是合法的 JSON
    /// 4. 保存到数据库
    pub async fn add(
        db: &DatabaseConnection,
        request: &SystemFormTempRequest,
    ) -> Result<bool> {
        // 验证参数
        Self::validate_request(request)?;

        // 验证 content 是合法的 JSON
        Self::validate_content_json(&request.name, &request.content)?;

        let now = Local::now().naive_local();

        let model = system_form_temp::ActiveModel {
            id: NotSet,
            name: Set(request.name.clone()),
            info: Set(request.info.clone()),
            content: Set(request.content.clone()),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
        };

        system_form_temp::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 修改表单模板
    ///
    /// Java: SystemFormTempServiceImpl.edit()
    /// 逻辑：
    /// 1. 验证 content 是合法的 JSON
    /// 2. 根据 id 更新记录
    /// 3. 设置 updateTime
    pub async fn edit(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemFormTempRequest,
    ) -> Result<bool> {
        // 验证参数
        Self::validate_request(request)?;

        // 验证 content 是合法的 JSON
        Self::validate_content_json(&request.name, &request.content)?;

        // 查找记录
        let existing = system_form_temp::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        let now = Local::now().naive_local();

        let mut model: system_form_temp::ActiveModel = existing.into();
        model.name = Set(request.name.clone());
        model.info = Set(request.info.clone());
        model.content = Set(request.content.clone());
        model.update_time = Set(Some(now));

        model.update(db).await?;
        Ok(true)
    }

    /// 查询表单模板详情
    ///
    /// Java: systemFormTempService.getById(id)
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemFormTempResponse> {
        let record = system_form_temp::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::NotFound)?;

        Ok(Self::model_to_response(record))
    }

    // ==================== 辅助方法 ====================

    /// 验证请求参数
    fn validate_request(request: &SystemFormTempRequest) -> Result<()> {
        if request.name.trim().is_empty() {
            return Err(Error::BadRequest("请填写表单名称".to_string()));
        }
        if request.name.len() > 500 {
            return Err(Error::BadRequest(
                "表单名称长度不能超过500个字符".to_string(),
            ));
        }
        if request.info.trim().is_empty() {
            return Err(Error::BadRequest("请填写表单简介".to_string()));
        }
        if request.info.len() > 500 {
            return Err(Error::BadRequest(
                "表单简介长度不能超过500个字符".to_string(),
            ));
        }
        if request.content.trim().is_empty() {
            return Err(Error::BadRequest("请填写表单内容".to_string()));
        }
        Ok(())
    }

    /// 验证 content 是合法的 JSON
    ///
    /// Java: JSONObject.parseObject(content, SystemConfigFormVo.class)
    fn validate_content_json(name: &str, content: &str) -> Result<()> {
        match serde_json::from_str::<serde_json::Value>(content) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::BadRequest(format!(
                "模板表单 【{}】 的内容不是正确的JSON格式！",
                name
            ))),
        }
    }

    /// Model 转 Response DTO
    fn model_to_response(model: system_form_temp::Model) -> SystemFormTempResponse {
        SystemFormTempResponse {
            id: model.id,
            name: model.name,
            info: model.info,
            content: model.content,
            create_time: model
                .create_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model
                .update_time
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
