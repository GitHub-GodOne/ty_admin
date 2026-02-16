/// 文章管理 -- 服务层
///
/// 实现与Java版本一致的文章管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.ArticleServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;

use crate::dtos::article::*;
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::models::_entities::article;
use crate::services::system_attachment_service::SystemAttachmentService;

pub struct ArticleService;

impl ArticleService {
    /// 管理端文章分页列表
    ///
    /// Java: ArticleServiceImpl.getAdminList()
    /// 搜索条件:
    /// - cid: 精确匹配分类id
    /// - keywords: 模糊匹配 title OR author OR synopsis
    /// 排序: visit DESC, id DESC
    pub async fn get_admin_list(
        db: &DatabaseConnection,
        request: &ArticleSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<ArticleVo>> {
        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        let mut query = article::Entity::find();

        // 分类id精确匹配
        if let Some(cid) = &request.cid {
            let cid = cid.trim();
            if !cid.is_empty() {
                query = query.filter(article::Column::Cid.eq(cid));
            }
        }

        // 关键字模糊搜索: title OR author OR synopsis
        if let Some(keywords) = &request.keywords {
            let kw = keywords.trim();
            if !kw.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(article::Column::Title.contains(kw))
                        .add(article::Column::Author.contains(kw))
                        .add(article::Column::Synopsis.contains(kw)),
                );
            }
        }

        // 排序: visit DESC, id DESC
        query = query
            .order_by_desc(article::Column::Visit)
            .order_by_desc(article::Column::Id);

        // 查询总数
        let total = query.clone().count(db).await? as i64;

        // 分页查询
        let records = query
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        let list: Vec<ArticleVo> = records
            .into_iter()
            .map(|a| ArticleVo {
                id: a.id,
                cid: a.cid,
                title: a.title,
                author: a.author,
                image_input: a.image_input,
                synopsis: a.synopsis,
                visit: a.visit,
                update_time: Some(a.update_time.format("%Y-%m-%d").to_string()),
            })
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }

    /// 新增文章
    ///
    /// Java: ArticleServiceImpl.create()
    /// 1. 验证参数
    /// 2. clearPrefix 处理图片和内容中的域名前缀
    /// 3. visit 初始化为 "0"
    pub async fn create(
        db: &DatabaseConnection,
        request: &ArticleRequest,
    ) -> Result<bool> {
        Self::validate_request(request)?;

        // clearPrefix 处理图片路径和内容
        let image_input = SystemAttachmentService::clear_prefix(db, &request.image_input)
            .await
            .unwrap_or_else(|_| request.image_input.clone());
        let content = SystemAttachmentService::clear_prefix(db, &request.content)
            .await
            .unwrap_or_else(|_| request.content.clone());

        let model = article::ActiveModel {
            id: NotSet,
            cid: Set(request.cid.clone()),
            title: Set(request.title.clone()),
            author: Set(Some(request.author.clone())),
            image_input: Set(image_input),
            synopsis: Set(Some(request.synopsis.clone())),
            share_title: Set(Some(request.share_title.clone())),
            share_synopsis: Set(Some(request.share_synopsis.clone())),
            visit: Set(Some("0".to_string())),
            sort: Set(0),
            url: Set(None),
            media_id: Set(None),
            status: Set(0i16),
            hide: Set(0i16),
            admin_id: Set(0),
            mer_id: Set(Some(0)),
            product_id: Set(0),
            is_hot: Set(if request.is_hot { 1i16 } else { 0i16 }),
            is_banner: Set(if request.is_banner { 1i16 } else { 0i16 }),
            content: Set(content),
            create_time: Set(chrono::Utc::now().naive_utc()),
            update_time: Set(chrono::Utc::now().naive_utc()),
        };

        article::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 删除文章
    ///
    /// Java: ArticleServiceImpl.deleteById()
    pub async fn delete_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<bool> {
        let existing = article::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("文章已删除".to_string()))?;

        let _result = article::Entity::delete_by_id(existing.id).exec(db).await?;
        Ok(true)
    }

    /// 修改文章
    ///
    /// Java: ArticleServiceImpl.updateArticle()
    /// clearPrefix 处理图片和内容
    pub async fn update_article(
        db: &DatabaseConnection,
        id: i32,
        request: &ArticleRequest,
    ) -> Result<bool> {
        Self::validate_request(request)?;

        let existing = article::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("文章不存在".to_string()))?;

        // clearPrefix 处理
        let image_input = SystemAttachmentService::clear_prefix(db, &request.image_input)
            .await
            .unwrap_or_else(|_| request.image_input.clone());
        let content = SystemAttachmentService::clear_prefix(db, &request.content)
            .await
            .unwrap_or_else(|_| request.content.clone());

        let mut model: article::ActiveModel = existing.into();
        model.cid = Set(request.cid.clone());
        model.title = Set(request.title.clone());
        model.author = Set(Some(request.author.clone()));
        model.image_input = Set(image_input);
        model.synopsis = Set(Some(request.synopsis.clone()));
        model.share_title = Set(Some(request.share_title.clone()));
        model.share_synopsis = Set(Some(request.share_synopsis.clone()));
        model.is_hot = Set(if request.is_hot { 1i16 } else { 0i16 });
        model.is_banner = Set(if request.is_banner { 1i16 } else { 0i16 });
        model.content = Set(content);
        model.update_time = Set(chrono::Utc::now().naive_utc());

        model.update(db).await?;
        Ok(true)
    }

    /// 获取文章详情
    ///
    /// Java: ArticleServiceImpl.getDetail()
    pub async fn get_detail(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<ArticleDetailResponse> {
        let record = article::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("文章不存在".to_string()))?;

        Ok(Self::model_to_detail_response(record))
    }

    // ==================== 内部方法 ====================

    /// 验证文章请求参数
    ///
    /// Java: @Validated ArticleRequest 注解验证
    fn validate_request(request: &ArticleRequest) -> Result<()> {
        if request.cid.trim().is_empty() {
            return Err(Error::BadRequest("请选择分类".to_string()));
        }
        if request.title.trim().is_empty() {
            return Err(Error::BadRequest("请填写文章标题".to_string()));
        }
        if request.title.len() > 200 {
            return Err(Error::BadRequest("文章标题最多200个字符".to_string()));
        }
        if request.author.trim().is_empty() {
            return Err(Error::BadRequest("请填写文章作者".to_string()));
        }
        if request.author.len() > 50 {
            return Err(Error::BadRequest("文章作者最多50个字符".to_string()));
        }
        if request.image_input.trim().is_empty() {
            return Err(Error::BadRequest("请上传文章图片".to_string()));
        }
        if request.synopsis.trim().is_empty() {
            return Err(Error::BadRequest("请填写文章简介".to_string()));
        }
        if request.synopsis.len() > 200 {
            return Err(Error::BadRequest("文章简介最多200个字符".to_string()));
        }
        if request.share_title.trim().is_empty() {
            return Err(Error::BadRequest("请填写文章分享标题".to_string()));
        }
        if request.share_title.len() > 200 {
            return Err(Error::BadRequest("文章分享标题最多200个字符".to_string()));
        }
        if request.share_synopsis.trim().is_empty() {
            return Err(Error::BadRequest("请填写文章分享简介".to_string()));
        }
        if request.share_synopsis.len() > 200 {
            return Err(Error::BadRequest("文章分享简介最多200个字符".to_string()));
        }
        if request.content.trim().is_empty() {
            return Err(Error::BadRequest("请填写文章内容".to_string()));
        }
        Ok(())
    }

    /// Model 转详情 Response
    fn model_to_detail_response(model: article::Model) -> ArticleDetailResponse {
        ArticleDetailResponse {
            id: model.id,
            cid: model.cid,
            title: model.title,
            author: model.author,
            image_input: model.image_input,
            synopsis: model.synopsis,
            share_title: model.share_title,
            share_synopsis: model.share_synopsis,
            visit: model.visit,
            sort: model.sort,
            url: model.url,
            media_id: model.media_id,
            status: model.status != 0,
            hide: model.hide != 0,
            admin_id: model.admin_id,
            mer_id: model.mer_id,
            product_id: model.product_id,
            is_hot: model.is_hot != 0,
            is_banner: model.is_banner != 0,
            content: model.content,
            create_time: Some(model.create_time.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(model.update_time.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}
