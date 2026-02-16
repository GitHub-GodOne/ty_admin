/// 版权服务层
///
/// 处理版权相关的业务逻辑
/// Java代码参考: com.zbkj.admin.service.impl.CopyrightServiceImpl
use sea_orm::*;
use serde::Deserialize;

use crate::common::constants;
use crate::common::error::{AppError, AppResult};
use crate::dtos::copyright::*;
use crate::models::_entities::system_config;

/// CRMEB版权接口响应
#[derive(Debug, Deserialize)]
struct CopyrightApiResponse {
    status: Option<i32>,
    data: Option<CopyrightApiData>,
}

/// CRMEB版权接口响应data字段
#[derive(Debug, Deserialize)]
struct CopyrightApiData {
    status: Option<i32>,
    copyright: Option<String>,
    auth_code: Option<String>,
}

pub struct CopyrightService;

impl CopyrightService {
    /// 获取版权信息
    ///
    /// Java: CopyrightServiceImpl.getInfo()
    /// 流程：
    /// 1. 从system_config获取api_url，如果为空返回status=-2
    /// 2. 从system_config获取copyright_label
    /// 3. 获取项目版本号
    /// 4. 调用CRMEB版权查询接口
    /// 5. 解析响应，填充版权信息
    /// 6. 如果授权成功(status=1)，获取公司信息和图片
    pub async fn get_info(
        db: &DatabaseConnection,
    ) -> AppResult<CopyrightInfoResponse> {
        let mut response = CopyrightInfoResponse {
            domain_url: None,
            version: None,
            label: None,
            auth_code: None,
            status: None,
            company_name: None,
            company_image: None,
            copyright: None,
        };

        // 1. 获取API域名
        let domain_name = Self::get_config_value(db, constants::CONFIG_KEY_API_URL).await?;
        if domain_name.is_none() || domain_name.as_ref().map_or(true, |s| s.is_empty()) {
            response.status = Some(-2);
            return Ok(response);
        }
        let domain_name = domain_name.unwrap();

        // 2. 获取版权标签
        let label_str = Self::get_config_value(db, constants::CONFIG_COPYRIGHT_LABEL).await?;
        let label = label_str
            .as_ref()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        // 3. 获取版本号
        let version = constants::PROJECT_VERSION.to_string();
        if version.is_empty() {
            return Err(AppError::BusinessError(
                "请先在配置中设置版本号".to_string(),
            ));
        }

        response.domain_url = Some(domain_name.clone());
        response.label = Some(label);
        response.version = Some(version.clone());

        // 4. 调用CRMEB版权查询接口
        let url = format!(
            "{}?domain_name={}&label={}&version={}",
            constants::CRMEB_COPYRIGHT_URL, domain_name, label, version
        );

        let client = reqwest::Client::new();
        let api_response = client
            .post(&url)
            .send()
            .await
            .map_err(|e| {
                AppError::InternalError(format!("CRMEB版权接口调用失败: {}", e))
            })?;

        let api_result: CopyrightApiResponse = api_response
            .json()
            .await
            .map_err(|e| {
                AppError::InternalError(format!("CRMEB版权接口响应解析失败: {}", e))
            })?;

        // 5. 检查接口返回状态
        if api_result.status != Some(200) {
            return Err(AppError::BusinessError(format!(
                "CRMEB版权接口调用失败, status: {:?}",
                api_result.status
            )));
        }

        // 6. 解析data
        let data = api_result.data.ok_or_else(|| {
            AppError::InternalError("CRMEB版权接口返回数据为空".to_string())
        })?;

        response.status = data.status;
        response.copyright = data.copyright;

        // 7. 如果授权成功(status=1)，获取公司信息
        if data.status == Some(1) {
            response.auth_code = data.auth_code;
            response.company_name =
                Self::get_config_value(db, constants::CONFIG_COPYRIGHT_COMPANY_INFO).await?;
            response.company_image =
                Self::get_config_value(db, constants::CONFIG_COPYRIGHT_COMPANY_IMAGE).await?;
        }

        Ok(response)
    }

    /// 根据配置名称获取配置值
    ///
    /// Java: systemConfigService.getValueByKey(key)
    async fn get_config_value(
        db: &DatabaseConnection,
        name: &str,
    ) -> AppResult<Option<String>> {
        let config = system_config::Entity::find()
            .filter(system_config::Column::Name.eq(name))
            .one(db)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(config.and_then(|c| c.value))
    }
}
