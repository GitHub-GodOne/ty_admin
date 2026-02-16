/// 易联云(YiLianYun) 工具类
///
/// 处理易联云打印机的OAuth认证、打印机注册、打印内容格式化和API调用
/// Java参考: com.zbkj.service.util.YlyUtil + RequestMethod + Utils
use std::collections::HashMap;
use tracing::{info, error};
use uuid::Uuid;

use crate::common::constants;
use crate::dtos::yly_print::{YlyAccessTokenResponse, YlyPrintRequest, YlyPrintRequestGoods};
use crate::services::system_config_service::SystemConfigService;
use crate::utils::redis_client::RedisClient;

/// 30天过期时间（秒）
const YLY_TOKEN_EXPIRE_SECONDS: usize = 30 * 24 * 60 * 60;

/// 简单的URL编码实现（与Java URLEncoder.encode(str, "utf-8")一致）
fn url_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

/// 易联云工具类
pub struct YlyUtil;

impl YlyUtil {
    /// 获取当前时间戳（秒）
    /// Java: Utils.getTimestamp()
    fn get_timestamp() -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        (now.as_secs()).to_string()
    }

    /// 计算MD5签名
    /// Java: Utils.getMD5Str(str)
    fn md5_sign(input: &str) -> String {
        let digest = md5::compute(input.as_bytes());
        format!("{:032x}", digest)
    }

    /// 构建签名参数的公共部分
    /// Java: RequestMethod中每个方法的公共逻辑
    fn build_common_params(client_id: &str, client_secret: &str) -> HashMap<String, String> {
        let timestamp = Self::get_timestamp();
        let sign_str = format!("{}{}{}", client_id, timestamp, client_secret);
        let sign = Self::md5_sign(&sign_str);

        let mut params = HashMap::new();
        params.insert("client_id".to_string(), client_id.to_string());
        params.insert("timestamp".to_string(), timestamp);
        params.insert("sign".to_string(), sign);
        params.insert("id".to_string(), Uuid::new_v4().to_string());
        params
    }

    /// 发送POST请求到易联云API
    /// Java: HttpRequest.sendPost(url, paramMap)
    async fn send_post(url: &str, params: &HashMap<String, String>) -> Result<String, String> {
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .form(params)
            .send()
            .await
            .map_err(|e| format!("易联云HTTP请求失败: {}", e))?;

        let body = response
            .text()
            .await
            .map_err(|e| format!("易联云HTTP响应读取失败: {}", e))?;

        Ok(body)
    }

    /// 获取AccessToken（client_credentials模式）
    /// Java: RequestMethod.getAccessToken()
    async fn get_access_token(client_id: &str, client_secret: &str) -> Result<String, String> {
        let mut params = Self::build_common_params(client_id, client_secret);
        params.insert("grant_type".to_string(), "client_credentials".to_string());
        params.insert("scope".to_string(), "all".to_string());

        Self::send_post(constants::YLY_URL_OAUTH, &params).await
    }

    /// 添加打印机
    /// Java: RequestMethod.addPrinter(machine_code, msign, access_token)
    async fn add_printer(
        client_id: &str,
        client_secret: &str,
        machine_code: &str,
        msign: &str,
        access_token: &str,
    ) -> Result<String, String> {
        let mut params = Self::build_common_params(client_id, client_secret);
        params.insert("machine_code".to_string(), machine_code.to_string());
        params.insert("msign".to_string(), msign.to_string());
        params.insert("access_token".to_string(), access_token.to_string());

        Self::send_post(constants::YLY_URL_ADD_PRINTER, &params).await
    }

    /// 文本打印
    /// Java: RequestMethod.printIndex(access_token, machine_code, content, origin_id)
    async fn print_index(
        client_id: &str,
        client_secret: &str,
        access_token: &str,
        machine_code: &str,
        content: &str,
        origin_id: &str,
    ) -> Result<String, String> {
        let mut params = Self::build_common_params(client_id, client_secret);
        params.insert("access_token".to_string(), access_token.to_string());
        params.insert("machine_code".to_string(), machine_code.to_string());
        params.insert("content".to_string(), content.to_string());
        params.insert("origin_id".to_string(), origin_id.to_string());

        Self::send_post(constants::YLY_URL_PRINT_INDEX, &params).await
    }

    /// 初始化易联云打印机并获取AccessToken
    /// Java: YlyUtil.instant()
    /// 从数据库读取配置 → 获取/缓存Token → 注册打印机
    async fn instant(
        db: &sea_orm::DatabaseConnection,
        redis: &RedisClient,
    ) -> Result<(String, String, String, String), String> {
        // 读取配置
        let client_id = SystemConfigService::get_value_by_key(db, constants::YLY_PRINT_APP_ID)
            .await.map_err(|e| e.to_string())?;
        let client_secret = SystemConfigService::get_value_by_key(db, constants::YLY_PRINT_APP_SECRET)
            .await.map_err(|e| e.to_string())?;
        let machine_code = SystemConfigService::get_value_by_key(db, constants::YLY_PRINT_APP_MACHINE_CODE)
            .await.map_err(|e| e.to_string())?;
        let msign = SystemConfigService::get_value_by_key(db, constants::YLY_PRINT_APP_MACHINE_MSIGN)
            .await.map_err(|e| e.to_string())?;

        if client_id.is_empty() || client_secret.is_empty()
            || machine_code.is_empty() || msign.is_empty()
        {
            return Err("易联云配置数据不完整".to_string());
        }

        // 获取AccessToken（优先从Redis缓存）
        let access_token = match redis.get(constants::YLY_REDIS_TOKEN).await {
            Ok(Some(cached)) => {
                // 从缓存中解析token
                let token_resp: YlyAccessTokenResponse = serde_json::from_str(&cached)
                    .map_err(|e| format!("解析缓存Token失败: {}", e))?;
                match token_resp.body {
                    Some(body) if !body.access_token.is_empty() => body.access_token,
                    _ => {
                        // 缓存中的token无效，重新获取
                        Self::fetch_and_cache_token(&client_id, &client_secret, redis).await?
                    }
                }
            }
            _ => {
                // 缓存不存在，获取新token
                Self::fetch_and_cache_token(&client_id, &client_secret, redis).await?
            }
        };

        info!("获取的易联云AccessToken: {}", &access_token[..std::cmp::min(10, access_token.len())]);

        // 添加打印机
        match Self::add_printer(&client_id, &client_secret, &machine_code, &msign, &access_token).await {
            Ok(result) => info!("添加打印机结果: {}", result),
            Err(e) => error!("添加易联云打印机失败: {}", e),
        }

        Ok((client_id, client_secret, machine_code, access_token))
    }

    /// 获取新Token并缓存到Redis
    async fn fetch_and_cache_token(
        client_id: &str,
        client_secret: &str,
        redis: &RedisClient,
    ) -> Result<String, String> {
        let token_json = Self::get_access_token(client_id, client_secret).await?;
        let token_resp: YlyAccessTokenResponse = serde_json::from_str(&token_json)
            .map_err(|e| format!("解析AccessToken响应失败: {}, 原始响应: {}", e, token_json))?;

        // 缓存到Redis，30天过期
        let _ = redis.set(constants::YLY_REDIS_TOKEN, &token_json, YLY_TOKEN_EXPIRE_SECONDS).await;

        match token_resp.body {
            Some(body) if !body.access_token.is_empty() => Ok(body.access_token),
            _ => Err(format!("获取AccessToken失败: {}", token_resp.error_description)),
        }
    }

    /// 检查是否开启打印
    /// Java: YlyUtil.checkYlyPrintStatus()
    /// 返回true表示【未开启】打印（与Java逻辑一致）
    pub async fn check_yly_print_status(db: &sea_orm::DatabaseConnection) -> bool {
        let status = SystemConfigService::get_value_by_key(db, constants::YLY_PRINT_STATUS)
            .await
            .unwrap_or_default();
        !status.is_empty() && status == "'0'"
    }

    /// 检查是否开启自动打印
    /// Java: YlyUtil.checkYlyPrintAfterPaySuccess()
    /// 返回true表示【未开启】自动打印
    pub async fn check_yly_print_after_pay_success(db: &sea_orm::DatabaseConnection) -> bool {
        let auto_status = SystemConfigService::get_value_by_key(db, constants::YLY_PRINT_AUTO_STATUS)
            .await
            .unwrap_or_default();
        !auto_status.is_empty() && auto_status == "'0'"
    }

    /// 格式化商品列表为打印格式
    /// Java: YlyUtil.ylyPrintFormatGoodsList(List<YlyPrintRequestGoods> goods)
    fn format_goods_list(goods: &[YlyPrintRequestGoods]) -> String {
        let mut result = String::new();
        for good in goods {
            result.push_str(&good.goods_name);
            result.push(' ');
            result.push_str(&good.unit_price);
            result.push(' ');
            result.push_str(&good.num);
            result.push(' ');
            result.push_str(&good.money);
            result.push('\n');
        }
        result
    }

    /// 构建打印内容（YLY标记语言）
    /// Java: YlyUtil.ylyPrint() 中的 printSb 拼接
    fn build_print_content(request: &YlyPrintRequest) -> String {
        let goods_str = Self::format_goods_list(&request.goods);

        format!(
            "<FH><FB><center>{}</center></FB></FH>\
             ********************************<FH>\
             订单编号：{}\n\
             日   期：{}\n\
             姓   名：{}\n\
             电   话：{}\n\
             地   址：{}\n\
             订单备注：{}</FH>\n\
             ********************************\n\
             <FH>\
             商品名称 单价 数量 金额\n\
             {}\
             </FH>\
             ********************************\n\
             <FH>\
             <LR>合计：¥{}元，优惠：¥{}元</LR>\
             <LR>邮费：¥{}元，抵扣：¥{}元</LR>\
             </FH>\
             <FH><right>实际支付：¥{}元</right></FH>\
             <FB><FB><center>完</center></FB></FB>",
            request.business_name,
            request.order_no,
            request.date,
            request.name,
            request.phone,
            request.address,
            request.note,
            goods_str,
            request.amount,
            request.discount,
            request.postal,
            request.deduction,
            request.pay_money,
        )
    }

    /// 执行打印
    /// Java: YlyUtil.ylyPrint(YlyPrintRequest)
    /// 初始化连接 → 构建打印内容 → URL编码 → 调用打印API
    pub async fn yly_print(
        db: &sea_orm::DatabaseConnection,
        redis: &RedisClient,
        request: &YlyPrintRequest,
    ) -> Result<(), String> {
        // 初始化并获取配置
        let (client_id, client_secret, machine_code, access_token) =
            Self::instant(db, redis).await?;

        // 构建打印内容
        let print_content = Self::build_print_content(request);

        // URL编码（与Java URLEncoder.encode一致）
        let encoded_content = url_encode(&print_content);

        // 调用打印API
        let result = Self::print_index(
            &client_id,
            &client_secret,
            &access_token,
            &machine_code,
            &encoded_content,
            "order111",
        )
        .await?;

        info!("易联云打印结果: {}", result);
        Ok(())
    }
}
