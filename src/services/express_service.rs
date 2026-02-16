/// 快递公司 -- 服务层
///
/// 实现与Java版本一致的快递公司管理逻辑
/// Java代码参考: com.zbkj.service.service.impl.ExpressServiceImpl
use loco_rs::prelude::*;
use sea_orm::*;

use crate::dtos::common::{CommonPage, PageParamRequest};
use crate::dtos::express::*;
use crate::dtos::one_pass;
use crate::initializers::redis::get_redis;
use crate::models::_entities::express;
use crate::services::one_pass_service::OnePassService;

/// 同步物流公司 Redis 缓存 key
const SYNC_EXPRESS_CACHE_KEY: &str = "sync_express";

/// 一号通快递公司列表接口
const ONE_PASS_API_EXPRESS_URI: &str = "v2/expr/express";

/// 一号通快递公司面单模板接口
const ONE_PASS_API_EXPRESS_TEMP_URI: &str = "v2/expr_dump/temp";

pub struct ExpressService;

impl ExpressService {
    /// 分页列表
    ///
    /// Java: ExpressServiceImpl.getList()
    /// 搜索条件: keywords 匹配 code LIKE 或 name LIKE
    /// 排序: sort 倒序, id 正序
    pub async fn get_list(
        db: &DatabaseConnection,
        request: &ExpressSearchRequest,
        page_param: &PageParamRequest,
    ) -> Result<CommonPage<ExpressResponse>> {
        let page = page_param.get_page().max(1);
        let limit = page_param.get_limit().max(1);

        let mut query = express::Entity::find();

        // 关键字搜索: code LIKE 或 name LIKE
        if let Some(keywords) = &request.keywords {
            let kw = keywords.trim();
            if !kw.is_empty() {
                query = query.filter(
                    Condition::any()
                        .add(express::Column::Code.contains(kw))
                        .add(express::Column::Name.contains(kw)),
                );
            }
        }

        // 排序: sort 倒序, id 正序（方便展示常用物流公司）
        query = query
            .order_by_desc(express::Column::Sort)
            .order_by_asc(express::Column::Id);

        // 查询总数
        let total = query.clone().count(db).await? as i64;

        // 分页查询
        let records = query
            .offset(((page - 1) * limit) as u64)
            .limit(limit as u64)
            .all(db)
            .await?;

        let list: Vec<ExpressResponse> = records
            .into_iter()
            .map(Self::model_to_response)
            .collect();

        Ok(CommonPage::new(list, total, page, limit))
    }

    /// 编辑快递公司
    ///
    /// Java: ExpressServiceImpl.updateExpress()
    /// 验证逻辑:
    /// 1. 如果 partner_id=true 且 account 为空，报错
    /// 2. 如果 partner_key=true 且 password 为空，报错
    /// 3. 如果 net=true 且 net_name 为空，报错
    pub async fn update_express(
        db: &DatabaseConnection,
        request: &ExpressUpdateRequest,
    ) -> Result<bool> {
        // 查找快递公司
        let existing = express::Entity::find_by_id(request.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("快递公司不存在!".to_string()))?;

        // 验证月结账号
        if existing.partner_id != 0 {
            if request.account.as_deref().unwrap_or("").trim().is_empty() {
                return Err(Error::BadRequest("请输入月结账号".to_string()));
            }
        }
        // 验证月结密码
        if existing.partner_key != 0 {
            if request.password.as_deref().unwrap_or("").trim().is_empty() {
                return Err(Error::BadRequest("请输入月结密码".to_string()));
            }
        }
        // 验证取件网点
        if existing.net != 0 {
            if request.net_name.as_deref().unwrap_or("").trim().is_empty() {
                return Err(Error::BadRequest("请输入取件网点".to_string()));
            }
        }

        // 更新
        let mut model: express::ActiveModel = existing.into();
        model.account = Set(request.account.clone().unwrap_or_default());
        model.password = Set(request.password.clone().unwrap_or_default());
        model.net_name = Set(request.net_name.clone().unwrap_or_default());
        model.sort = Set(request.sort);
        model.status = Set(if request.status { 1i16 } else { 0i16 });

        model.update(db).await?;
        Ok(true)
    }

    /// 修改显示状态
    ///
    /// Java: ExpressServiceImpl.updateExpressShow()
    pub async fn update_express_show(
        db: &DatabaseConnection,
        request: &ExpressUpdateShowRequest,
    ) -> Result<bool> {
        let existing = express::Entity::find_by_id(request.id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("编辑的记录不存在!".to_string()))?;

        let new_is_show: i16 = if request.is_show { 1 } else { 0 };

        // 如果状态相同，直接返回成功
        if existing.is_show == new_is_show {
            return Ok(true);
        }

        let mut model: express::ActiveModel = existing.into();
        model.is_show = Set(new_is_show);
        model.update(db).await?;
        Ok(true)
    }

    /// 同步物流公司
    ///
    /// Java: ExpressServiceImpl.syncExpress()
    /// 1. 检查 Redis 缓存，如果存在则跳过（1小时内不重复同步）
    /// 2. 从一号通平台获取物流公司列表
    /// 3. 过滤已存在的公司（按 code 去重）
    /// 4. 批量插入新公司
    /// 5. 设置 Redis 缓存标记（1小时过期）
    pub async fn sync_express(
        db: &DatabaseConnection,
    ) -> Result<bool> {
        let redis = get_redis().await?;

        // 检查缓存，1小时内不重复同步
        if let Ok(true) = redis.exists(SYNC_EXPRESS_CACHE_KEY).await {
            return Ok(true);
        }

        // 从一号通平台获取物流公司列表
        Self::get_express_list_from_platform(db).await?;

        // 设置缓存标记，1小时过期
        let _ = redis.set(SYNC_EXPRESS_CACHE_KEY, "1", 3600).await;

        Ok(true)
    }

    /// 查询全部物流公司
    ///
    /// Java: ExpressServiceImpl.findAll(String type)
    /// type: "normal" - 普通, "elec" - 电子面单
    pub async fn find_all(
        db: &DatabaseConnection,
        express_type: &str,
    ) -> Result<Vec<ExpressResponse>> {
        let mut query = express::Entity::find()
            .filter(express::Column::IsShow.eq(1i16));

        // 电子面单需要 status=true
        if express_type == "elec" {
            query = query.filter(express::Column::Status.eq(1i16));
        }

        query = query
            .order_by_desc(express::Column::Sort)
            .order_by_asc(express::Column::Id);

        let records = query.all(db).await?;

        Ok(records.into_iter().map(Self::model_to_response).collect())
    }

    /// 查询物流公司面单模板
    ///
    /// Java: ExpressServiceImpl.template(String com)
    /// 调用一号通API获取面单模板
    pub async fn template(
        db: &DatabaseConnection,
        com: &str,
    ) -> Result<serde_json::Value> {
        let login_vo = Self::get_onepass_login_vo(db).await?;
        let access_token = Self::get_onepass_token(&login_vo).await?;

        let url = format!(
            "{}{}?com={}&is_shipment=1",
            one_pass::ONE_PASS_API_URL,
            ONE_PASS_API_EXPRESS_TEMP_URI,
            com
        );

        let result = OnePassService::get_data_public(&url, &access_token).await?;
        Ok(result)
    }

    /// 获取快递公司详情
    ///
    /// Java: ExpressServiceImpl.getInfo(Integer id)
    pub async fn get_info(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<ExpressResponse> {
        let record = express::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| Error::BadRequest("快递公司不存在".to_string()))?;

        Ok(Self::model_to_response(record))
    }

    // ==================== 内部方法 ====================

    /// 从一号通平台获取物流公司列表并存入数据库
    ///
    /// Java: ExpressServiceImpl.getExpressList()
    async fn get_express_list_from_platform(db: &DatabaseConnection) -> Result<()> {
        let login_vo = Self::get_onepass_login_vo(db).await?;
        let access_token = Self::get_onepass_token(&login_vo).await?;

        let url = format!(
            "{}{}?page=0&limit=9999",
            one_pass::ONE_PASS_API_URL,
            ONE_PASS_API_EXPRESS_URI,
        );

        let post = OnePassService::get_data_public(&url, &access_token).await?;

        tracing::info!("OnePass Express ALL post = {}", post);

        // 解析返回数据
        let data = match post.get("data") {
            Some(d) => d,
            None => return Ok(()),
        };

        let json_array = match data.get("data").and_then(|d| d.as_array()) {
            Some(arr) => arr,
            None => return Ok(()),
        };

        if json_array.is_empty() {
            return Ok(());
        }

        // 获取已有的所有 code
        let existing_codes = Self::get_all_codes(db).await?;

        // 构建新的快递公司列表
        let mut new_express_list: Vec<express::ActiveModel> = Vec::new();

        for item in json_array {
            let code = item.get("code").and_then(|v| v.as_str()).unwrap_or("");
            if code.is_empty() || existing_codes.contains(&code.to_string()) {
                continue;
            }

            let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let partner_id_val = item.get("partner_id").and_then(|v| v.as_i64()).unwrap_or(0);
            let partner_key_val = item.get("partner_key").and_then(|v| v.as_i64()).unwrap_or(0);
            let net_val = item.get("net").and_then(|v| v.as_i64()).unwrap_or(0);

            let partner_id: i16 = if partner_id_val == 1 { 1 } else { 0 };
            let partner_key: i16 = if partner_key_val == 1 { 1 } else { 0 };
            let net: i16 = if net_val == 1 { 1 } else { 0 };

            // 如果不需要月结账号、密码、网点，则 status=true
            let status: i16 = if partner_id == 0 && partner_key == 0 && net == 0 {
                1
            } else {
                0
            };

            let model = express::ActiveModel {
                id: NotSet,
                code: Set(code.to_string()),
                name: Set(name.to_string()),
                partner_id: Set(partner_id),
                partner_key: Set(partner_key),
                net: Set(net),
                account: Set(String::new()),
                password: Set(String::new()),
                net_name: Set(String::new()),
                sort: Set(0),
                is_show: Set(1i16),
                status: Set(status),
            };

            new_express_list.push(model);
        }

        // 批量插入
        if !new_express_list.is_empty() {
            for model in new_express_list {
                express::Entity::insert(model)
                    .exec(db)
                    .await
                    .map_err(|e| Error::string(&format!("同步物流公司失败: {}", e)))?;
            }
        }

        Ok(())
    }

    /// 获取所有物流公司 code
    ///
    /// Java: ExpressServiceImpl.getAllCode()
    async fn get_all_codes(db: &DatabaseConnection) -> Result<Vec<String>> {
        let records = express::Entity::find()
            .all(db)
            .await?;

        Ok(records.into_iter().map(|r| r.code).collect())
    }

    /// 获取一号通登录凭证（内部复用）
    async fn get_onepass_login_vo(
        db: &DatabaseConnection,
    ) -> Result<one_pass::OnePassLoginVo> {
        use crate::services::system_config_service::SystemConfigService;

        let access_key = SystemConfigService::get_value_by_key(db, one_pass::ONE_PASS_ACCESS_KEY)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;

        if access_key.trim().is_empty() {
            return Err(Error::BadRequest(
                "请配置一号通 应用对应的 accessKey".to_string(),
            ));
        }

        let secret_key = SystemConfigService::get_value_by_key(db, one_pass::ONE_PASS_SECRET_KEY)
            .await
            .map_err(|e| Error::string(&e.to_string()))?;

        if secret_key.trim().is_empty() {
            return Err(Error::BadRequest(
                "请配置一号通 应用对应的 secretKey".to_string(),
            ));
        }

        Ok(one_pass::OnePassLoginVo {
            access_key,
            secret_key,
        })
    }

    /// 获取一号通 token（内部复用）
    async fn get_onepass_token(login_vo: &one_pass::OnePassLoginVo) -> Result<String> {
        let redis = get_redis().await?;
        let cache_key = format!("{}{}", one_pass::ONE_PASS_TOKEN_KEY_PREFIX, login_vo.access_key);

        // 尝试从 Redis 获取
        if let Ok(Some(token)) = redis.get(&cache_key).await {
            if !token.is_empty() {
                return Ok(token);
            }
        }

        // 调用登录接口
        let url = format!("{}{}", one_pass::ONE_PASS_API_URL, one_pass::USER_LOGIN_URI);
        let mut params = std::collections::HashMap::new();
        params.insert(one_pass::ONE_PASS_ACCESS_KEY, login_vo.access_key.as_str());
        params.insert(one_pass::ONE_PASS_SECRET_KEY, login_vo.secret_key.as_str());

        let client = reqwest::Client::new();
        let response = client.post(&url).form(&params).send().await
            .map_err(|e| Error::string(&format!("一号通登录请求失败: {}", e)))?;
        let result_text = response.text().await
            .map_err(|e| Error::string(&format!("一号通登录响应读取失败: {}", e)))?;

        let json: serde_json::Value = serde_json::from_str(&result_text)
            .map_err(|_| Error::string("一号通平台接口异常！"))?;

        if let Some(status) = json.get("status").and_then(|v| v.as_i64()) {
            if status == 400 {
                let msg = json.get("msg").and_then(|v| v.as_str()).unwrap_or("未知错误");
                return Err(Error::string(&format!("一号通平台接口{}", msg)));
            }
        }

        let data = json.get("data").ok_or_else(|| Error::string("一号通登录返回数据格式错误"))?;
        let access_token_raw = data.get("access_token").and_then(|v| v.as_str())
            .ok_or_else(|| Error::string("一号通登录返回的access_token为空"))?;
        let access_token = format!("{}{}", one_pass::ONE_PASS_USER_TOKEN_PREFIX, access_token_raw);
        let expires_in = data.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(3600);
        let cache_seconds = (expires_in - 1000).max(60) as usize;

        let _ = redis.set(&cache_key, &access_token, cache_seconds).await;

        Ok(access_token)
    }

    /// Model 转 Response DTO
    fn model_to_response(model: express::Model) -> ExpressResponse {
        ExpressResponse {
            id: model.id,
            code: model.code,
            name: model.name,
            partner_id: model.partner_id != 0,
            partner_key: model.partner_key != 0,
            net: model.net != 0,
            account: model.account,
            password: model.password,
            net_name: model.net_name,
            sort: model.sort,
            is_show: model.is_show != 0,
            status: model.status != 0,
        }
    }
}
