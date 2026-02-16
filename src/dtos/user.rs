use serde::{Deserialize, Deserializer, Serialize};
use rust_decimal::Decimal;

// ==================== 自定义反序列化 ====================

fn deserialize_empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        other => Ok(other),
    }
}

fn deserialize_optional_i16<'de, D>(deserializer: D) -> Result<Option<i16>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        Some(ref v) => Ok(v.parse::<i16>().ok()),
        None => Ok(None),
    }
}

fn deserialize_optional_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(ref v) if v.is_empty() => Ok(None),
        Some(ref v) => Ok(v.parse::<i32>().ok()),
        None => Ok(None),
    }
}

// ==================== 请求/响应 DTO ====================

/// 用户统计响应
#[derive(Debug, Serialize)]
pub struct UserStatisticsResponse {
    /// 总用户数
    #[serde(rename = "totalUsers")]
    pub total_users: i64,

    /// 新增用户数
    #[serde(rename = "newUsers")]
    pub new_users: i64,

    /// 活跃用户数
    #[serde(rename = "activeUsers")]
    pub active_users: i64,
}

/// 用户搜索请求 - 对应Java的UserSearchRequest
#[derive(Debug, Deserialize)]
pub struct UserSearchRequest {
    /// 关键字搜索（账号/昵称/手机号/真实姓名）
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub keywords: Option<String>,

    /// 用户标签ID
    #[serde(default, rename = "tagId", deserialize_with = "deserialize_empty_string_as_none")]
    pub tag_id: Option<String>,

    /// 用户标签ID (labelId别名)
    #[serde(default, rename = "labelId", deserialize_with = "deserialize_empty_string_as_none")]
    pub label_id: Option<String>,

    /// 用户分组ID
    #[serde(default, rename = "groupId", deserialize_with = "deserialize_empty_string_as_none")]
    pub group_id: Option<String>,

    /// 用户类型
    #[serde(default, rename = "userType", deserialize_with = "deserialize_empty_string_as_none")]
    pub user_type: Option<String>,

    /// 用户状态 0-禁用 1-正常
    #[serde(default, deserialize_with = "deserialize_optional_i16")]
    pub status: Option<i16>,

    /// 是否为推广员 0-否 1-是
    #[serde(default, rename = "isPromoter", deserialize_with = "deserialize_optional_i16")]
    pub is_promoter: Option<i16>,

    /// 用户等级
    #[serde(default, deserialize_with = "deserialize_optional_i16")]
    pub level: Option<i16>,

    /// 性别 0-未知 1-男 2-女
    #[serde(default, deserialize_with = "deserialize_optional_i16")]
    pub sex: Option<i16>,

    /// 国家
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub country: Option<String>,

    /// 省份
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub province: Option<String>,

    /// 城市
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub city: Option<String>,

    /// 注册时间范围
    #[serde(default, rename = "dateLimit", deserialize_with = "deserialize_empty_string_as_none")]
    pub date_limit: Option<String>,

    /// 消费次数
    #[serde(default, rename = "payCount", deserialize_with = "deserialize_empty_string_as_none")]
    pub pay_count: Option<String>,

    /// 消费次数（最小）
    #[serde(default, rename = "payCountMin", deserialize_with = "deserialize_optional_i32")]
    pub pay_count_min: Option<i32>,

    /// 消费次数（最大）
    #[serde(default, rename = "payCountMax", deserialize_with = "deserialize_optional_i32")]
    pub pay_count_max: Option<i32>,

    /// 访问情况
    #[serde(default, rename = "accessType", deserialize_with = "deserialize_empty_string_as_none")]
    pub access_type: Option<String>,
}

/// 用户响应 - 对应Java的UserResponse
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// 用户ID
    pub uid: i32,

    /// 账号
    pub account: String,

    /// 真实姓名
    #[serde(rename = "realName")]
    pub real_name: Option<String>,

    /// 生日
    pub birthday: Option<String>,

    /// 身份证号
    #[serde(rename = "cardId")]
    pub card_id: Option<String>,

    /// 备注
    pub mark: Option<String>,

    /// 合伙人ID
    #[serde(rename = "partnerId")]
    pub partner_id: Option<i32>,

    /// 用户分组ID
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,

    /// 用户标签ID
    #[serde(rename = "tagId")]
    pub tag_id: Option<String>,

    /// 昵称
    pub nickname: Option<String>,

    /// 头像
    pub avatar: Option<String>,

    /// 手机号
    pub phone: Option<String>,

    /// 注册IP
    #[serde(rename = "addIp")]
    pub add_ip: Option<String>,

    /// 最后登录IP
    #[serde(rename = "lastIp")]
    pub last_ip: Option<String>,

    /// 当前余额
    #[serde(rename = "nowMoney")]
    pub now_money: Option<Decimal>,

    /// 佣金金额
    #[serde(rename = "brokeragePrice")]
    pub brokerage_price: Option<Decimal>,

    /// 积分
    pub integral: Option<i32>,

    /// 经验值
    pub experience: Option<i32>,

    /// 签到次数
    #[serde(rename = "signNum")]
    pub sign_num: Option<i32>,

    /// 状态 0-禁用 1-正常
    pub status: bool,

    /// 用户等级
    pub level: Option<i16>,

    /// 推广员ID
    #[serde(rename = "spreadUid")]
    pub spread_uid: Option<i32>,

    /// 推广员关联时间
    #[serde(rename = "spreadTime")]
    pub spread_time: Option<String>,

    /// 用户类型
    #[serde(rename = "userType")]
    pub user_type: String,

    /// 是否为推广员
    #[serde(rename = "isPromoter")]
    pub is_promoter: bool,

    /// 消费次数
    #[serde(rename = "payCount")]
    pub pay_count: Option<i32>,

    /// 推广人数
    #[serde(rename = "spreadCount")]
    pub spread_count: Option<i32>,

    /// 地址
    pub addres: Option<String>,

    /// 管理员ID
    pub adminid: Option<i32>,

    /// 登录类型
    #[serde(rename = "loginType")]
    pub login_type: String,

    /// 创建时间
    #[serde(rename = "createTime")]
    pub create_time: String,

    /// 更新时间
    #[serde(rename = "updateTime")]
    pub update_time: String,

    /// 最后登录时间
    #[serde(rename = "lastLoginTime")]
    pub last_login_time: Option<String>,

    /// 路径
    pub path: String,

    /// 是否关注公众号
    pub subscribe: bool,

    /// 关注时间
    #[serde(rename = "subscribeTime")]
    pub subscribe_time: Option<String>,

    /// 性别 0-未知 1-男 2-女
    pub sex: Option<i16>,

    /// 国家
    pub country: Option<String>,

    /// 成为推广员时间
    #[serde(rename = "promoterTime")]
    pub promoter_time: Option<String>,

    /// 是否注销
    #[serde(rename = "isLogoff")]
    pub is_logoff: bool,

    /// 注销时间
    #[serde(rename = "logoffTime")]
    pub logoff_time: Option<String>,
}

/// 用户渠道统计
#[derive(Debug, Serialize)]
pub struct UserChannelResponse {
    /// 渠道名称
    pub channel: String,

    /// 用户数量
    pub count: i64,

    /// 占比
    pub percentage: f64,
}

/// 用户概览响应
#[derive(Debug, Serialize)]
pub struct UserOverviewResponse {
    /// 总用户数
    #[serde(rename = "totalUsers")]
    pub total_users: i64,

    /// 今日新增
    #[serde(rename = "todayNew")]
    pub today_new: i64,

    /// 本周新增
    #[serde(rename = "weekNew")]
    pub week_new: i64,

    /// 本月新增
    #[serde(rename = "monthNew")]
    pub month_new: i64,
}
