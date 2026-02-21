/// 附件管理服务
///
/// 实现与Java版本一致的附件管理业务逻辑
/// Java参考: SystemAttachmentServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use sea_orm::sea_query::Expr;

use crate::common::constants;
use crate::common::error::AppResult;
use crate::models::_entities::system_attachment;
use crate::dtos::system_attachment::{
    SystemAttachmentRequest, SystemAttachmentMoveRequest, SystemAttachmentResponse,
};
use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::services::system_config_service::SystemConfigService;

/// 附件管理服务
pub struct SystemAttachmentService;

impl SystemAttachmentService {
    /// 分页列表
    ///
    /// Java参考: SystemAttachmentServiceImpl.getList()
    /// 按pid和attType过滤，按attId降序
    pub async fn get_list(
        db: &DatabaseConnection,
        pid: i32,
        att_type: &Option<String>,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<SystemAttachmentResponse>> {
        let page = page_param.get_page();
        let limit = page_param.get_limit();

        tracing::info!("SystemAttachment get_list: pid={}, att_type={:?}, page={}, limit={}", pid, att_type, page, limit);

        let mut query = system_attachment::Entity::find()
            .filter(system_attachment::Column::Pid.eq(pid));

        // 按附件类型过滤
        // Java: lambdaQueryWrapper.in(SystemAttachment::getAttType, CrmebUtil.stringToArrayStr(attType))
        if let Some(att_type_str) = att_type {
            if !att_type_str.is_empty() {
                let types: Vec<String> = att_type_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !types.is_empty() {
                    query = query.filter(
                        system_attachment::Column::AttType.is_in(types)
                    );
                }
            }
        }

        // 按attId降序
        query = query.order_by_desc(system_attachment::Column::AttId);

        // 分页查询
        let paginator = query.paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let list = paginator.fetch_page((page - 1) as u64).await?;

        let list: Vec<SystemAttachmentResponse> = list
            .into_iter()
            .map(Self::model_to_response)
            .collect();

        Ok(CommonPage::new(list, total as i64, page, limit))
    }

    /// 新增附件
    ///
    /// Java参考: SystemAttachmentServiceImpl.add()
    pub async fn add(
        db: &DatabaseConnection,
        request: &SystemAttachmentRequest,
    ) -> Result<bool> {
        let now = chrono::Local::now().naive_local();

        let model = system_attachment::ActiveModel {
            name: Set(request.name.clone().unwrap_or_default()),
            att_dir: Set(request.att_dir.clone().unwrap_or_default()),
            satt_dir: Set(request.satt_dir.clone()),
            att_size: Set(request.att_size.clone().unwrap_or_default()),
            att_type: Set(request.att_type.clone().unwrap_or_default()),
            pid: Set(0),
            image_type: Set(request.image_type.unwrap_or(1i16)),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };

        system_attachment::Entity::insert(model).exec(db).await?;
        Ok(true)
    }

    /// 编辑附件
    ///
    /// Java参考: SystemAttachmentServiceImpl.edit()
    pub async fn edit(
        db: &DatabaseConnection,
        id: i32,
        request: &SystemAttachmentRequest,
    ) -> Result<bool> {
        let existing = system_attachment::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("附件不存在"))?;

        let mut model: system_attachment::ActiveModel = existing.into();
        let now = chrono::Local::now().naive_local();

        if let Some(name) = &request.name {
            model.name = Set(name.clone());
        }
        if let Some(att_dir) = &request.att_dir {
            model.att_dir = Set(att_dir.clone());
        }
        if let Some(satt_dir) = &request.satt_dir {
            model.satt_dir = Set(Some(satt_dir.clone()));
        }
        if let Some(att_size) = &request.att_size {
            model.att_size = Set(att_size.clone());
        }
        if let Some(att_type) = &request.att_type {
            model.att_type = Set(att_type.clone());
        }
        if let Some(image_type) = request.image_type {
            model.image_type = Set(image_type);
        }
        model.update_time = Set(Some(now));
        model.update(db).await?;

        Ok(true)
    }

    /// 删除附件（批量）
    ///
    /// Java参考: SystemAttachmentServiceImpl.deleteByIds()
    pub async fn delete_by_ids(
        db: &DatabaseConnection,
        ids: Vec<i32>,
    ) -> Result<bool> {
        if ids.is_empty() {
            return Err(Error::string("请选择要删除的附件"));
        }

        system_attachment::Entity::delete_many()
            .filter(system_attachment::Column::AttId.is_in(ids))
            .exec(db)
            .await?;

        Ok(true)
    }

    /// 更改图片目录（移动附件到其他分类）
    ///
    /// Java参考: SystemAttachmentServiceImpl.updateAttrId()
    /// 批量更新附件的pid
    pub async fn update_attr_id(
        db: &DatabaseConnection,
        request: &SystemAttachmentMoveRequest,
    ) -> Result<bool> {
        if request.attr_id.is_empty() {
            return Err(Error::string("请选择附件"));
        }

        let ids: Vec<i32> = request.attr_id
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        if ids.is_empty() {
            return Err(Error::string("附件ID格式错误"));
        }

        // 批量更新pid
        system_attachment::Entity::update_many()
            .col_expr(system_attachment::Column::Pid, Expr::value(request.pid))
            .filter(system_attachment::Column::AttId.is_in(ids))
            .exec(db)
            .await?;

        Ok(true)
    }

    /// 获取附件详情
    ///
    /// Java参考: Controller中调用 getById(id)
    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<SystemAttachmentResponse> {
        let model = system_attachment::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::string("附件不存在"))?;

        Ok(Self::model_to_response(model))
    }

    /// 获取CDN URL
    ///
    /// Java参考: SystemAttachmentServiceImpl.getCdnUrl()
    /// 根据上传类型配置获取对应的CDN域名
    pub async fn get_cdn_url(db: &DatabaseConnection) -> AppResult<String> {
        let upload_type_str = SystemConfigService::get_value_by_key(db, constants::CONFIG_UPLOAD_TYPE).await?;
        let upload_type: i32 = upload_type_str.parse().unwrap_or(1);

        let upload_url_key = match upload_type {
            2 => constants::CONFIG_QN_UPLOAD_URL,
            3 => constants::CONFIG_AL_UPLOAD_URL,
            4 => constants::CONFIG_TX_UPLOAD_URL,
            5 => constants::CONFIG_JD_UPLOAD_URL,
            _ => constants::CONFIG_LOCAL_UPLOAD_URL,
        };

        SystemConfigService::get_value_by_key(db, upload_url_key).await
    }

    /// 给图片加前缀
    ///
    /// Java参考: SystemAttachmentServiceImpl.prefixImage()
    pub async fn prefix_image(db: &DatabaseConnection, path: &str) -> AppResult<String> {
        let cdn_url = Self::get_cdn_url(db).await?;
        let keyword = constants::UPLOAD_FILE_KEYWORD;
        Ok(path.replace(
            &format!("{}/", keyword),
            &format!("{}/{}/", cdn_url, keyword),
        ))
    }

    /// 给文件加前缀
    ///
    /// Java参考: SystemAttachmentServiceImpl.prefixFile()
    pub async fn prefix_file(db: &DatabaseConnection, path: &str) -> AppResult<String> {
        let cdn_url = Self::get_cdn_url(db).await?;
        Ok(path.replace(
            "crmebimage/file/",
            &format!("{}/crmebimage/file/", cdn_url),
        ))
    }

    /// 清除CDN URL前缀
    ///
    /// Java参考: SystemAttachmentServiceImpl.clearPrefix()
    pub async fn clear_prefix(db: &DatabaseConnection, path: &str) -> AppResult<String> {
        if path.is_empty() {
            return Ok(path.to_string());
        }

        let cdn_url = Self::get_cdn_url(db).await?;
        let cdn_prefix = format!("{}/", cdn_url);

        if path.contains(&cdn_prefix) {
            if path.contains("callback/alipay") {
                return Ok(path.to_string());
            }
            return Ok(path.replace(&cdn_prefix, ""));
        }

        Ok(path.to_string())
    }

    // ==================== 辅助方法 ====================

    /// Model转Response
    fn model_to_response(model: system_attachment::Model) -> SystemAttachmentResponse {
        let create_time = model.create_time
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());
        let update_time = model.update_time
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());

        // 确保路径以 / 开头，避免浏览器按相对路径解析
        let satt_dir = model.satt_dir.map(|s| {
            if !s.is_empty() && !s.starts_with('/') && !s.starts_with("http") {
                format!("/{}", s)
            } else {
                s
            }
        });
        let att_dir = if !model.att_dir.is_empty() && !model.att_dir.starts_with('/') && !model.att_dir.starts_with("http") {
            format!("/{}", model.att_dir)
        } else {
            model.att_dir
        };

        SystemAttachmentResponse {
            att_id: model.att_id,
            name: model.name,
            att_dir,
            satt_dir,
            att_size: model.att_size,
            att_type: model.att_type,
            pid: model.pid,
            image_type: model.image_type,
            create_time,
            update_time,
        }
    }
}
