use serde::Deserialize;
use std::fs;

use crate::error::AppError;

/// 应用配置
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// 调试开关
    #[serde(default)]
    pub debug: bool,
    /// 服务配置
    pub server: ServerConfig,
    /// 日志配置
    #[serde(default)]
    pub log: LogConfig,
    /// 管理员配置
    pub admin: AdminConfig,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 业务配置
    pub app: BizConfig,
}

/// 日志配置
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    /// 日志等级，如 trace/debug/info/warn/error
    #[serde(default = "default_log_level")]
    pub level: String,
    /// 是否启用彩色日志
    #[serde(default = "default_log_color")]
    pub color: bool,
    /// 日志前缀
    #[serde(default = "default_log_prefix")]
    pub prefix: String,
}

/// 服务配置
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// 监听端口
    pub http_port: u16,
    /// 监听地址
    pub host: String,
}

/// 管理员配置
#[derive(Debug, Clone, Deserialize)]
pub struct AdminConfig {
    /// 管理员用户名
    pub username: String,
    /// 管理员密码
    pub password: String,
    /// JWT密钥
    pub jwt_secret: String,
    /// JWT过期小时数
    pub jwt_expire_hours: i64,
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库连接URL
    pub url: String,
}

/// 业务配置
#[derive(Debug, Clone, Deserialize)]
pub struct BizConfig {
    /// 是否用正文生成摘要
    #[serde(default = "default_use_content_as_abs")]
    pub use_content_as_abs: bool,
    /// 正文摘要最大截取长度
    #[serde(default = "default_max_content_length")]
    pub max_content_length: usize,
    /// 自定义空摘要内容
    #[serde(default = "default_custom_empty_abs")]
    pub custom_empty_abs: String,
}

fn default_use_content_as_abs() -> bool {
    true
}

fn default_max_content_length() -> usize {
    120
}

fn default_custom_empty_abs() -> String {
    "<code>Sorry</code>该文章暂无概述".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_color() -> bool {
    true
}

fn default_log_prefix() -> String {
    "RustBlog".to_string()
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            color: default_log_color(),
            prefix: default_log_prefix(),
        }
    }
}

impl AppConfig {
    /// 从TOML文件加载配置
    pub fn load_from_file(path: &str) -> Result<Self, AppError> {
        let content = fs::read_to_string(path)?;
        let cfg = toml::from_str::<Self>(&content)?;
        Ok(cfg)
    }
}
