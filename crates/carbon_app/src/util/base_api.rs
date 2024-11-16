macro_rules! get_base_api_env {
    () => {{
        let version_json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../packages/config/version.json"
        ));

        let version: serde_json::Value = serde_json::from_str(version_json).unwrap();
        if version["channel"] == "snapshot" || cfg!(debug_assertions) {
            env!("TEST_BASE_API").to_string()
        } else {
            env!("BASE_API").to_string()
        }
    }};
}

pub(crate) use get_base_api_env;
