/// 一号通服务 -- 服务层
///
/// 实现与Java版本一致的一号通服务管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.OnePassServiceImpl
use loco_rs::prelude::*;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

use crate::dtos::one_pass::*;
use crate::initializers::redis::get_redis;
use crate::services::system_config_service::SystemConfigService;

pub struct OnePassService;

impl OnePassService {
    // ==================== Controller 暴露的方法 ====================

    /// 保存一号通应用信息
    ///
    /// Java: OnePassServiceImpl.saveOnePassApplicationInfo()
    /// 将 accessKey 和 secretKey 保存到 system_config 表
    pub async fn save_application_info(
        db: &DatabaseConnection,
        request: &OnePassLoginRequest,
    ) -> Result<bool> {
        // 验证参数
        if request.access_key.trim().is_empty() {
            return Err(Error::BadRequest("AccessKey 不能为空".to_string()));
        }
        if request.secret_key.trim().is_empty() {
            return Err(Error::BadRequest("SecretKey 不能为空".to_string()));
        }

        let redis = get_redis().await?;

        // 保存 access_key
        SystemConfigService::update_or_save_value_by_name(
            db, &redis, ONE_PASS_ACCESS_KEY, &request.access_key,
        ).await.map_err(|e| Error::string(&e.to_string()))?;

        // 保存 secret_key
        SystemConfigService::update_or_save_value_by_name(
            db, &redis, ONE_PASS_SECRET_KEY, &request.secret_key,
        ).await.map_err(|e| Error::string(&e.to_string()))?;

        Ok(true)
    }

    /// 获取一号通应用信息
    ///
    /// Java: OnePassServiceImpl.getOnePassApplicationInfo()
    /// 从 system_config 表读取 accessKey 和 secretKey
    pub async fn get_application_info(
        db: &DatabaseConnection,
    ) -> Result<OnePassLoginRequest> {
        let access_key = SystemConfigService::get_value_by_key(db, ONE_PASS_ACCESS_KEY)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;
        let secret_key = SystemConfigService::get_value_by_key(db, ONE_PASS_SECRET_KEY)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;

        Ok(OnePassLoginRequest {
            access_key,
            secret_key,
        })
    }

    /// 取消商家寄件
    ///
    /// Java: OnePassServiceImpl.shipmentCancelOrder()
    /// 调用一号通外部API取消寄件订单
    pub async fn shipment_cancel_order(
        db: &DatabaseConnection,
        request: &OnePassShipmentCancelOrderRequest,
    ) -> Result<serde_json::Value> {
        // 验证参数
        if request.task_id.trim().is_empty() {
            return Err(Error::BadRequest("任务ID 不能为空".to_string()));
        }
        if request.order_id.trim().is_empty() {
            return Err(Error::BadRequest("订单ID 不能为空".to_string()));
        }
        if request.cancel_msg.trim().is_empty() {
            return Err(Error::BadRequest("取消理由 不能为空".to_string()));
        }

        let login_vo = Self::get_login_vo(db).await?;
        let access_token = Self::get_token(&login_vo).await?;
        let header = Self::get_common_header(&access_token);

        let mut params = HashMap::new();
        params.insert("task_id", request.task_id.as_str());
        params.insert("order_id", request.order_id.as_str());
        params.insert("cancel_msg", request.cancel_msg.as_str());

        let url = format!("{}{}", ONE_PASS_API_URL, SHIPMENT_CANCEL_ORDER_URI);
        let json_result = Self::post_form(&url, &params, &header).await?;

        tracing::info!("一号通-商家取消寄件: {}", json_result);

        // 返回 data 字段
        Ok(json_result.get("data").cloned().unwrap_or(serde_json::Value::Null))
    }

    /// 获取商家寄件快递公司列表
    ///
    /// Java: OnePassServiceImpl.shipmentComs()
    /// 调用一号通外部API获取快递公司列表
    pub async fn shipment_coms(
        db: &DatabaseConnection,
    ) -> Result<serde_json::Value> {
        let login_vo = Self::get_login_vo(db).await?;
        let access_token = Self::get_token(&login_vo).await?;
        let header = Self::get_common_header(&access_token);

        let url = format!("{}{}", ONE_PASS_API_URL, SHIPMENT_GET_KUAIDI_COMS_URI);
        let json_result = Self::get_data(&url, &header).await?;

        tracing::info!("一号通-商家寄件-物流地址: {}", json_result);

        Ok(json_result)
    }

    /// 商家寄件回调处理
    ///
    /// Java: OnePassServiceImpl.shipmentCallBackMethod()
    /// 处理一号通商家寄件的各种回调事件
    /// 回调类型:
    /// - order_success: 下单成功回调
    /// - order_take: 取件回调
    /// - order_cancel: 用户主动取消回调
    /// - order_fail: 下单失败回调
    /// - order_receipt: 快递签收回调
    pub async fn shipment_callback(
        callback_type: &str,
        data: &str,
    ) -> Result<bool> {
        tracing::info!("一号通-商家寄件-回调:type:{}", callback_type);
        tracing::info!("一号通-商家寄件-回调:data:{}", data);

        // 尝试解密回调数据
        let json_data: serde_json::Value = match Self::decrypt_callback_data(data) {
            Ok(decrypted) => {
                serde_json::from_str(&decrypted).unwrap_or(serde_json::Value::Null)
            }
            Err(e) => {
                tracing::error!("一号通商家寄件 解密数据失败: {}", e);
                serde_json::Value::Null
            }
        };

        // 根据回调类型处理
        match callback_type {
            "order_success" => {
                // 下单成功回调 - 暂无处理
                tracing::info!("一号通回调-下单成功: {}", json_data);
            }
            "order_take" => {
                // 取件回调 - 更新订单快递信息
                tracing::info!("一号通回调-取件: {}", json_data);
                // TODO: storeOrderService.expressForOnePassShipmentTakeCallBack(jsonObject)
            }
            "order_cancel" | "order_fail" => {
                // 取消/失败回调 - 清理商家寄件数据
                tracing::info!("一号通回调-取消/失败: {}", json_data);
                // TODO: storeOrderService.expressForOnePassShipmentCancelCallBack(jsonObject)
            }
            "order_receipt" => {
                // 签收回调 - 后期可规划和订单签收结果合并
                tracing::info!("一号通回调-签收: {}", json_data);
            }
            _ => {
                tracing::warn!("一号通回调-未知类型: {}", callback_type);
            }
        }

        Ok(true)
    }

    // ==================== 工具方法 ====================

    /// 公开的 GET 请求方法（供其他 Service 调用）
    ///
    /// 发送带 Authorization 头的 GET 请求到一号通平台
    pub async fn get_data_public(
        url: &str,
        access_token: &str,
    ) -> Result<serde_json::Value> {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), access_token.to_string());
        Self::get_data(url, &headers).await
    }

    /// 获取一号通登录凭证
    ///
    /// Java: OnePassUtil.getLoginVo()
    /// 从 system_config 读取 access_key 和 secret_key
    async fn get_login_vo(db: &DatabaseConnection) -> Result<OnePassLoginVo> {
        let access_key = SystemConfigService::get_value_by_key(db, ONE_PASS_ACCESS_KEY)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;

        if access_key.trim().is_empty() {
            return Err(Error::BadRequest(
                "请配置一号通 应用对应的 accessKey".to_string(),
            ));
        }

        let secret_key = SystemConfigService::get_value_by_key(db, ONE_PASS_SECRET_KEY)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;

        if secret_key.trim().is_empty() {
            return Err(Error::BadRequest(
                "请配置一号通 应用对应的 secretKey".to_string(),
            ));
        }

        Ok(OnePassLoginVo {
            access_key,
            secret_key,
        })
    }

    /// 获取一号通 access token
    ///
    /// Java: OnePassUtil.getToken(OnePassLoginVo loginVo)
    /// 1. 先从 Redis 缓存获取
    /// 2. 缓存不存在则调用登录接口获取新 token
    /// 3. 将新 token 存入 Redis 缓存
    async fn get_token(login_vo: &OnePassLoginVo) -> Result<String> {
        let redis = get_redis().await?;
        let cache_key = format!("{}{}", ONE_PASS_TOKEN_KEY_PREFIX, login_vo.access_key);

        // 1. 尝试从 Redis 获取缓存的 token
        if let Ok(Some(token)) = redis.get(&cache_key).await {
            if !token.is_empty() {
                return Ok(token);
            }
        }

        // 2. 缓存不存在，调用登录接口
        let url = format!("{}{}", ONE_PASS_API_URL, USER_LOGIN_URI);
        let mut params = HashMap::new();
        params.insert(ONE_PASS_ACCESS_KEY, login_vo.access_key.as_str());
        params.insert(ONE_PASS_SECRET_KEY, login_vo.secret_key.as_str());

        let json_result = Self::post_form(&url, &params, &HashMap::new()).await?;

        // 3. 解析 token 和过期时间
        let data = json_result.get("data")
            .ok_or_else(|| Error::string("一号通登录返回数据格式错误"))?;

        let access_token_raw = data.get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::string("一号通登录返回的access_token为空"))?;

        let access_token = format!("{}{}", ONE_PASS_USER_TOKEN_PREFIX, access_token_raw);

        let expires_in = data.get("expires_in")
            .and_then(|v| v.as_i64())
            .unwrap_or(3600);

        // 过期时间减去1000秒作为缓存时间（与Java一致）
        let cache_seconds = (expires_in - 1000).max(60) as usize;

        // 4. 存入 Redis 缓存
        let _ = redis.set(&cache_key, &access_token, cache_seconds).await;

        Ok(access_token)
    }

    /// 构建通用请求头
    ///
    /// Java: OnePassUtil.getCommonHeader(String accessToken)
    fn get_common_header(access_token: &str) -> HashMap<String, String> {
        let mut header = HashMap::new();
        header.insert("Authorization".to_string(), access_token.to_string());
        header
    }

    /// POST 表单请求
    ///
    /// Java: OnePassUtil.postFrom()
    /// 发送 form-urlencoded POST 请求到一号通平台
    async fn post_form(
        url: &str,
        params: &HashMap<&str, &str>,
        headers: &HashMap<String, String>,
    ) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let mut request_builder = client.post(url);

        // 添加自定义请求头
        for (key, value) in headers {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }

        // 发送 form 请求
        let response = request_builder
            .form(params)
            .send()
            .await
            .map_err(|e| Error::string(&format!("一号通平台接口请求失败: {}", e)))?;

        let result_text = response
            .text()
            .await
            .map_err(|e| Error::string(&format!("一号通平台接口响应读取失败: {}", e)))?;

        tracing::info!("OnePass-postForm: {}", result_text);

        Self::check_result(&result_text)
    }

    /// GET 请求
    ///
    /// Java: OnePassUtil.getData()
    /// 发送 GET 请求到一号通平台
    async fn get_data(
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<serde_json::Value> {
        let client = reqwest::Client::new();
        let mut request_builder = client.get(url);

        // 添加自定义请求头
        for (key, value) in headers {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| Error::string(&format!("一号通平台接口请求失败: {}", e)))?;

        let result_text = response
            .text()
            .await
            .map_err(|e| Error::string(&format!("一号通平台接口响应读取失败: {}", e)))?;

        tracing::info!("OnePass-getData: {}", result_text);

        Self::check_result(&result_text)
    }

    /// 检查一号通API返回结果
    ///
    /// Java: OnePassUtil.checkResult(String result)
    /// 验证返回数据格式和状态码
    fn check_result(result: &str) -> Result<serde_json::Value> {
        if result.trim().is_empty() {
            return Err(Error::string("一号通平台接口异常，没任何数据返回！"));
        }

        let json: serde_json::Value = serde_json::from_str(result)
            .map_err(|_| Error::string("一号通平台接口异常！"))?;

        // 检查错误码
        if let Some(status) = json.get("status").and_then(|v| v.as_i64()) {
            if status == ONE_PASS_ERROR_CODE as i64 {
                let msg = json.get("msg")
                    .and_then(|v| v.as_str())
                    .unwrap_or("未知错误");
                return Err(Error::string(&format!("一号通平台接口{}", msg)));
            }
        }

        Ok(json)
    }

    /// 解密回调数据
    ///
    /// Java: com.zbkj.common.utils.OnePassUtil.decrypt(data)
    /// AES/CBC/PKCS5Padding 解密
    /// 注意：此处为简化实现，实际需要AES解密
    fn decrypt_callback_data(data: &str) -> std::result::Result<String, String> {
        // Java实现使用 AES/CBC/PKCS5Padding 解密
        // key = "user-AppSecret-key" 的MD5前16字节
        // IV = 密文前16字节
        // 实际密文 = 密文[16..]
        //
        // 如果数据本身就是JSON，直接返回
        if let Ok(_) = serde_json::from_str::<serde_json::Value>(data) {
            return Ok(data.to_string());
        }

        // 尝试 Base64 解码 + AES 解密
        // 使用与Java一致的密钥: "user-AppSecret-key"
        use base64::Engine;
        let encrypted = base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|e| format!("Base64解码失败: {}", e))?;

        if encrypted.len() < 16 {
            return Err("加密数据长度不足".to_string());
        }

        // IV = 前16字节, 密文 = 后续字节
        let iv = &encrypted[..16];
        let ciphertext = &encrypted[16..];

        // 密钥 = MD5("user-AppSecret-key") 的前16字节
        let key_str = "user-AppSecret-key";
        let key_md5 = format!("{:x}", md5::compute(key_str.as_bytes()));
        let key_bytes = &key_md5.as_bytes()[..16];

        // AES-128-CBC 解密
        use aes::cipher::{BlockDecryptMut, KeyIvInit};
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

        let mut buf = ciphertext.to_vec();
        let decryptor = Aes128CbcDec::new_from_slices(key_bytes, iv)
            .map_err(|e| format!("AES初始化失败: {}", e))?;

        let decrypted = decryptor
            .decrypt_padded_mut::<aes::cipher::block_padding::Pkcs7>(&mut buf)
            .map_err(|e| format!("AES解密失败: {}", e))?;

        String::from_utf8(decrypted.to_vec())
            .map_err(|e| format!("UTF-8解码失败: {}", e))
    }
}
