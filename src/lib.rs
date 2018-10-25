#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang;
extern crate directories;
#[macro_use]
extern crate failure;
extern crate glob;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate python_parser;
extern crate regex;
extern crate run_script;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate slog_envlogger;
extern crate snapshot;
extern crate walkdir;
extern crate yaml_rust;

pub mod analysis;
pub mod entity;
pub mod language_type;
pub mod project_parameters;
pub mod stubs;
pub mod synthesis;
pub mod value_parser;

use analysis::{Analysis, ProjectFile};
use directories::BaseDirs;
use failure::{err_msg, Fallible};
use language_type::LanguageType;
use project_parameters::ProjectParameters;
use std::path::PathBuf;
use synthesis::*;

////////////////////////////////////////////////////////////////////////////////

static DEFAULT_ENV_YML: &str = "./stubs/environment/env_stubs.yml";

#[derive(Default, Debug)]
/// Global structure representing the Thinline lib.
pub struct Thinline<T>
where
    T: LanguageType,
{
    /// The target project directory.
    pub project_dir: PathBuf,

    /// The parsed project parameters.
    pub project_parameters: ProjectParameters,

    /// The structure holding the analysis_c data.
    analysis: Analysis<T>,

    /// The structure holding the synthesized testdata.
    synthesis: Synthesis<T>,
}

impl<T> Thinline<T>
where
    T: LanguageType,
{
    /// Creates an instance of the lib containing Thinlines functionality.
    pub fn new<P: Into<PathBuf>>(project_dir: P) -> Self {
        Self {
            project_dir: project_dir.into(),
            project_parameters: ProjectParameters::new(),
            analysis: Analysis::new(),
            synthesis: Synthesis::new(),
        }
    }

    /// Starts the analysis of the target project.
    pub fn analyze(&mut self, thinline_cfg: &str, build: bool) -> Fallible<()> {
        // Parses the project config
        self.parse_project_config(thinline_cfg)?;

        // Analyze the project at the given source directory
        self.analyze_project()?;

        // Builds target project when build flag is set
        if build {
            self.project_parameters.build_script.run(&self.project_dir)?;
        }

        Ok(())
    }

    /// Traverses upwared through the `seed_dir`s parent directories until
    /// `.thinline.yml` configuration exists in dir or home directory is reached.
    fn find_thinline_config_dir<'a>(seed_dir: &'a PathBuf, config_name: &str) -> Option<PathBuf> {
        if let Some(base_dirs) = BaseDirs::new() {
            let home_dir = base_dirs.home_dir();
            let mut project_config = seed_dir.clone();

            loop {
                if project_config == home_dir || project_config.join(config_name).is_file() {
                    trace!("Config dir: {:?}", project_config);
                    break;
                }

                if let Some(parent) = project_config.clone().as_path().parent() {
                    project_config = parent.to_path_buf();
                } else {
                    return None;
                }
            }

            return Some(project_config.join(config_name));
        }

        None
    }

    /// Parses configuration from the given config yaml.
    fn parse_project_config(&mut self, config_name: &str) -> Fallible<()> {
        if let Some(project_config) = Self::find_thinline_config_dir(&self.project_dir, config_name)
        {
            if !project_config.exists() || !project_config.is_file() {
                return Err(format_err!(
                    "The given project config file '{}' does not exist or is a directory.",
                    project_config
                        .to_str()
                        .ok_or_else(|| err_msg("Unable to stringify project config file."))?
                ));
            }

            self.project_parameters = ProjectParameters::parse(
                project_config
                    .to_str()
                    .ok_or_else(|| err_msg("Unable to stringify project config file."))?,
            )?;

            debug!("{:#?}", self.project_parameters);
        } else {
            return Err(format_err!("Unable to get project dir parent for"));
        }

        Ok(())
    }

    /// Analyzes the project which should be tested.
    fn analyze_project(&self) -> Fallible<()> {
        if let Some(project_path_s) = self.project_dir.to_str() {
            info!("Starting project analysis at '{}'", project_path_s);
        }

        if self.project_dir.is_dir() {
            // Project path is a directory, thus it is neccessay to traverse to the project
            // and collect all the sources.
            self.analysis
                .collect_sources(&self.project_dir, &self.project_parameters.source_dirs)?;
        }

        if self.project_dir.is_file() {
            if let Some(ext) = self.project_dir.extension() {
                // Project path is a file and has the right extension.
                if T::file_types().contains(
                    &ext.to_str()
                        .ok_or_else(|| err_msg("Unable to stringify file extension."))?,
                ) {
                    // Push it to the project file vector for analyzing purposes.
                    self.analysis
                        .project_files_mut()
                        .push(ProjectFile::new(&self.project_dir));
                }
            }
        }

        self.analysis.extract_entities()?;

        Ok(())
    }

    pub fn synthesize<P: Into<PathBuf>>(&mut self, base_path: P) -> Fallible<()> {
        self.synthesis.parse_stubs(
            DEFAULT_ENV_YML,
            self.project_parameters.test_env.as_str(),
            &base_path.into(),
        )?;

        Ok(())
    }
}
