/// 微信小程序服务 -- 服务层
///
/// Java参考: WechatNewServiceImpl, QrCodeServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;
use chrono::Local;

use crate::common::constants;
use crate::dtos::wechat_mini::*;
use crate::initializers::redis::get_redis;
use crate::models::_entities::wechat_exceptions;
use crate::services::system_config_service::SystemConfigService;

pub struct WechatMiniService;

impl WechatMiniService {
    /// 获取微信小程序码
    ///
    /// Java: QrCodeServiceImpl.getWecahtQrCode() + WechatNewServiceImpl.createQrCode()
    pub async fn get_wechat_qr_code(
        db: &DatabaseConnection,
        data: &serde_json::Value,
    ) -> Result<QrCodeVo> {
        if data.is_null() || (data.is_object() && data.as_object().unwrap().is_empty()) {
            return Err(Error::BadRequest("生成微信参数不能为空".to_string()));
        }
        let code = Self::create_qr_code(db, data).await?;
        Ok(QrCodeVo { code })
    }

    /// 生成小程序码
    ///
    /// Java: WechatNewServiceImpl.createQrCode()
    /// 1. 获取小程序access_token
    /// 2. 调用微信API生成小程序码
    /// 3. 处理token过期重试
    /// 4. 返回base64编码的图片
    async fn create_qr_code(
        db: &DatabaseConnection,
        data: &serde_json::Value,
    ) -> Result<String> {
        let mini_access_token = Self::get_mini_access_token(db).await?;
        let url = constants::WECHAT_MINI_QRCODE_UNLIMITED_URL
            .replace("{}", &mini_access_token);

        tracing::info!("微信小程序码生成参数:{}", data);

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(data)
            .send()
            .await
            .map_err(|e| Error::string(&format!("微信API请求失败: {}", e)))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| Error::string(&format!("微信API响应读取失败: {}", e)))?;

        let response_str = String::from_utf8_lossy(&bytes);

        // 检查是否返回了错误JSON（成功时返回二进制图片数据）
        if response_str.contains("errcode") {
            tracing::error!("微信生成小程序码异常: {}", response_str);

            if let Ok(err_data) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                Self::wx_exception_dispose(db, &err_data, "微信小程序生成小程序码异常").await;

                // errcode 40001: token过期，清除缓存重试
                let errcode = err_data.get("errcode")
                    .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
                    .unwrap_or(0);

                if errcode == 40001 {
                    // 清除Redis缓存的token
                    if let Ok(redis) = get_redis().await {
                        let _ = redis.del(constants::REDIS_WECHAT_MINI_ACCESS_TOKEN_KEY).await;
                    }

                    // 重新获取token并重试
                    let new_token = Self::get_mini_access_token(db).await?;
                    let retry_url = constants::WECHAT_MINI_QRCODE_UNLIMITED_URL
                        .replace("{}", &new_token);

                    let retry_response = client
                        .post(&retry_url)
                        .json(data)
                        .send()
                        .await
                        .map_err(|e| Error::string(&format!("微信API重试请求失败: {}", e)))?;

                    let retry_bytes = retry_response
                        .bytes()
                        .await
                        .map_err(|e| Error::string(&format!("微信API重试响应读取失败: {}", e)))?;

                    let retry_str = String::from_utf8_lossy(&retry_bytes);
                    if retry_str.contains("errcode") {
                        tracing::error!("微信生成小程序码重试异常: {}", retry_str);
                        if let Ok(err_data2) = serde_json::from_slice::<serde_json::Value>(&retry_bytes) {
                            Self::wx_exception_dispose(db, &err_data2, "微信小程序重试生成小程序码异常").await;
                        }
                    } else {
                        return Ok(Self::get_base64_image(&retry_bytes));
                    }
                }

                return Err(Error::string("微信生成二维码异常"));
            }
            return Err(Error::string("微信生成二维码异常"));
        }

        Ok(Self::get_base64_image(&bytes))
    }

// __CONTINUE_HERE_2__

    /// 获取微信小程序access_token
    ///
    /// Java: WechatNewServiceImpl.getMiniAccessToken()
    /// 1. 先从Redis缓存获取
    /// 2. 缓存不存在则从system_config读取appId和secret
    /// 3. 调用微信API获取新token
    /// 4. 缓存到Redis（过期时间 = expires_in - 1800秒）
    async fn get_mini_access_token(db: &DatabaseConnection) -> Result<String> {
        let redis = get_redis().await?;

        // 1. 尝试从Redis获取缓存的token
        if let Ok(Some(token)) = redis.get(constants::REDIS_WECHAT_MINI_ACCESS_TOKEN_KEY).await {
            if !token.is_empty() {
                return Ok(token);
            }
        }

        // 2. 从system_config获取appId和secret
        let app_id = SystemConfigService::get_value_by_key(db, constants::WECHAT_MINI_APPID)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;
        if app_id.trim().is_empty() {
            return Err(Error::BadRequest("微信小程序appId未设置".to_string()));
        }

        let secret = SystemConfigService::get_value_by_key(db, constants::WECHAT_MINI_APPSECRET)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;
        if secret.trim().is_empty() {
            return Err(Error::BadRequest("微信小程序secret未设置".to_string()));
        }

        // 3. 调用微信API获取access_token
        let token_vo = Self::get_access_token(&app_id, &secret).await?;

        let access_token = token_vo.access_token
            .ok_or_else(|| Error::string("微信返回的access_token为空"))?;
        let expires_in = token_vo.expires_in.unwrap_or(7200);

        // 4. 缓存到Redis（过期时间减去1800秒作为安全余量）
        let cache_seconds = (expires_in - 1800).max(60) as usize;
        let _ = redis.set(
            constants::REDIS_WECHAT_MINI_ACCESS_TOKEN_KEY,
            &access_token,
            cache_seconds,
        ).await;

        Ok(access_token)
    }

// __CONTINUE_HERE_3__

    /// 调用微信API获取access_token
    ///
    /// Java: WechatNewServiceImpl.getAccessToken()
    async fn get_access_token(app_id: &str, secret: &str) -> Result<WeChatAccessTokenVo> {
        let url = constants::WECHAT_ACCESS_TOKEN_URL
            .replacen("{}", app_id, 1)
            .replacen("{}", secret, 1);

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::string(&format!("微信获取access_token请求失败: {}", e)))?;

        let token_vo: WeChatAccessTokenVo = response
            .json()
            .await
            .map_err(|e| Error::string(&format!("微信获取access_token解析失败: {}", e)))?;

        // 检查错误
        if let Some(errcode) = token_vo.errcode {
            if errcode != 0 {
                let errmsg = token_vo.errmsg.as_deref().unwrap_or("未知错误");
                return Err(Error::string(&format!(
                    "微信获取access_token失败: errcode={}, errmsg={}", errcode, errmsg
                )));
            }
        }

        Ok(token_vo)
    }

    /// 记录微信异常到数据库
    ///
    /// Java: WechatNewServiceImpl.wxExceptionDispose()
    async fn wx_exception_dispose(
        db: &DatabaseConnection,
        data: &serde_json::Value,
        remark: &str,
    ) {
        let errcode = data.get("errcode")
            .map(|v| v.to_string())
            .unwrap_or_default();
        let errmsg = data.get("errmsg")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let data_str = data.to_string();
        let now = Local::now().naive_local();

        let active = wechat_exceptions::ActiveModel {
            errcode: Set(Some(errcode)),
            errmsg: Set(Some(errmsg)),
            data: Set(Some(data_str)),
            remark: Set(Some(remark.to_string())),
            create_time: Set(Some(now)),
            update_time: Set(Some(now)),
            ..Default::default()
        };

        if let Err(e) = active.insert(db).await {
            tracing::error!("记录微信异常失败: {}", e);
        }
    }

    /// 将二进制图片数据转为base64字符串
    ///
    /// Java: CrmebUtil.getBase64Image()
    fn get_base64_image(bytes: &[u8]) -> String {
        use base64::Engine;
        let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
        format!("data:image/png;base64,{}", b64)
    }
}
