pub mod convention;

#[macro_export]
macro_rules! filtered_assert_snapshot {
    // Case for normal snapshot (no inline expected output)
    ($value:expr) => {{
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r#"target datalayout = ".*""#, r#"target datalayout = "[filtered]""#);
        settings.add_filter(r#"target triple = ".*""#, r#"target triple = "[filtered]""#);
        settings.add_filter(r#"align:? \d{1,2}"#, r#"align [filtered]"#);
        settings.bind(|| insta::assert_snapshot!($value))
    }};

    // Case for inline snapshot: expression @literal
    ($value:expr, @$snapshot:literal) => {{
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r#"target datalayout = ".*""#, r#"target datalayout = "[filtered]""#);
        settings.add_filter(r#"target triple = ".*""#, r#"target triple = "[filtered]""#);
        settings.add_filter(r#"align:? \d{1,2}"#, r#"align [filtered]"#);
        settings.bind(|| insta::assert_snapshot!($value, @$snapshot));
    }};
}

#[macro_export]
macro_rules! filtered_assert_snapshot_with_alginments {
    // Case for normal snapshot (no inline expected output)
    ($value:expr) => {{
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r#"target datalayout = ".*""#, r#"target datalayout = "[filtered]""#);
        settings.add_filter(r#"target triple = ".*""#, r#"target triple = "[filtered]""#);
        settings.bind(|| insta::assert_snapshot!($value))
    }};

    // Case for inline snapshot: expression @literal
    ($value:expr, @$snapshot:literal) => {{
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r#"target datalayout = ".*""#, r#"target datalayout = "[filtered]""#);
        settings.add_filter(r#"target triple = ".*""#, r#"target triple = "[filtered]""#);
        settings.bind(|| insta::assert_snapshot!($value, @$snapshot));
    }};
}
