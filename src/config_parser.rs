use failure::{err_msg, Fallible};
use run_script::ScriptOptions;
use std::{env, fs::read_to_string};
use yaml_rust::{Yaml, YamlLoader};

////////////////////////////////////////////////////////////////////////////////

trait Conversion {
    /// Consumes an Option<Vec<&'a str>> and returns it's elements as String.
    fn to_string_vec(self) -> Vec<String>;
}

impl<'a> Conversion for Option<Vec<&'a str>> {
    fn to_string_vec(self) -> Vec<String> {
        if let Some(vec) = self {
            vec.iter().map(|f| String::from(*f)).collect()
        } else {
            vec![]
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

trait ValueParser {
    /// Reads a bool at the given config path.
    fn get_bool(&self, keys: &[&str], default: bool) -> bool;

    /// Reads a string at the given config path.
    fn get_str(&self, keys: &[&str]) -> Option<&str>;

    /// Reads a string vector at the given config path.
    fn get_str_vec(&self, keys: &[&str]) -> Option<Vec<&str>>;
}

impl ValueParser for Yaml {
    fn get_bool(&self, keys: &[&str], default: bool) -> bool {
        let mut yml_obj = self;

        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return default;
            }

            yml_obj = &yml_obj[*key];
            if let Some(yml_bool) = yml_obj.as_bool() {
                return yml_bool;
            }

            if yml_obj[0].is_badvalue() {
                continue;
            }
        }

        default
    }

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

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct BuildScript {
    /// Indicator whether the output is logged within thinline output or not.
    pub log: bool,

    /// Windows build steps.
    pub windows: Vec<String>,

    /// Linux build steps.
    pub linux: Vec<String>,
}

impl BuildScript {
    /// Executes the build script depending on the target OS.
    ///
    /// On Windows the script is executed with `cmd`, on linux with `sh`.
    /// All build steps are concatenated with ` && `.
    /// Given build steps are executed at the target project directory,
    /// not at thinline directory. The yaml param `log` at `build_script`
    /// section indicates whether the build outpus is print within the
    /// thinline output (true) or the child process (false).
    pub fn run(&self, dir: &str) -> Fallible<()> {
        info!("Building target");

        // Save current working dir
        let current_working_dir = env::current_dir()?;

        // Change to project dir
        env::set_current_dir(&dir)?;

        // Build script options
        let mut options = ScriptOptions::new();

        // Set the runner.
        options.runner = if cfg!(target_os = "windows") {
            Some(String::from("cmd"))
        } else {
            Some(String::from("sh"))
        };

        // Print it to the parent process output.
        options.capture_output = !self.log;

        // Format the commands depending on OS.
        let cmd = if cfg!(target_os = "windows") {
            format!(r#"{}"#, self.windows.join(" && "))
        } else {
            format!(r#"{}"#, self.linux.join(" && "))
        };

        // Run the script
        let (code, _, _) = run_script::run(cmd.as_str(), &vec![], &options)?;

        // Change back to thinline dir
        env::set_current_dir(&current_working_dir)?;

        // Check the return code
        if code > 0 {
            return Err(err_msg(format!(
                "Executing build steps returned error code {}.",
                code
            )));
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
/// The parsed project parameters.
pub struct ProjectParameters {
    /// The language of the source project (e.g. c, cpp or python)
    pub language: String,

    /// Test environment which should be used (e.g. google test)
    pub test_env: String,

    /// The build steps which should be executed when the build-option is set.
    pub build_script: BuildScript,

    /// Paths to libraries which should be linked.
    pub lib_paths: Vec<String>,

    /// The source directories to extract the test data.
    pub source_dirs: Vec<String>,

    /// The include directories necessary to build the tests.
    pub include_dirs: Vec<String>,
}

impl ProjectParameters {
    /// Parses the project parameters from the given yaml file.
    pub fn parse(yml: &str) -> Fallible<ProjectParameters> {
        if let Ok(yml_params) = YamlLoader::load_from_str(read_to_string(yml)?.as_str()) {
            if let Some(yml_param) = yml_params.get(0) {
                let mut params = ProjectParameters::default();

                params.language =
                    String::from(yml_param.get_str(&["language"]).ok_or_else(|| {
                        err_msg("Unable to get parameters for mandatory 'language'.")
                    })?);
                params.test_env =
                    String::from(yml_param.get_str(&["test_env"]).ok_or_else(|| {
                        err_msg("Unable to get parameters for mandatory 'test_env'.")
                    })?);

                params.source_dirs = yml_param.get_str_vec(&["analysis_dirs"]).to_string_vec();
                params.include_dirs = yml_param.get_str_vec(&["include_dirs"]).to_string_vec();
                params.build_script.log = yml_param.get_bool(&["build_script", "log"], true);
                params.build_script.linux = yml_param
                    .get_str_vec(&["build_script", "linux"])
                    .to_string_vec();
                params.build_script.windows = yml_param
                    .get_str_vec(&["build_script", "windows"])
                    .to_string_vec();
                params.lib_paths = yml_param.get_str_vec(&["libs"]).to_string_vec();

                return Ok(params);
            }
        }

        Err(format_err!("Unable to parse project parameters."))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod value_parser {
    use super::ValueParser;
    use std::fs::read_to_string;
    use std::path::Path;
    use yaml_rust::YamlLoader;

    #[test]
    fn parse_yaml_config_bool_succeed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        let build_log = yml_param.unwrap().get_bool(&["build_script", "log"], false);
        assert_eq!(build_log, true);
    }

    #[test]
    fn parse_yaml_config_bool_failed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        let build_log = yml_param
            .unwrap()
            .get_bool(&["build_script", "none_existing"], true);
        assert_eq!(build_log, true);
    }

    #[test]
    fn parse_yaml_config_str_succeed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        let test_env = yml_param.unwrap().get_str(&["test_env"]);
        assert_eq!(test_env, Some("ctest"));

        let language = yml_param.unwrap().get_str(&["language"]);
        assert_eq!(language, Some("c"));
    }

    #[test]
    fn parse_yaml_config_str_failed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        {
            let test_env = yml_param.unwrap().get_str(&["none_existing"]);
            assert!(test_env.is_none());
        }

        {
            let test_env = yml_param.unwrap().get_str(&[]);
            assert!(test_env.is_none());
        }

        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config4.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        {
            let test_env = yml_param.unwrap().get_str(&["test_env"]);
            assert!(test_env.is_none());
        }
    }

    #[test]
    fn parse_yaml_config_str_vec_failed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        {
            let test_env = yml_param.unwrap().get_str_vec(&["include_dirs"]);
            assert_eq!(test_env, Some(vec!["include", "src"]));
        }

        {
            let test_env = yml_param.unwrap().get_str_vec(&["none_existing"]);
            assert!(test_env.is_none());
        }

        {
            let test_env = yml_param.unwrap().get_str_vec(&[]);
            assert!(test_env.is_none());
        }
    }
}
