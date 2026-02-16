/// 微信关键字回复 -- 服务层
///
/// 实现与Java版本一致的微信关键字回复逻辑
/// Java代码参考: com.zbkj.service.service.impl.WechatReplyServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;

use crate::dtos::wechat_reply::*;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::models::_entities::wechat_reply;
use crate::services::system_attachment_service::SystemAttachmentService;

pub struct WechatReplyService;

impl WechatReplyService {
    /// 分页列表
    ///
    /// Java: WechatReplyServiceImpl.getList()
    /// 搜索条件:
    /// - type: 精确匹配回复类型
    /// - keywords: 精确匹配关键字
    /// 排序: id DESC
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &WechatReplySearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<WechatReplyResponse>> {
        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        let mut query = wechat_reply::Entity::find();

        // 类型精确匹配
        if let Some(reply_type) = &request.reply_type {
            let t = reply_type.trim();
            if !t.is_empty() {
                query = query.filter(wechat_reply::Column::Type.eq(t));
            }
        }

        // 关键字精确匹配
        if let Some(keywords) = &request.keywords {
            let kw = keywords.trim();
            if !kw.is_empty() {
                query = query.filter(wechat_reply::Column::Keywords.eq(kw));
            }
        }

        // 排序: id DESC
        query = query.order_by_desc(wechat_reply::Column::Id);

        // 查询总数
        let total = query.clone().count(db).await? as i64;

        // 分页查询
        let records = query
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        let list: Vec<WechatReplyResponse> = records
            .into_iter()
            .map(Self::model_to_response)
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }

    /// 新增微信关键字回复
    ///
    /// Java: WechatReplyServiceImpl.create()
    /// 1. 检测关键字是否重复
    /// 2. clearPrefix 处理回复数据
    /// 3. 保存
    pub async fn create(
        db: &DatabaseConnection,
        request: &WechatReplyRequest,
    ) -> Result<bool> {
        Self::validate_request(request)?;

        // 检测关键字重复
        let existing = Self::get_by_keywords(db, &request.keywords).await?;
        if existing.is_some() {
            return Err(Error::BadRequest(
                format!("{}关键字已经存在", request.keywords),
            ));
        }

        // clearPrefix 处理回复数据
        let data = SystemAttachmentService::clear_prefix(db, &request.data)
            .await
            .unwrap_or_else(|_| request.data.clone());

        let now = chrono::Utc::now().naive_utc();
        let model = wechat_reply::ActiveModel {
            id: NotSet,
            keywords: Set(request.keywords.clone()),
            r#type: Set(request.reply_type.clone()),
            data: Set(data),
            status: Set(if request.status { 1i16 } else { 0i16 }),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
        };

        wechat_reply::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 删除微信关键字回复
    ///
    /// Java: removeById()
    pub async fn delete_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let result = wechat_reply::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    /// 修改微信关键字回复
    ///
    /// Java: WechatReplyServiceImpl.updateReply()
    /// 1. 查找记录（getInfoException）
    /// 2. 复制属性
    /// 3. 调用 updateVo（检测重复 + clearPrefix + 更新）
    pub async fn update_reply(
        db: &DatabaseConnection,
        request: &WechatReplyRequest,
    ) -> Result<bool> {
        Self::validate_request(request)?;

        let id = request.id
            .ok_or_else(|| Error::BadRequest("关键字id不能为空".to_string()))?;

        // getInfoException(id, false) - 不检查status
        let existing = wechat_reply::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("没有找到相关数据".to_string()))?;

        // 检测关键字重复（排除自身）
        let duplicate = Self::get_by_keywords(db, &request.keywords).await?;
        if let Some(dup) = duplicate {
            if dup.id != existing.id {
                return Err(Error::BadRequest(
                    format!("{}关键字已经存在", request.keywords),
                ));
            }
        }

        // clearPrefix 处理回复数据
        let data = SystemAttachmentService::clear_prefix(db, &request.data)
            .await
            .unwrap_or_else(|_| request.data.clone());

        let mut model: wechat_reply::ActiveModel = existing.into();
        model.keywords = Set(request.keywords.clone());
        model.r#type = Set(request.reply_type.clone());
        model.data = Set(data);
        model.status = Set(if request.status { 1i16 } else { 0i16 });
        model.update_time = Set(Some(chrono::Utc::now().naive_utc()));

        model.update(db).await?;
        Ok(true)
    }

    /// 修改状态
    ///
    /// Java: WechatReplyServiceImpl.updateStatus()
    /// 1. getInfoException(id, false)
    /// 2. 设置status
    /// 3. 调用 updateVo（检测重复 + clearPrefix + 更新）
    pub async fn update_status(
        db: &DatabaseConnection,
        id: i32,
        status: bool,
    ) -> Result<bool> {
        let existing = wechat_reply::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("没有找到相关数据".to_string()))?;

        // 检测关键字重复（排除自身）— Java updateVo 中的逻辑
        let duplicate = Self::get_by_keywords(db, &existing.keywords).await?;
        if let Some(dup) = duplicate {
            if dup.id != existing.id {
                return Err(Error::BadRequest(
                    format!("{}关键字已经存在", existing.keywords),
                ));
            }
        }

        // clearPrefix 处理回复数据 — Java updateVo 中的逻辑
        let data = SystemAttachmentService::clear_prefix(db, &existing.data)
            .await
            .unwrap_or_else(|_| existing.data.clone());

        let mut model: wechat_reply::ActiveModel = existing.into();
        model.status = Set(if status { 1i16 } else { 0i16 });
        model.data = Set(data);
        model.update_time = Set(Some(chrono::Utc::now().naive_utc()));

        model.update(db).await?;
        Ok(true)
    }

    /// 获取详情
    ///
    /// Java: WechatReplyServiceImpl.getInfo()
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<WechatReplyResponse> {
        let record = wechat_reply::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("没有找到相关数据".to_string()))?;

        Ok(Self::model_to_response(record))
    }

    /// 根据关键字查询数据
    ///
    /// Java: WechatReplyServiceImpl.getVoByKeywords()
    pub async fn get_by_keywords_response(
        db: &DatabaseConnection,
        keywords: &str,
    ) -> Result<WechatReplyResponse> {
        let record = Self::get_by_keywords(db, keywords)
            .await?
            .ok_or_else(|| Error::BadRequest("没有找到相关数据".to_string()))?;

        Ok(Self::model_to_response(record))
    }

    // ==================== 内部方法 ====================

    /// 根据关键字查询记录
    ///
    /// Java: WechatReplyServiceImpl.getVoByKeywords()
    async fn get_by_keywords(
        db: &DatabaseConnection,
        keywords: &str,
    ) -> Result<Option<wechat_reply::Model>> {
        let record = wechat_reply::Entity::find()
            .filter(wechat_reply::Column::Keywords.eq(keywords))
            .one(db)
            .await?;
        Ok(record)
    }

    /// 验证请求参数
    ///
    /// Java: @Validated WechatReplyRequest 注解验证
    fn validate_request(request: &WechatReplyRequest) -> Result<()> {
        if request.keywords.trim().is_empty() {
            return Err(Error::BadRequest(
                "请填写关键字, 关注 = subscribe， 默认 = default".to_string(),
            ));
        }
        if request.reply_type.trim().is_empty() {
            return Err(Error::BadRequest("请选择回复类型".to_string()));
        }
        if request.data.trim().is_empty() {
            return Err(Error::BadRequest("请填写回复数据".to_string()));
        }
        Ok(())
    }

    /// Model 转 Response
    fn model_to_response(model: wechat_reply::Model) -> WechatReplyResponse {
        WechatReplyResponse {
            id: model.id,
            keywords: model.keywords,
            reply_type: model.r#type,
            data: model.data,
            status: model.status != 0,
            create_time: model.create_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: model.update_time.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
