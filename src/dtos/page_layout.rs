use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 底部导航响应
///
/// Java: PageLayoutBottomNavigationResponse
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageLayoutBottomNavigationResponse {
    /// 底部导航列表
    pub bottom_navigation_list: Vec<HashMap<String, serde_json::Value>>,
    /// 是否自定义
    pub is_custom: String,
}
