use error::*;
use filter::Filter;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::{Yaml, YamlLoader};

static MAX_ARGUMENT_NO: u8 = 20;

trait GetValues {
    fn get_str(&self, keys: &[&str]) -> Option<String>;
    fn get_vec(&self, keys: &[&str]) -> Option<Vec<String>>;
    fn get_str_raw_or_from_file(
        &self,
        config_path: &str,
        file_keys: &[&str],
        keys: &[&str],
    ) -> Option<String>;
}

impl GetValues for Yaml {
    fn get_str(&self, keys: &[&str]) -> Option<String> {
        let mut yml_obj = self;
        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return None;
            }
            yml_obj = &yml_obj[*key];
            if let Some(yml_str) = yml_obj.as_str() {
                return Some(String::from(yml_str));
            }
            if yml_obj[0].is_badvalue() {
                continue;
            }
            if let Some(yml_str) = yml_obj[0].as_str() {
                return Some(String::from(yml_str));
            }
        }
        None
    }

    fn get_vec(&self, keys: &[&str]) -> Option<Vec<String>> {
        let mut yml_obj = self;
        let mut yml_vec: Vec<String> = Vec::new();
        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return None;
            }
            yml_obj = &yml_obj[*key];
            let mut i = 0;
            while !yml_obj[i].is_badvalue() {
                if let Some(yml_str) = yml_obj[i].as_str() {
                    yml_vec.push(String::from(yml_str));
                }
                i += 1;
            }
        }
        if !yml_vec.is_empty() {
            return Some(yml_vec);
        }
        None
    }

    fn get_str_raw_or_from_file(
        &self,
        config_path: &str,
        file_keys: &[&str],
        keys: &[&str],
    ) -> Option<String> {

        let file = String::from(self.get_str(file_keys).unwrap_or_default());
        if file.is_empty() {
            if let Some(raw_str) = self.get_str(keys) {
                return Some(String::from(raw_str));
            }
        }

        let mut full_path = Path::new(config_path).join("environment");
        for path_part in file.split('/').collect::<Vec<&str>>() {
            full_path = full_path.join(path_part);
        }
        if let Some(full_path_str) = full_path.to_str() {
            if let Ok(mut file_instance) = File::open(full_path_str) {
                let mut file_str = String::new();
                if file_instance.read_to_string(&mut file_str).is_ok() {
                    return Some(file_str);
                }
            }
        }

        None
    }
}

#[derive(Default)]
pub struct BuildScript {
    pub windows: Option<Vec<String>>,
    pub linux: Option<Vec<String>>,
}

#[derive(Default)]
pub struct CFeatures {
    placeholder: String,
}

#[derive(Default)]
pub struct CppFeatures {
    pub namespace_filter: Filter,
    pub class_filter: Filter,
}

#[derive(Default)]
pub struct LanguageFeatures {
    pub c_features: CFeatures,
    pub cpp_features: CppFeatures,
}

#[derive(Default)]
pub struct ProjectParameters {
    pub build_script: BuildScript,
    pub lib_paths: Vec<String>,
    pub src_dirs: Option<Vec<String>>,
    pub include_dirs: Option<Vec<String>>,
    pub test_env: String,
    pub user_prefix: Option<String>,
    pub user_suffix: Option<String>,
    pub language_features: LanguageFeatures,
}

impl ProjectParameters {
    fn parse_language_features(&mut self, yml_param: &Yaml) -> Result<()> {
        self.language_features.cpp_features.namespace_filter = Filter::new(
            &yml_param
                .get_vec(&["language_features", "cpp", "blacklist_namespaces"])
                .unwrap_or_default(),
            &yml_param
                .get_vec(&["language_features", "cpp", "whitelist_namespaces"])
                .unwrap_or_default(),
        );

        self.language_features.cpp_features.class_filter = Filter::new(
            &yml_param
                .get_vec(&["language_features", "cpp", "blacklist_classes"])
                .unwrap_or_default(),
            &yml_param
                .get_vec(&["language_features", "cpp", "whitelist_classes"])
                .unwrap_or_default(),
        );
        Ok(())
    }

    pub fn parse(yml: &str) -> Result<ProjectParameters> {
        let mut file_buf = String::new();
        let mut file_instance = File::open(yml)?;
        file_instance.read_to_string(&mut file_buf)?;
        let mut params = ProjectParameters::default();
        if let Ok(yml_params) = YamlLoader::load_from_str(file_buf.as_str()) {
            if let Some(yml_param) = yml_params.get(0) {
                params.test_env = yml_param.get_str(&["test_env"]).ok_or_else(
                    || "Unable to get parameters for 'test_env'.",
                )?;
                params.src_dirs = yml_param.get_vec(&["src_dirs"]);
                params.include_dirs = yml_param.get_vec(&["include_dirs"]);
                params.build_script.linux = yml_param.get_vec(&["build_script", "linux"]);
                params.build_script.windows = yml_param.get_vec(&["build_script", "windows"]);
                params.lib_paths = yml_param.get_vec(&["libs"]).ok_or_else(
                    || "Unable to get parameters for 'libs'.",
                )?;
                params.user_prefix = yml_param.get_str(&["tlg", "prefix"]);
                params.user_suffix = yml_param.get_str(&["tlg", "suffix"]);
                params.parse_language_features(yml_param)?;
                return Ok(params);
            }
        }
        Err(Error::from("Unable to parse project parameters."))
    }
}

#[derive(Default, Clone)]
pub struct TestFunctionSignature {
    pub stub: String,
    pub inline_sig: String,
    pub arg_no: u8,
}

impl TestFunctionSignature {
    pub fn parse(yml: &str, env: &str, sig_key: &str) -> Result<TestFunctionSignature> {
        let mut sig = TestFunctionSignature::default();
        if let Ok(yml_params) = YamlLoader::load_from_str(yml) {
            if let Some(yml_param) = yml_params.get(0) {
                sig.stub = String::from(yml_param
                    .get_str(&[env, "test_signatures", sig_key, "stub"])
                    .ok_or_else(|| {
                        format!(
                            "Unable to get test function signature stub '{}' for environment '{}'.",
                            sig_key,
                            env
                        )
                    })?);
                sig.inline_sig = String::from(
                    yml_param.get_str(&[env, "test_signatures", sig_key, "inline"])
                             .ok_or_else(|| format!("Unable to get test function signature separator for '{}' in environment '{}'.",
                                                    sig_key,
                                                    env))?);
                for i in 0..MAX_ARGUMENT_NO {
                    if sig.stub.contains(format!("//#ARG_{}#/", i).as_str()) {
                        sig.arg_no = i + 1;
                    }
                }
                return Ok(sig);
            }
        }
        Err(Error::from(format!(
            "Unable to parse test function signature '{}' for environment '{}'",
            sig_key,
            env
        )))
    }
}

#[derive(Default)]
pub struct EnvironmentParameters {
    pub cfg_path: String,
    pub yml: String,
    pub env: String,
    pub file_stub: String,
    pub class_sig: String,
    pub fct_sig: String,
    pub output_format: String,
    pub test_sig: HashMap<String, TestFunctionSignature>,
}

impl EnvironmentParameters {
    pub fn get_test_fct_sig(&self, test_cmd: &str) -> Result<TestFunctionSignature> {
        let cst_sig = self.test_sig
            .get(test_cmd.replace(':', "").as_str())
            .ok_or_else(|| {
                format!(
                    "Unable to access key '{}' in test function hashmap.",
                    test_cmd
                )
            })?;
        Ok((*cst_sig).to_owned())
    }

    pub fn parse(cfg_path: &str, yml: &str, env: &str) -> Result<EnvironmentParameters> {
        let mut file_buf = String::new();
        let mut file_instance = File::open(yml)?;
        file_instance.read_to_string(&mut file_buf)?;

        let mut params = EnvironmentParameters::default();
        params.env = String::from(env);
        params.yml = file_buf;

        if let Ok(yml_params) = YamlLoader::load_from_str(params.yml.as_str()) {
            if let Some(yml_param) = yml_params.get(0) {
                params.output_format =
                    String::from(yml_param.get_str(&[env, "output_format"]).ok_or_else(|| {
                        format!("Unable to get output format for environment '{}'.", env)
                    })?);
                params.file_stub = String::from(yml_param
                    .get_str_raw_or_from_file(
                        cfg_path,
                        &[env, "file", "file"],
                        &[env, "file", "raw"],
                    )
                    .ok_or_else(|| {
                        format!("Unable to get file stub for environment '{}'.", env)
                    })?);
                params.class_sig = String::from(yml_param
                    .get_str_raw_or_from_file(
                        cfg_path,
                        &[env, "class", "file"],
                        &[env, "class", "raw"],
                    )
                    .ok_or_else(|| {
                        format!(
                            "Unable to get class signature stub for environment '{}'.",
                            env
                        )
                    })?);
                params.fct_sig = String::from(yml_param
                    .get_str_raw_or_from_file(
                        cfg_path,
                        &[env, "function", "file"],
                        &[env, "function", "raw"],
                    )
                    .ok_or_else(|| {
                        format!(
                            "Unable to get function signature stub for environment '{}'.",
                            env
                        )
                    })?);
                //debug!("class_sig: \n{}", params.class_sig);
                //debug!("fct_sig: \n{}", params.fct_sig);
                return Ok(params);
            }
        }
        Err(Error::from(format!(
            "Unable to parse environment parameters for '{}'",
            env
        )))
    }

    pub fn add_test_fct_sig(&mut self, sig_key: &str) -> Result<()> {
        self.test_sig.insert(
            String::from(sig_key),
            TestFunctionSignature::parse(self.yml.as_str(), self.env.as_str(), sig_key)?,
        );
        Ok(())
    }
}
