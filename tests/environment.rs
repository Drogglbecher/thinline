extern crate thinlinelib;

use std::env::*;
use std::fs;
use std::path::Path;
use thinlinelib::environment::Environment;

#[cfg(not(target_os = "windows"))]
static THINLINE_CONFIG_PATH: &str = ".config/thinline";
#[cfg(target_os = "windows")]
static THINLINE_CONFIG_PATH: &str = "thinline";

#[test]
fn test_create_config() {

    assert!(home_dir().is_some());
    let tl_config_path_raw = home_dir().unwrap().join(Path::new(THINLINE_CONFIG_PATH));
    assert!(tl_config_path_raw.to_str().is_some());
    let env_config_path = tl_config_path_raw.to_str().unwrap();

    if Path::new(env_config_path).exists() {
        assert!(fs::remove_dir_all(env_config_path).is_ok());
    }

    let env_config_dir_raw = Path::new(env_config_path).join("environment");
    assert!(env_config_dir_raw.to_str().is_some());
    let env_config_dir = env_config_dir_raw.to_str().unwrap();

    if Path::new(env_config_dir).exists() {
        assert!(fs::remove_dir_all(env_config_dir).is_ok());
    }

    let mut env = Environment::default();

    assert!(env.create_config_directory().is_ok());
    assert!(env.create_environment_config().is_ok());
    assert!(env.create_google_test_stubs().is_ok());
}
