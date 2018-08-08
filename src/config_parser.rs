use error::*;
use std::fs::read_to_string;
use yaml_rust::{Yaml, YamlLoader};

trait Conversion {
    fn to_string_vec(self) -> Option<Vec<String>>;
}

impl<'a> Conversion for Option<Vec<&'a str>> {
    fn to_string_vec(self) -> Option<Vec<String>> {
        self.map(|e| e.iter().map(|f| String::from(*f)).collect())
    }
}

trait ValueParser {
    fn get_str(&self, keys: &[&str]) -> Option<&str>;
    fn get_str_vec(&self, keys: &[&str]) -> Option<Vec<&str>>;
}

impl ValueParser for Yaml {
    fn get_str(&self, keys: &[&str]) -> Option<&str> {
        let mut yml_obj = self;

        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return None;
            }

            yml_obj = &yml_obj[*key];
            if let Some(yml_str) = yml_obj.as_str() {
                return Some(yml_str);
            }

            if yml_obj[0].is_badvalue() {
                continue;
            }

            if let Some(yml_str) = yml_obj[0].as_str() {
                return Some(yml_str);
            }
        }

        None
    }

    fn get_str_vec(&self, keys: &[&str]) -> Option<Vec<&str>> {
        let mut yml_obj = self;
        let mut yml_vec: Vec<&str> = Vec::new();

        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return None;
            }

            yml_obj = &yml_obj[*key];
            let mut i = 0;
            while !yml_obj[i].is_badvalue() {
                if let Some(yml_str) = yml_obj[i].as_str() {
                    yml_vec.push(yml_str);
                }
                i += 1;
            }
        }

        if !yml_vec.is_empty() {
            return Some(yml_vec);
        }

        None
    }
}

#[derive(Default, Debug)]
pub struct BuildScript {
    pub windows: Option<Vec<String>>,
    pub linux: Option<Vec<String>>,
}

#[derive(Default, Debug)]
pub struct ProjectParameters {
    pub test_env: String,
    pub build_script: Option<BuildScript>,
    pub lib_paths: Option<Vec<String>>,
    pub source_dirs: Option<Vec<String>>,
    pub include_dirs: Option<Vec<String>>,
}

impl ProjectParameters {
    /// Parses the project parameters from the given yaml file.
    pub fn parse(yml: &str) -> Result<ProjectParameters> {
        if let Ok(yml_params) = YamlLoader::load_from_str(read_to_string(yml)?.as_str()) {

            let mut params = ProjectParameters::default();
            if let Some(yml_param) = yml_params.get(0) {
                params.test_env = String::from(
                    yml_param
                        .get_str(&["test_env"])
                        .ok_or_else(|| "Unable to get parameters for 'test_env'.")?,
                );

                params.source_dirs = yml_param.get_str_vec(&["src_dirs"]).to_string_vec();
                params.include_dirs = yml_param.get_str_vec(&["include_dirs"]).to_string_vec();
                // params.build_script.linux = yml_param.get_str_vec(&["build_script", "linux"]).to_string_vec();
                // params.build_script.windows = yml_param.get_str_vec(&["build_script", "windows"]).to_string_vec();
                params.lib_paths = yml_param.get_str_vec(&["libs"]).to_string_vec();

                return Ok(params);
            }
        }

        Err(Error::from("Unable to parse project parameters."))
    }
}
