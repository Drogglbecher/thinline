#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang;
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
use failure::{err_msg, Fallible};
use language_type::LanguageType;
use project_parameters::ProjectParameters;
use std::path::PathBuf;
use synthesis::*;

static DEFAULT_ENV_YML: &str = include_str!("../stubs/environment/env_stubs.yml");

#[derive(Default, Debug)]
/// Global structure representing the Thinline lib.
pub struct Thinline<T>
where
    T: LanguageType,
{
    /// The parsed project parameters.
    project_parameters: ProjectParameters,

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
    pub fn new() -> Self {
        Self {
            project_parameters: ProjectParameters::new(),
            analysis: Analysis::new(),
            synthesis: Synthesis::new(),
        }
    }

    /// Starts the analysis of the target project.
    pub fn analyze<P: Into<PathBuf>>(
        &mut self,
        source_dir: P,
        thinline_cfg: &str,
        build: bool,
    ) -> Fallible<()> {
        let source_dir_path = source_dir.into();

        // Parses the project config
        self.parse_project_config(&source_dir_path, thinline_cfg)?;

        // Analyze the project at the given source directory
        self.analyze_project(&source_dir_path)?;

        // Builds target project when build flag is set
        if build {
            self.project_parameters.build_script.run(&source_dir_path)?;
        }

        Ok(())
    }

    /// Parses configuration from the given config yaml.
    fn parse_project_config<P: Into<PathBuf>>(
        &mut self,
        project_dir: P,
        config_name: &str,
    ) -> Fallible<()> {
        let project_config = project_dir.into().join(config_name);

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

        Ok(())
    }

    /// Analyzes the project which should be tested.
    fn analyze_project<P: Into<PathBuf>>(&self, project_path: P) -> Fallible<()> {
        let project_path_p = project_path.into();

        if let Some(project_path_s) = project_path_p.to_str() {
            info!("Starting project analysis at '{}'", project_path_s);
        }

        if project_path_p.is_dir() {
            // Project path is a directory, thus it is neccessay to traverse to the project
            // and collect all the sources.
            self.analysis
                .collect_sources(&project_path_p, &self.project_parameters.source_dirs)?;
        }

        if project_path_p.is_file() {
            if let Some(ext) = project_path_p.extension() {
                // Project path is a file and has the right extension.
                if T::file_types().contains(
                    &ext.to_str()
                        .ok_or_else(|| err_msg("Unable to stringify file extension."))?,
                ) {
                    // Push it to the project file vector for analyzing purposes.
                    self.analysis
                        .project_files_mut()
                        .push(ProjectFile::new(&project_path_p));
                }
            }
        }

        self.analysis.extract_entities()?;

        Ok(())
    }

    pub fn synthesize(&mut self) -> Fallible<()> {
        self.synthesis.parse_stubs(DEFAULT_ENV_YML)?;

        Ok(())
    }
}
