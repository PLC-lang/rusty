use shadow_rs::{BuildPattern, ShadowBuilder};

fn main() {
    ShadowBuilder::builder()
        .build_pattern(BuildPattern::Custom {
            if_path_changed: vec!["../../.git/HEAD".to_string()],
            if_env_changed: vec![],
        })
        .build()
        .expect("shadow-rs collects the build information");
}
