// Re-export core profile types from plc_diagnostics so downstream users
// can import from either location.
pub use plc_diagnostics::profiles::{BehaviorFlags, CompatibilityProfile, PROFILE_CODESYS, PROFILE_STANDARD};

#[cfg(test)]
mod tests {
    use plc_diagnostics::diagnostics::diagnostics_registry::DiagnosticsConfiguration;

    use super::*;

    #[test]
    fn named_profile_codesys() {
        let profile = CompatibilityProfile::from_name_or_path("codesys").unwrap();
        assert_eq!(profile.name.as_deref(), Some("codesys"));
    }

    #[test]
    fn named_profile_standard() {
        let profile = CompatibilityProfile::from_name_or_path("standard").unwrap();
        assert_eq!(profile.name.as_deref(), Some("standard"));
    }

    #[test]
    fn default_profile_is_codesys() {
        let profile = CompatibilityProfile::default();
        assert_eq!(profile.name.as_deref(), Some("codesys"));
    }

    #[test]
    fn load_profile_from_json_string() {
        let json = r#"{
            "name": "custom",
            "behaviors": {},
            "diagnostics": {}
        }"#;
        let profile = CompatibilityProfile::from_json(json).unwrap();
        assert_eq!(profile.name.as_deref(), Some("custom"));
    }

    #[test]
    fn load_profile_from_toml_string() {
        let toml_str = r#"
            name = "custom-toml"

            [behaviors]

            [diagnostics]
        "#;
        let profile = CompatibilityProfile::from_toml(toml_str).unwrap();
        assert_eq!(profile.name.as_deref(), Some("custom-toml"));
    }

    #[test]
    fn partial_profile_defaults_missing_sections() {
        let json = r#"{ "name": "minimal" }"#;
        let profile = CompatibilityProfile::from_json(json).unwrap();
        assert_eq!(profile.name.as_deref(), Some("minimal"));
    }

    #[test]
    fn empty_json_object_is_valid() {
        let json = "{}";
        let profile = CompatibilityProfile::from_json(json).unwrap();
        assert!(profile.name.is_none());
    }

    #[test]
    fn unknown_behavior_flags_are_skipped() {
        let json = r#"{
            "name": "future-profile",
            "behaviors": {
                "some_future_flag": true,
                "another_future_flag": 42
            }
        }"#;
        let profile = CompatibilityProfile::from_json(json).unwrap();
        assert_eq!(profile.name.as_deref(), Some("future-profile"));
    }

    #[test]
    fn nonexistent_file_errors() {
        let result = CompatibilityProfile::from_name_or_path("/nonexistent/profile.json");
        assert!(result.is_err());
    }

    #[test]
    fn roundtrip_json() {
        let profile = CompatibilityProfile::codesys();
        let json = profile.to_json().unwrap();
        let roundtripped = CompatibilityProfile::from_json(&json).unwrap();
        assert_eq!(roundtripped.name, profile.name);
    }

    #[test]
    fn roundtrip_toml() {
        let profile = CompatibilityProfile::standard();
        let toml_str = profile.to_toml().unwrap();
        let roundtripped = CompatibilityProfile::from_toml(&toml_str).unwrap();
        assert_eq!(roundtripped.name, profile.name);
    }

    #[test]
    fn from_diagnostics_configuration_preserves_config() {
        let config = DiagnosticsConfiguration::default();
        let profile = CompatibilityProfile::from_diagnostics_configuration(config);
        assert!(profile.name.is_none());
    }

    #[test]
    fn load_profile_from_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-profile.json");
        let json = r#"{ "name": "file-test", "behaviors": {} }"#;
        std::fs::write(&path, json).unwrap();

        let profile = CompatibilityProfile::from_file(&path).unwrap();
        assert_eq!(profile.name.as_deref(), Some("file-test"));
    }

    #[test]
    fn load_profile_from_toml_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test-profile.toml");
        let toml_content = "name = \"toml-file-test\"\n[behaviors]\n";
        std::fs::write(&path, toml_content).unwrap();

        let profile = CompatibilityProfile::from_file(&path).unwrap();
        assert_eq!(profile.name.as_deref(), Some("toml-file-test"));
    }
}
