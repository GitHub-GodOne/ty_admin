use serde::Deserialize;

fn deserialize_empty_string_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        s.parse::<T>().map(Some).map_err(serde::de::Error::custom)
    }
}

/// 组合数据搜索请求
///
/// Java: SystemGroupDataSearchRequest
#[derive(Debug, Deserialize)]
pub struct SystemGroupDataSearchRequest {
    /// 分组id
    #[serde(default, deserialize_with = "deserialize_empty_string_as_none")]
    pub gid: Option<i32>,

    /// 状态（true：开启；false：关闭）
    #[serde(default)]
    pub status: Option<bool>,

    /// 关键字
    #[serde(default)]
    pub keywords: Option<String>,
}

/// 组合数据表单项
///
/// Java: SystemFormItemCheckRequest
#[derive(Debug, Deserialize, serde::Serialize, Clone)]
pub struct GroupDataFormItem {
    pub name: String,
    #[serde(default)]
    pub title: String,
    pub value: Option<serde_json::Value>,
}

/// 组合数据表单
///
/// Java: SystemFormCheckRequest (用于GroupData场景)
#[derive(Debug, Deserialize, serde::Serialize)]
pub struct GroupDataForm {
    /// 表单模版id
    #[serde(default)]
    pub id: Option<i32>,
    /// 排序
    #[serde(default)]
    pub sort: Option<i32>,
    /// 状态
    #[serde(default)]
    pub status: Option<bool>,
    /// 字段列表
    #[serde(default)]
    pub fields: Vec<GroupDataFormItem>,
}

/// 组合数据新增/修改请求
///
/// Java: SystemGroupDataRequest
#[derive(Debug, Deserialize)]
pub struct SystemGroupDataRequest {
    /// 对应的数据组id
    pub gid: Option<i32>,
    /// 表单数据
    pub form: GroupDataForm,
}
