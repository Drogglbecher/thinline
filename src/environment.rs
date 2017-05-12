use error::*;
use std::env::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

static DEFAULT_ENV_YML: &str = include_str!("../stubs/environment/env_stubs.yml");

static DEFAULT_ENV_GOOGLE_TEST_CLASS_STUB: &str =
    include_str!("../stubs/environment/google_test/class.stub");
static DEFAULT_ENV_GOOGLE_TEST_FILE_STUB: &str =
    include_str!("../stubs/environment/google_test/file.stub");
static DEFAULT_ENV_GOOGLE_TEST_FCT_STUB: &str =
    include_str!("../stubs/environment/google_test/fct.stub");

#[cfg(not(target_os = "windows"))]
static THINLINE_CONFIG_PATH: &str = ".config/thinline";
#[cfg(target_os = "windows")]
static THINLINE_CONFIG_PATH: &str = "thinline";

#[derive(Default)]
pub struct Environment {
    pub env_config_path: String,
    pub env_config_file: String,
}

impl Environment {
    pub fn create_config_directory(&mut self) -> Result<()> {
        self.env_config_path = String::from(
            home_dir()
                .ok_or_else(|| "Unable to get home dir.")?
                .join(Path::new(THINLINE_CONFIG_PATH))
                .to_str()
                .ok_or_else(|| "Unable to stringify thinline config path.")?,
        );

        if !Path::new(self.env_config_path.as_str()).exists() {
            info!("Create thinline config folder '{}'.", self.env_config_path);
            fs::create_dir_all(self.env_config_path.as_str())?;
        }

        Ok(())
    }

    pub fn create_environment_config(&mut self) -> Result<()> {
        self.env_config_file = String::from(
            Path::new(self.env_config_path.as_str())
                .join("environment")
                .join("env_stubs.yml")
                .to_str()
                .ok_or_else(|| "Unable to stringify env_stubs.yml path.")?,
        );

        if !Path::new(self.env_config_file.as_str()).exists() {
            info!("Create '{}'.", self.env_config_file);
            fs::create_dir_all(Path::new(self.env_config_path.as_str()).join("environment"))?;
            File::create(&self.env_config_file)?.write_all(
                DEFAULT_ENV_YML
                    .as_bytes(),
            )?;
        }

        Ok(())
    }

    pub fn create_google_test_stubs(&mut self) -> Result<()> {
        let google_test_env_path = String::from(Path::new(self.env_config_path.as_str())
            .join("environment")
            .join("google_test")
            .to_str()
            .ok_or_else(|| "Unable to stringify google test path.")?);

        if !Path::new(google_test_env_path.as_str()).exists() {
            info!("Create '{}'.", google_test_env_path);
            fs::create_dir_all(
                Path::new(self.env_config_path.as_str())
                    .join("environment")
                    .join("google_test"),
            )?;
            File::create(
                Path::new(self.env_config_path.as_str())
                    .join("environment")
                    .join("google_test")
                    .join("class.stub"),
            )?
                .write_all(DEFAULT_ENV_GOOGLE_TEST_CLASS_STUB.as_bytes())?;
            File::create(
                Path::new(self.env_config_path.as_str())
                    .join("environment")
                    .join("google_test")
                    .join("file.stub"),
            )?
                .write_all(DEFAULT_ENV_GOOGLE_TEST_FILE_STUB.as_bytes())?;
            File::create(
                Path::new(self.env_config_path.as_str())
                    .join("environment")
                    .join("google_test")
                    .join("fct.stub"),
            )?
                .write_all(DEFAULT_ENV_GOOGLE_TEST_FCT_STUB.as_bytes())?;
        }

        Ok(())
    }
}
