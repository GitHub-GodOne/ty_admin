use serde::{Deserialize, Serialize};

/// 版权信息响应（对应Java CopyrightInfoResponse）
#[derive(Debug, Serialize)]
pub struct CopyrightInfoResponse {
    /// 管理端API域名
    #[serde(rename = "domainUrl")]
    pub domain_url: Option<String>,

    /// 项目版本号
    pub version: Option<String>,

    /// 版权标签
    pub label: Option<i32>,

    /// 授权码
    #[serde(rename = "authCode")]
    pub auth_code: Option<String>,

    /// 版权状态：-2=API域名未配置 -1=未提交，0-待审核，1-授权成功，2-审核失败
    pub status: Option<i32>,

    /// 公司信息
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,

    /// 公司图片
    #[serde(rename = "companyImage")]
    pub company_image: Option<String>,

    /// 版权码
    pub copyright: Option<String>,
}

/// 编辑公司版权信息请求（对应Java CopyrightUpdateInfoRequest）
#[derive(Debug, Deserialize)]
pub struct CopyrightUpdateInfoRequest {
    /// 公司信息
    #[serde(rename = "companyName")]
    pub company_name: String,

    /// 公司图片
    #[serde(rename = "companyImage")]
    pub company_image: Option<String>,
}

/// 授权信息响应（对应Java CopyrightConfigInfoResponse）
#[derive(Debug, Serialize)]
pub struct CopyrightConfigInfoResponse {
    /// 公司信息
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,

    /// 公司图片
    #[serde(rename = "companyImage")]
    pub company_image: Option<String>,
}
