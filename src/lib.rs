#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang;
#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate lazy_static;
#[macro_use]
extern crate mac;
extern crate python_parser;
extern crate regex;
extern crate slog_envlogger;
extern crate walkdir;

pub mod analysis;
pub mod error;
pub mod function;
pub mod language_type;
pub mod synthesis;

use analysis::{Analysis, ProjectFile};
use error::*;
use language_type::LanguageType;
use std::path::PathBuf;
use synthesis::*;

#[derive(Default, Debug)]
/// Global structure representing the Thinline lib.
pub struct Thinline<T>
where
    T: LanguageType,
{
    /// The structure holding the analysis_c data.
    pub analysis: Analysis<T>,

    /// The structure holding the synthesized testdata.
    pub synthesis: Synthesis,
}

impl<T> Thinline<T>
where
    T: LanguageType,
{
    /// Creates an instance of the lib containing Thinlines functionality.
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyzes the project which should be tested.
    pub fn analyze_project<P: Into<PathBuf>>(&mut self, project_path: P) -> Result<()> {
        self.analysis = Analysis::new();
        let project_path_p = project_path.into();

        if project_path_p.is_dir() {
            // Project path is a directory, thus it is neccessay to traverse to the project
            // and collect all the sources.
            self.analysis
                .collect_sources(&project_path_p, &[".", "include"])?;
        }

        if project_path_p.is_file() {
            if let Some(ext) = project_path_p.extension() {
                // Project path is a file and has the right extension.
                if T::file_types().contains(
                    &ext.to_str()
                        .ok_or_else(|| "Unable to stringify file extension.")?,
                ) {
                    // Push it to the project file vectory for analyzing purposes.
                    self.analysis
                        .project_files_mut()
                        .push(ProjectFile::new(&project_path_p));
                }
            }
        }

        self.analysis.extract_entities()?;

        Ok(())
    }
}
