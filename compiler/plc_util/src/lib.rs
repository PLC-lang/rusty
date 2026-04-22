pub mod convention;

#[doc(hidden)]
pub fn escape_regex_literal(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// FIXME(#1692-followup): move snapshot filter macros to a dedicated test-support crate
/// (e.g. `snapshot_utils`) so they are not hosted in production utility code.
#[doc(hidden)]
#[macro_export]
macro_rules! __plc_add_common_snapshot_filters {
    ($settings:ident) => {{
        $settings.add_filter(r#"target datalayout = ".*""#, r#"target datalayout = "[filtered]""#);
        $settings.add_filter(r#"target triple = ".*""#, r#"target triple = "[filtered]""#);

        if let Ok(cwd) = std::env::current_dir() {
            let cwd = cwd.to_string_lossy().to_string();
            for path in [cwd.clone(), cwd.replace('\\', "/"), format!(r"\\?\{}", cwd)].into_iter() {
                $settings.add_filter(&$crate::escape_regex_literal(&path), "[cwd]");
            }
        }
    }};
}

#[macro_export]
macro_rules! filtered_assert_snapshot {
    // Case for normal snapshot (no inline expected output)
    ($value:expr) => {{
        let mut settings = insta::Settings::clone_current();
        $crate::__plc_add_common_snapshot_filters!(settings);
        settings.add_filter(r#"align:? \d{1,2}"#, r#"align [filtered]"#);
        settings.bind(|| insta::assert_snapshot!($value))
    }};

    // Case for inline snapshot: expression @literal
    ($value:expr, @$snapshot:literal) => {{
        let mut settings = insta::Settings::clone_current();
        $crate::__plc_add_common_snapshot_filters!(settings);
        settings.add_filter(r#"align:? \d{1,2}"#, r#"align [filtered]"#);
        settings.bind(|| insta::assert_snapshot!($value, @$snapshot));
    }};
}

#[macro_export]
macro_rules! filtered_assert_snapshot_with_alignments {
    // Case for normal snapshot (no inline expected output)
    ($value:expr) => {{
        let mut settings = insta::Settings::clone_current();
        $crate::__plc_add_common_snapshot_filters!(settings);
        settings.bind(|| insta::assert_snapshot!($value))
    }};

    // Case for inline snapshot: expression @literal
    ($value:expr, @$snapshot:literal) => {{
        let mut settings = insta::Settings::clone_current();
        $crate::__plc_add_common_snapshot_filters!(settings);
        settings.bind(|| insta::assert_snapshot!($value, @$snapshot));
    }};
}
