use failure::{err_msg, Fallible};
use run_script::{self, ScriptOptions};
use std::{env, fs::read_to_string, path::PathBuf};
use value_parser::{Conversion, ValueParser};
use yaml_rust::YamlLoader;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
/// The parsed build actions depending on the used OS.
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
    pub fn run<P: Into<PathBuf>>(&self, dir: P) -> Fallible<()> {
        info!("Building target");

        // Save current working dir
        let current_working_dir = env::current_dir()?;

        // Change to project dir
        env::set_current_dir(&dir.into())?;

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
    /// Creates a new ProjectParameters instance.
    pub fn new() -> Self {
        Self::default()
    }

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
