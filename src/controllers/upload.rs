/// 文件上传 -- 控制器
///
/// 实现与Java版本一致的文件上传接口
/// Java代码参考: com.zbkj.admin.controller.UploadController
///
/// 2个接口：
/// 1. POST /api/admin/upload/image - 图片上传
/// 2. POST /api/admin/upload/file  - 文件上传
///
/// 注意：Java版本中这两个接口的权限注解被注释掉了（无需权限），Rust版本保持一致
use loco_rs::prelude::*;
use axum::extract::Multipart;
use serde::Deserialize;

use crate::common::response::ApiResponse;
use crate::services::upload_service::UploadService;

/// 上传查询参数（从URL query string中获取）
#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    pub model: Option<String>,
    pub pid: Option<i32>,
}

// ==================== 接口实现 ====================

/// 1. 图片上传
#[debug_handler]
async fn image(
    State(ctx): State<AppContext>,
    Query(query): Query<UploadQuery>,
    multipart: Multipart,
) -> Result<Response> {
    let result = handle_upload(&ctx, multipart, "image", query).await?;
    format::json(ApiResponse::success(result))
}

/// 2. 文件上传
#[debug_handler]
async fn file(
    State(ctx): State<AppContext>,
    Query(query): Query<UploadQuery>,
    multipart: Multipart,
) -> Result<Response> {
    let result = handle_upload(&ctx, multipart, "file", query).await?;
    format::json(ApiResponse::success(result))
}

/// 处理multipart上传的公共逻辑
///
/// model和pid优先从multipart表单中获取，如果没有则从query参数中获取
async fn handle_upload(
    ctx: &AppContext,
    mut multipart: Multipart,
    upload_type: &str,
    query: UploadQuery,
) -> Result<crate::dtos::upload::FileResultVo> {
    let mut file_name: Option<String> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;
    let mut model: Option<String> = None;
    let mut pid: Option<i32> = None;

    // 遍历multipart字段，提取文件和参数
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::string(&format!("解析multipart失败: {}", e)))?
    {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "model" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| Error::string(&format!("读取model参数失败: {}", e)))?;
                model = Some(text);
            }
            "pid" => {
                let text = field
                    .text()
                    .await
                    .map_err(|e| Error::string(&format!("读取pid参数失败: {}", e)))?;
                pid = Some(text.parse::<i32>().unwrap_or(0));
            }
            _ => {
                // 任何带有文件名的字段都当作文件处理
                if field.file_name().is_some() {
                    file_name = field.file_name().map(|s| s.to_string());
                    content_type = field.content_type().map(|s| s.to_string());
                    file_data = Some(
                        field
                            .bytes()
                            .await
                            .map_err(|e| Error::string(&format!("读取文件数据失败: {}", e)))?
                            .to_vec(),
                    );
                } else {
                    // 忽略未知非文件字段
                    let _ = field.bytes().await;
                }
            }
        }
    }

    // 参数校验：优先multipart表单中的值，其次query参数
    let file_data =
        file_data.ok_or_else(|| Error::string("上载的文件对象不存在..."))?;
    let file_name =
        file_name.ok_or_else(|| Error::string("文件名不能为空"))?;
    let content_type = content_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let model = model.or(query.model)
        .ok_or_else(|| Error::string("model参数不能为空"))?;
    let pid = pid.or(query.pid).unwrap_or(0);

    // 调用Service
    match upload_type {
        "image" => {
            UploadService::image_upload(
                &ctx.db,
                &file_name,
                &file_data,
                &content_type,
                &model,
                pid,
            )
            .await
        }
        "file" => {
            UploadService::file_upload(
                &ctx.db,
                &file_name,
                &file_data,
                &content_type,
                &model,
                pid,
            )
            .await
        }
        _ => Err(Error::string("不支持的上传类型")),
    }
}

// ==================== 路由注册 ====================

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api/admin/upload")
        .add("/image", post(image))
        .add("/file", post(file))
}
