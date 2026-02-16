use serde::{Deserialize, Deserializer, Serialize};

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

// ==================== 请求 DTO ====================

/// 定时任务新增/编辑请求
/// Java: ScheduleJobRequest
#[derive(Debug, Deserialize)]
pub struct ScheduleJobRequest {
    /// 任务id, 编辑时必传
    #[serde(rename = "jobId")]
    pub job_id: Option<i32>,

    /// spring bean名称
    #[serde(rename = "beanName")]
    pub bean_name: String,

    /// 方法名
    #[serde(rename = "methodName")]
    pub method_name: String,

    /// 参数
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub params: Option<String>,

    /// cron表达式
    #[serde(rename = "cronExpression")]
    pub cron_expression: String,

    /// 备注
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub remark: Option<String>,
}

/// 定时任务日志搜索请求
/// Java: ScheduleJobLogSearchRequest
#[derive(Debug, Deserialize)]
pub struct ScheduleJobLogSearchRequest {
    #[serde(rename = "jobId")]
    pub job_id: Option<i32>,

    #[serde(default, deserialize_with = "deserialize_empty_string_as_none", rename = "beanName")]
    pub bean_name: Option<String>,

    #[serde(default, deserialize_with = "deserialize_empty_string_as_none", rename = "methodName")]
    pub method_name: Option<String>,

    pub status: Option<i32>,
}

/// jobId查询参数
#[derive(Debug, Deserialize)]
pub struct JobIdQuery {
    #[serde(rename = "jobId")]
    pub job_id: i32,
}

// ==================== 响应 DTO ====================

/// 定时任务响应
/// Java: ScheduleJob
#[derive(Debug, Serialize)]
pub struct ScheduleJobResponse {
    #[serde(rename = "jobId")]
    pub job_id: i32,
    #[serde(rename = "beanName")]
    pub bean_name: Option<String>,
    #[serde(rename = "methodName")]
    pub method_name: Option<String>,
    pub params: Option<String>,
    #[serde(rename = "cronExpression")]
    pub cron_expression: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
}

/// 定时任务日志响应
/// Java: ScheduleJobLog
#[derive(Debug, Serialize)]
pub struct ScheduleJobLogResponse {
    #[serde(rename = "logId")]
    pub log_id: i32,
    #[serde(rename = "jobId")]
    pub job_id: i32,
    #[serde(rename = "beanName")]
    pub bean_name: Option<String>,
    #[serde(rename = "methodName")]
    pub method_name: Option<String>,
    pub params: Option<String>,
    pub status: i32,
    pub error: Option<String>,
    pub times: i32,
    #[serde(rename = "createTime")]
    pub create_time: Option<String>,
}
