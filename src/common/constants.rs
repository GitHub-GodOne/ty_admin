/// 系统常量
///
/// 统一管理所有常量，与Java Constants/SysConfigConstants/CopyrightConstants 一致

// ==================== Redis缓存Key ====================

/// 配置列表缓存Key（Java: Constants.CONFIG_LIST）
pub const CONFIG_LIST_KEY: &str = "config_list";

// ==================== 系统配置Key（Java: Constants / SysConfigConstants） ====================

/// API域名配置Key（Java: Constants.CONFIG_KEY_API_URL）
pub const CONFIG_KEY_API_URL: &str = "api_url";

/// 文件上传类型配置Key
pub const CONFIG_UPLOAD_TYPE: &str = "uploadType";

/// 腾讯地图Key配置Key
pub const CONFIG_TX_MAP_KEY: &str = "txMapKey";

/// 移动端首页列表样式配置Key
pub const CONFIG_HOME_PAGE_SALE_LIST_STYLE: &str = "homePageSaleListStyle";

/// 授权Host配置Key
pub const CONFIG_AUTH_HOST: &str = "authHost";

/// 主题颜色配置Key
pub const CONFIG_CHANGE_COLOR: &str = "changeColor";

/// 方形Logo配置Key
pub const CONFIG_SITE_LOGO_SQUARE: &str = "siteLogoSquare";

/// 左上角Logo配置Key
pub const CONFIG_SITE_LOGO_LEFT_TOP: &str = "siteLogoLeftTop";

// ==================== 版权相关常量（Java: SysConfigConstants / CopyrightConstants） ====================

/// 版权标签配置Key（Java: SysConfigConstants.CONFIG_COPYRIGHT_LABEL）
pub const CONFIG_COPYRIGHT_LABEL: &str = "copyright_label";

/// 版权公司信息配置Key（Java: SysConfigConstants.CONFIG_COPYRIGHT_COMPANY_INFO）
pub const CONFIG_COPYRIGHT_COMPANY_INFO: &str = "copyright_company_name";

/// 版权公司图片配置Key（Java: SysConfigConstants.CONFIG_COPYRIGHT_COMPANY_IMAGE）
pub const CONFIG_COPYRIGHT_COMPANY_IMAGE: &str = "copyright_company_image";

/// CRMEB版权查询接口URL
pub const CRMEB_COPYRIGHT_URL: &str = "https://authorize.crmeb.net/api/auth_cert_query";

// ==================== 版权备案常量（Java: CopyrightConstants） ====================

/// 备案号
pub const COPYRIGHT_ICP_NUMBER: &str = "copyright_icp_number";

/// 备案号链接
pub const COPYRIGHT_ICP_NUMBER_URL: &str = "copyright_icp_number_url";

/// 网安备案
pub const COPYRIGHT_INTERNET_RECORD: &str = "copyright_internet_record";

/// 网安备案链接
pub const COPYRIGHT_INTERNET_RECORD_URL: &str = "copyright_internet_record_url";

// ==================== 项目信息 ====================

/// 项目版本号（从Cargo.toml读取，与Java CrmebConfig.version一致）
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

// ==================== 上传相关常量（Java: SysConfigConstants / UploadConstants） ====================

/// 本地上传URL配置Key
pub const CONFIG_LOCAL_UPLOAD_URL: &str = "localUploadUrl";

/// 七牛云上传URL配置Key
pub const CONFIG_QN_UPLOAD_URL: &str = "qnUploadUrl";

/// 阿里云OSS上传URL配置Key
pub const CONFIG_AL_UPLOAD_URL: &str = "alUploadUrl";

/// 腾讯云COS上传URL配置Key
pub const CONFIG_TX_UPLOAD_URL: &str = "txUploadUrl";

/// 京东云上传URL配置Key
pub const CONFIG_JD_UPLOAD_URL: &str = "jdUploadUrl";

/// 上传文件关键字（Java: UploadConstants.UPLOAD_FILE_KEYWORD）
pub const UPLOAD_FILE_KEYWORD: &str = "crmebimage";

/// 上传后文件关键字（Java: UploadConstants.UPLOAD_AFTER_FILE_KEYWORD）
pub const UPLOAD_AFTER_FILE_KEYWORD: &str = "file";

// ==================== 易联云打印相关常量（Java: YlyConstants） ====================

/// 易联云应用ID
pub const YLY_PRINT_APP_ID: &str = "ylyprint_app_id";
/// 易联云应用密钥
pub const YLY_PRINT_APP_SECRET: &str = "ylyprint_app_secret";
/// 易联云打印机设备码
pub const YLY_PRINT_APP_MACHINE_CODE: &str = "ylyprint_app_machine_code";
/// 易联云打印机设备密钥
pub const YLY_PRINT_APP_MACHINE_MSIGN: &str = "ylyprint_app_machine_msign";
/// 易联云打印开关
pub const YLY_PRINT_STATUS: &str = "ylyprint_status";
/// 易联云自动打印开关
pub const YLY_PRINT_AUTO_STATUS: &str = "ylyprint_auto_status";
/// 易联云Redis Token缓存Key
pub const YLY_REDIS_TOKEN: &str = "yly_token";

// ==================== 易联云API地址 ====================

/// 易联云API基础地址
pub const YLY_BASE_URL: &str = "https://open-api.10ss.net/";
/// OAuth Token接口
pub const YLY_URL_OAUTH: &str = "https://open-api.10ss.net/oauth/oauth";
/// 添加打印机接口
pub const YLY_URL_ADD_PRINTER: &str = "https://open-api.10ss.net/printer/addprinter";
/// 文本打印接口
pub const YLY_URL_PRINT_INDEX: &str = "https://open-api.10ss.net/print/index";

// ==================== 站点配置Key ====================

/// 站点名称配置Key（Java: Constants.CONFIG_KEY_SITE_NAME）
pub const CONFIG_KEY_SITE_NAME: &str = "site_name";

/// 站点URL配置Key（Java: SysConfigConstants.CONFIG_KEY_SITE_URL）
pub const CONFIG_KEY_SITE_URL: &str = "site_url";

// ==================== 组合数据GID（Java: Constants / SysGroupDataConstants） ====================

/// 首页Banner GID（Java: Constants.GROUP_DATA_ID_INDEX_BANNER）
pub const GROUP_DATA_ID_INDEX_BANNER: i32 = 48;

/// 首页金刚区菜单 GID（Java: Constants.GROUP_DATA_ID_INDEX_MENU）
pub const GROUP_DATA_ID_INDEX_MENU: i32 = 67;

/// 首页新闻Banner GID（Java: Constants.GROUP_DATA_ID_INDEX_NEWS_BANNER）
pub const GROUP_DATA_ID_INDEX_NEWS_BANNER: i32 = 68;

/// 首页扩展Banner GID（Java: Constants.GROUP_DATA_ID_INDEX_EX_BANNER）
pub const GROUP_DATA_ID_INDEX_EX_BANNER: i32 = 70;

/// 底部导航 GID（Java: Constants.GROUP_DATA_ID_BOTTOM_NAVIGATION）
pub const GROUP_DATA_ID_BOTTOM_NAVIGATION: i32 = 74;

/// 用户中心菜单 GID（Java: SysGroupDataConstants.GROUP_DATA_ID_USER_CENTER_MENU）
pub const GROUP_DATA_ID_USER_CENTER_MENU: i32 = 54;

/// 用户中心Banner GID（Java: SysGroupDataConstants.GROUP_DATA_ID_USER_CENTER_BANNER）
pub const GROUP_DATA_ID_USER_CENTER_BANNER: i32 = 65;

/// 底部导航是否自定义配置Key（Java: Constants.CONFIG_BOTTOM_NAVIGATION_IS_CUSTOM）
pub const CONFIG_BOTTOM_NAVIGATION_IS_CUSTOM: &str = "bottom_navigation_is_custom";

// ==================== 微信小程序相关常量（Java: WeChatConstants） ====================

/// 微信小程序AppId配置Key（Java: WeChatConstants.WECHAT_MINI_APPID）
pub const WECHAT_MINI_APPID: &str = "routine_appid";

/// 微信小程序AppSecret配置Key（Java: WeChatConstants.WECHAT_MINI_APPSECRET）
pub const WECHAT_MINI_APPSECRET: &str = "routine_appsecret";

/// 微信小程序无限制小程序码URL（Java: WeChatConstants.WECHAT_MINI_QRCODE_UNLIMITED_URL）
pub const WECHAT_MINI_QRCODE_UNLIMITED_URL: &str = "https://api.weixin.qq.com/wxa/getwxacodeunlimit?access_token={}";

/// 微信获取access_token URL（Java: WeChatConstants.WECHAT_ACCESS_TOKEN_URL）
pub const WECHAT_ACCESS_TOKEN_URL: &str = "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}";

/// Redis缓存: 微信小程序access_token（Java: WeChatConstants.REDIS_WECAHT_MINI_ACCESS_TOKEN_KEY）
pub const REDIS_WECHAT_MINI_ACCESS_TOKEN_KEY: &str = "wechat_mini_accessToken";

// ==================== 日期格式 ====================

/// 标准日期时间格式（Java: Constants.DATE_FORMAT）
pub const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
