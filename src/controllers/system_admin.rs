use axum::{
    extract::{Query, State},
    Json,
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, QueryOrder, Set, ActiveModelTrait};
use validator::Validate;

use crate::common::response::ApiResponse;
use crate::dtos::common::CommonPage;
use crate::models::_entities::{system_admin, prelude::TySystemAdmin};

/// 分页请求参数
#[derive(Debug, Deserialize, Validate)]
pub struct PageParamRequest {
    #[serde(default = "default_page", deserialize_with = "deserialize_number_from_string")]
    pub page: u64,
    #[serde(default = "default_limit", deserialize_with = "deserialize_number_from_string")]
    pub limit: u64,
}

fn default_page() -> u64 {
    1
}

fn default_limit() -> u64 {
    20
}

fn deserialize_number_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(u64),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse().map_err(D::Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

/// 后台管理员列表查询请求
#[derive(Debug, Deserialize, Validate)]
pub struct SystemAdminRequest {
    /// 后台管理员姓名
    #[serde(rename = "realName")]
    pub real_name: Option<String>,
    /// 后台管理员权限(menus_id)
    pub roles: Option<String>,
    /// 后台管理员状态 1有效0无效
    #[serde(default, deserialize_with = "deserialize_optional_number_from_string")]
    pub status: Option<i32>,
    #[serde(flatten)]
    pub page: PageParamRequest,
}

fn deserialize_optional_number_from_string<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i32),
    }

    let opt: Option<StringOrInt> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(StringOrInt::String(s)) => s.parse().map(Some).map_err(D::Error::custom),
        Some(StringOrInt::Int(i)) => Ok(Some(i)),
    }
}

/// 新增后台管理员请求
#[derive(Debug, Deserialize, Validate)]
pub struct SystemAdminAddRequest {
    /// 后台管理员账号
    #[validate(length(max = 32, message = "账号长度不能超过32个字符"))]
    pub account: String,
    /// 后台管理员密码
    #[validate(length(max = 32, message = "密码长度不能超过32个字符"))]
    pub pwd: String,
    /// 后台管理员姓名
    #[serde(rename = "realName")]
    #[validate(length(max = 16, message = "姓名长度不能超过16个字符"))]
    pub real_name: String,
    /// 后台管理员角色(menus_id)
    #[validate(length(max = 128, message = "角色组合长度不能超过128个字符"))]
    pub roles: String,
    /// 后台管理员状态 1有效0无效
    pub status: i32,
    /// 手机号
    #[validate(length(min = 11, max = 11, message = "请填写正确的手机号"))]
    pub phone: String,
}

/// 修改后台管理员请求
#[derive(Debug, Deserialize, Validate)]
pub struct SystemAdminUpdateRequest {
    /// 后台管理员表ID
    pub id: i32,
    /// 后台管理员账号
    #[validate(length(max = 32, message = "账号长度不能超过32个字符"))]
    pub account: String,
    /// 后台管理员密码
    #[validate(length(max = 32, message = "密码长度不能超过32个字符"))]
    pub pwd: String,
    /// 后台管理员姓名
    #[serde(rename = "realName")]
    #[validate(length(max = 16, message = "姓名长度不能超过16个字符"))]
    pub real_name: String,
    /// 后台管理员权限(menus_id)
    #[validate(length(max = 128, message = "角色组合长度不能超过128个字符"))]
    pub roles: String,
    /// 后台管理员状态 1有效0无效
    pub status: i32,
    /// 手机号码
    #[validate(length(min = 11, max = 11, message = "请填写正确的手机号"))]
    pub phone: String,
}

/// 后台管理员响应对象
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemAdminResponse {
    pub id: i32,
    pub account: String,
    #[serde(rename = "realName")]
    pub real_name: String,
    pub roles: String,
    #[serde(rename = "roleNames")]
    pub role_names: Option<String>,
    #[serde(rename = "lastIp")]
    pub last_ip: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "loginCount")]
    pub login_count: i32,
    pub level: i16,
    pub status: i16,
    pub phone: Option<String>,
    #[serde(rename = "isSms")]
    pub is_sms: Option<i16>,
}

impl From<system_admin::Model> for SystemAdminResponse {
    fn from(model: system_admin::Model) -> Self {
        Self {
            id: model.id as i32,
            account: model.account,
            real_name: model.real_name,
            roles: model.roles,
            role_names: None, // TODO: 需要关联查询角色名称
            last_ip: model.last_ip,
            update_time: model.update_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            create_time: model.create_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            login_count: model.login_count,
            level: model.level,
            status: model.status,
            phone: model.phone,
            is_sms: model.is_sms,
        }
    }
}

/// 分页列表
///
/// # 接口信息
/// - 路径: GET /api/admin/system/admin/list
/// - 权限: admin:system:admin:list
/// - 描述: 分页显示后台管理员列表
pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<SystemAdminRequest>,
) -> Result<Json<ApiResponse<CommonPage<SystemAdminResponse>>>> {
    let db = &ctx.db;

    // 构建查询条件
    let mut query = TySystemAdmin::find();

    if let Some(real_name) = &params.real_name {
        query = query.filter(system_admin::Column::RealName.contains(real_name));
    }

    if let Some(roles) = &params.roles {
        query = query.filter(system_admin::Column::Roles.eq(roles));
    }

    if let Some(status) = params.status {
        query = query.filter(system_admin::Column::Status.eq(status as u32));
    }

    // 添加排序
    query = query.order_by_desc(system_admin::Column::Id);

    // 分页查询
    let paginator = query.paginate(db, params.page.limit);
    let total = paginator.num_items().await?;
    let items = paginator.fetch_page(params.page.page - 1).await?;

    // 转换为响应对象
    let list: Vec<SystemAdminResponse> = items.into_iter().map(|item| item.into()).collect();

    let page_data = CommonPage::new(
        list,
        total as i64,
        params.page.page as i32,
        params.page.limit as i32,
    );

    Ok(Json(ApiResponse::success(page_data)))
}

/// 新增后台管理员
///
/// # 接口信息
/// - 路径: POST /api/admin/system/admin/save
/// - 权限: admin:system:admin:save
/// - 描述: 新增后台管理员
pub async fn save(
    State(ctx): State<AppContext>,
    Json(params): Json<SystemAdminAddRequest>,
) -> Result<Json<ApiResponse<()>>> {
    validator::Validate::validate(&params)
        .map_err(|e| Error::BadRequest(format!("参数验证失败: {}", e)))?;

    let db = &ctx.db;

    // 检查账号是否已存在
    let existing = TySystemAdmin::find()
        .filter(system_admin::Column::Account.eq(&params.account))
        .one(db)
        .await?;

    if existing.is_some() {
        return Ok(Json(ApiResponse::failed("账号已存在".to_string())));
    }

    // TODO: 密码加密处理
    let encrypted_pwd = params.pwd; // 实际应该加密

    // 创建新管理员
    let now = chrono::Local::now().naive_local();
    let new_admin = system_admin::ActiveModel {
        account: Set(params.account),
        pwd: Set(encrypted_pwd),
        real_name: Set(params.real_name),
        roles: Set(params.roles),
        status: Set(params.status as i16),
        phone: Set(Some(params.phone)),
        is_sms: Set(Some(0)),
        login_count: Set(0),
        level: Set(1),
        is_del: Set(0),
        last_ip: Set(None),
        update_time: Set(now),
        create_time: Set(now),
        ..Default::default()
    };

    new_admin.insert(db).await?;

    Ok(Json(ApiResponse::success_with_message((), "添加管理员成功".to_string())))
}

/// 删除后台管理员
///
/// # 接口信息
/// - 路径: GET /api/admin/system/admin/delete
/// - 权限: admin:system:admin:delete
/// - 描述: 删除后台管理员
#[derive(Debug, Deserialize)]
pub struct DeleteRequest {
    pub id: i32,
}

pub async fn delete(
    State(ctx): State<AppContext>,
    Query(params): Query<DeleteRequest>,
) -> Result<Json<ApiResponse<()>>> {
    let db = &ctx.db;

    // 查找管理员
    let admin = TySystemAdmin::find_by_id(params.id)
        .one(db)
        .await?;

    if admin.is_none() {
        return Ok(Json(ApiResponse::failed("管理员不存在".to_string())));
    }

    // 删除管理员
    TySystemAdmin::delete_by_id(params.id).exec(db).await?;

    Ok(Json(ApiResponse::success_with_message((), "删除成功".to_string())))
}

/// 修改后台管理员
///
/// # 接口信息
/// - 路径: POST /api/admin/system/admin/update
/// - 权限: admin:system:admin:update
/// - 描述: 修改后台管理员信息
pub async fn update(
    State(ctx): State<AppContext>,
    Json(params): Json<SystemAdminUpdateRequest>,
) -> Result<Json<ApiResponse<()>>> {
    validator::Validate::validate(&params)
        .map_err(|e| Error::BadRequest(format!("参数验证失败: {}", e)))?;

    let db = &ctx.db;

    // 查找管理员
    let admin = TySystemAdmin::find_by_id(params.id)
        .one(db)
        .await?;

    if admin.is_none() {
        return Ok(Json(ApiResponse::failed("管理员不存在".to_string())));
    }

    // TODO: 密码加密处理
    let encrypted_pwd = params.pwd;

    // 更新管理员信息
    let now = chrono::Local::now().naive_local();
    let mut admin: system_admin::ActiveModel = admin.unwrap().into();
    admin.account = Set(params.account);
    admin.pwd = Set(encrypted_pwd);
    admin.real_name = Set(params.real_name);
    admin.roles = Set(params.roles);
    admin.status = Set(params.status as i16);
    admin.phone = Set(Some(params.phone));
    admin.update_time = Set(now);

    admin.update(db).await?;

    Ok(Json(ApiResponse::success_with_message((), "修改成功".to_string())))
}

/// 后台管理员详情
///
/// # 接口信息
/// - 路径: GET /api/admin/system/admin/info
/// - 权限: admin:system:admin:info
/// - 描述: 获取后台管理员详情
#[derive(Debug, Deserialize)]
pub struct InfoRequest {
    pub id: i32,
}

pub async fn info(
    State(ctx): State<AppContext>,
    Query(params): Query<InfoRequest>,
) -> Result<Json<ApiResponse<system_admin::Model>>> {
    let db = &ctx.db;

    let admin = TySystemAdmin::find_by_id(params.id)
        .one(db)
        .await?;

    match admin {
        Some(admin) => Ok(Json(ApiResponse::success(admin))),
        None => Ok(Json(ApiResponse::failed("管理员不存在".to_string()))),
    }
}

/// 修改后台管理员状态
///
/// # 接口信息
/// - 路径: GET /api/admin/system/admin/updateStatus
/// - 权限: admin:system:admin:update:status
/// - 描述: 修改后台管理员状态
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub id: i32,
    pub status: bool,
}

pub async fn update_status(
    State(ctx): State<AppContext>,
    Query(params): Query<UpdateStatusRequest>,
) -> Result<Json<ApiResponse<String>>> {
    let db = &ctx.db;

    let admin = TySystemAdmin::find_by_id(params.id)
        .one(db)
        .await?;

    if admin.is_none() {
        return Ok(Json(ApiResponse::failed("管理员不存在".to_string())));
    }

    let mut admin: system_admin::ActiveModel = admin.unwrap().into();
    admin.status = Set(if params.status { 1 } else { 0 });
    admin.update_time = Set(chrono::Local::now().naive_local());

    admin.update(db).await?;

    Ok(Json(ApiResponse::success_with_message("修改成功".to_string(), "修改成功".to_string())))
}

/// 修改后台管理员是否接收短信状态
///
/// # 接口信息
/// - 路径: GET /api/admin/system/admin/update/isSms
/// - 权限: admin:system:admin:update:sms
/// - 描述: 修改后台管理员是否接收短信状态
#[derive(Debug, Deserialize)]
pub struct UpdateIsSmsRequest {
    pub id: i32,
}

pub async fn update_is_sms(
    State(ctx): State<AppContext>,
    Query(params): Query<UpdateIsSmsRequest>,
) -> Result<Json<ApiResponse<String>>> {
    let db = &ctx.db;

    let admin = TySystemAdmin::find_by_id(params.id)
        .one(db)
        .await?;

    if admin.is_none() {
        return Ok(Json(ApiResponse::failed("管理员不存在".to_string())));
    }

    let admin_model = admin.unwrap();
    let current_is_sms = admin_model.is_sms.unwrap_or(0);

    let mut admin: system_admin::ActiveModel = admin_model.into();
    admin.is_sms = Set(Some(if current_is_sms == 1 { 0 } else { 1 }));
    admin.update_time = Set(chrono::Local::now().naive_local());

    admin.update(db).await?;

    Ok(Json(ApiResponse::success_with_message("修改成功".to_string(), "修改成功".to_string())))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin/system/admin")
        .add("/list", get(list))
        .add("/save", post(save))
        .add("/delete", get(delete))
        .add("/update", post(update))
        .add("/info", get(info))
        .add("/updateStatus", get(update_status))
        .add("/update/isSms", get(update_is_sms))
}
