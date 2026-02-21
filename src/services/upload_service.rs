/// 文件上传服务
///
/// 实现与Java版本一致的文件上传业务逻辑
/// Java参考: UploadServiceImpl
use loco_rs::prelude::*;
use sea_orm::Set;
use std::path::Path;

use crate::dtos::upload::FileResultVo;
use crate::models::_entities::system_attachment;
use crate::services::system_config_service::SystemConfigService;

/// 上传文件关键字
const UPLOAD_FILE_KEYWORD: &str = "crmebimage";

/// 上传配置Key
const UPLOAD_IMAGE_EXT_STR_KEY: &str = "image_ext_str";
const UPLOAD_IMAGE_MAX_SIZE_KEY: &str = "image_max_size";
const UPLOAD_FILE_EXT_STR_KEY: &str = "file_ext_str";
const UPLOAD_FILE_MAX_SIZE_KEY: &str = "file_max_size";
const CONFIG_UPLOAD_TYPE: &str = "uploadType";

/// 默认上传根路径
const DEFAULT_IMAGE_PATH: &str = "./uploads";

pub struct UploadService;

impl UploadService {
    /// 图片上传
    ///
    /// Java参考: UploadServiceImpl.imageUpload()
    pub async fn image_upload(
        db: &DatabaseConnection,
        file_name: &str,
        file_data: &[u8],
        content_type: &str,
        model: &str,
        pid: i32,
    ) -> Result<FileResultVo> {
        Self::common_upload(db, file_name, file_data, content_type, model, pid, UPLOAD_FILE_KEYWORD).await
    }

    /// 文件上传
    ///
    /// Java参考: UploadServiceImpl.fileUpload()
    pub async fn file_upload(
        db: &DatabaseConnection,
        file_name: &str,
        file_data: &[u8],
        content_type: &str,
        model: &str,
        pid: i32,
    ) -> Result<FileResultVo> {
        Self::common_upload(db, file_name, file_data, content_type, model, pid, UPLOAD_FILE_KEYWORD).await
    }

    /// 上传校验
    ///
    /// Java参考: UploadServiceImpl.uploadValidate()
    /// 校验文件后缀名和大小
    async fn upload_validate(
        db: &DatabaseConnection,
        file_name: &str,
        file_size_mb: f64,
        file_type: &str,
        content_type: &str,
    ) -> Result<String> {
        // 获取文件后缀名
        let ext_name = Path::new(file_name)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_else(|| {
                // 如果文件名没有后缀，从content_type中提取
                if !content_type.is_empty() {
                    content_type.split('/').last().unwrap_or("").to_string()
                } else {
                    String::new()
                }
            });

        if ext_name.is_empty() {
            return Err(Error::string("文件类型未定义，无法上传..."));
        }

        // 判断是否为文件上传（非图片）
        let is_file_upload = file_type != UPLOAD_FILE_KEYWORD;

        // 获取允许的文件扩展名
        let ext_str_key = if is_file_upload {
            UPLOAD_FILE_EXT_STR_KEY
        } else {
            UPLOAD_IMAGE_EXT_STR_KEY
        };
        let ext_str = SystemConfigService::get_value_by_key(db, ext_str_key)
            .await
            .unwrap_or_default();

        // 校验文件扩展名
        if !ext_str.is_empty() {
            let extensions: Vec<&str> = ext_str.split(',').map(|s| s.trim()).collect();
            if extensions.is_empty() || !extensions.contains(&ext_name.as_str()) {
                return Err(Error::string(&format!(
                    "上载文件类型只能为：{}",
                    ext_str
                )));
            }
        }

        // 获取最大文件大小（MB）
        let max_size_key = if is_file_upload {
            UPLOAD_FILE_MAX_SIZE_KEY
        } else {
            UPLOAD_IMAGE_MAX_SIZE_KEY
        };
        let max_size_str = SystemConfigService::get_value_by_key(db, max_size_key)
            .await
            .unwrap_or_default();
        let max_size: f64 = max_size_str.parse().unwrap_or(10.0);

        // 校验文件大小
        if file_size_mb > max_size {
            return Err(Error::string(&format!(
                "最大允许上传 {} MB文件，当前文件大小为 {:.2} MB",
                max_size, file_size_mb
            )));
        }

        Ok(ext_name)
    }

    /// 公共上传
    ///
    /// Java参考: UploadServiceImpl.commonUpload()
    /// 1. 校验文件
    /// 2. 生成文件名和路径
    /// 3. 保存到本地
    /// 4. 记录到system_attachment表
    /// 5. 返回FileResultVo
    async fn common_upload(
        db: &DatabaseConnection,
        file_name: &str,
        file_data: &[u8],
        content_type: &str,
        model: &str,
        pid: i32,
        file_type: &str,
    ) -> Result<FileResultVo> {
        if file_data.is_empty() {
            return Err(Error::string("上载的文件对象不存在..."));
        }

        // 校验
        let file_size_mb = file_data.len() as f64 / 1024.0 / 1024.0;
        let ext_name = Self::upload_validate(db, file_name, file_size_mb, file_type, content_type).await?;

        // 文件名截断（与Java一致：超过99字符截断）
        let mut final_file_name = file_name.to_string();
        if final_file_name.len() > 99 {
            final_file_name = format!("{}.{}", &final_file_name[..90], ext_name);
        }

        // 服务器存储根路径
        let root_path = DEFAULT_IMAGE_PATH.to_string();

        // 模块路径
        let model_path = format!("public/{}/", model);

        // 类型路径
        let type_path = format!("{}/", UPLOAD_FILE_KEYWORD);

        // 生成新文件名: UUID.ext
        let new_file_name = format!(
            "{}.{}",
            uuid::Uuid::new_v4().to_string().replace("-", ""),
            ext_name
        );

        // 日期路径: yyyy/MM/dd/
        let now = chrono::Local::now();
        let date_path = now.format("%Y/%m/%d/").to_string();

        // 完整web路径: type/public/model/yyyy/MM/dd/
        let web_path = format!("{}{}{}", type_path, model_path, date_path);

        // 完整磁盘路径
        let dest_dir = format!("{}/{}", root_path, web_path);
        let dest_path = format!("{}{}", dest_dir, new_file_name);

        // 创建目录
        tokio::fs::create_dir_all(&dest_dir)
            .await
            .map_err(|e| Error::string(&format!("创建目录失败: {}", e)))?;

        // 写入文件
        tokio::fs::write(&dest_path, file_data)
            .await
            .map_err(|e| Error::string(&format!("保存文件失败: {}", e)))?;

        // 构建返回结果（带前导 / 确保浏览器按绝对路径解析）
        let url = format!("/{}{}", web_path, new_file_name);

        // 处理type字段（与Java一致）
        let result_type = if file_type == UPLOAD_FILE_KEYWORD {
            content_type.replace("image/", "")
        } else {
            content_type.replace("file/", "")
        };

        let result = FileResultVo {
            file_name: final_file_name.clone(),
            ext_name: ext_name.clone(),
            file_size: file_data.len() as u64,
            url: url.clone(),
            file_type: result_type.clone(),
        };

        // 获取上传类型配置（默认本地=1）
        let upload_type_str = SystemConfigService::get_value_by_key(db, CONFIG_UPLOAD_TYPE)
            .await
            .unwrap_or_else(|_| "1".to_string());
        let upload_type: i16 = upload_type_str.parse().unwrap_or(1);

        // 保存附件记录到数据库
        let now_dt = chrono::Local::now().naive_local();
        let attachment = system_attachment::ActiveModel {
            name: Set(final_file_name),
            att_dir: Set(String::new()),
            satt_dir: Set(Some(url)),
            att_size: Set(file_data.len().to_string()),
            att_type: Set(result_type),
            pid: Set(pid),
            image_type: Set(upload_type),
            create_time: Set(Some(now_dt)),
            update_time: Set(Some(now_dt)),
            ..Default::default()
        };
        system_attachment::Entity::insert(attachment)
            .exec(db)
            .await?;

        // 云存储上传（type 2=七牛, 3=OSS, 4=COS, 5=京东云）
        // 目前仅实现本地存储，云存储需要额外集成SDK
        if upload_type > 1 {
            tracing::warn!(
                "当前上传类型为 {}，云存储上传暂未实现，文件已保存到本地",
                upload_type
            );
        }

        Ok(result)
    }
}
