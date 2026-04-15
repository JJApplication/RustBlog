use crate::config::BizConfig;

/// 按配置把正文或默认文本转换为摘要。
///
/// 与Go版本逻辑保持一致：
/// 1. `abs` 为空且 `use_content_as_abs=true` 时，优先截取 `content`；
/// 2. `abs` 为空且未开启正文摘要时，返回 `custom_empty_abs`；
/// 3. 其他情况返回 `abs` 原值。
pub fn content_to_abs(abs: &str, content: &str, app: &BizConfig) -> String {
    if app.use_content_as_abs && abs.is_empty() {
        if content.chars().count() >= app.max_content_length {
            return content.chars().take(app.max_content_length).collect();
        }
        return content.to_string();
    }
    if abs.is_empty() {
        return app.custom_empty_abs.clone();
    }
    abs.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> BizConfig {
        BizConfig {
            use_content_as_abs: true,
            max_content_length: 5,
            custom_empty_abs: "<code>Sorry</code>该文章暂无概述".to_string(),
        }
    }

    #[test]
    fn should_cut_content_when_abs_empty_and_enabled() {
        let got = content_to_abs("", "abcdef", &cfg());
        assert_eq!(got, "abcde");
    }

    #[test]
    fn should_return_custom_when_abs_empty_and_disabled() {
        let mut c = cfg();
        c.use_content_as_abs = false;
        let got = content_to_abs("", "abcdef", &c);
        assert_eq!(got, "<code>Sorry</code>该文章暂无概述");
    }

    #[test]
    fn should_keep_abs_when_abs_not_empty() {
        let got = content_to_abs("已有摘要", "abcdef", &cfg());
        assert_eq!(got, "已有摘要");
    }
}
