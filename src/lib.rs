#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang;
#[macro_use]
extern crate error_chain;
extern crate glob;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate slog_envlogger;
extern crate walkdir;

pub mod analysis;
pub mod analysis_c;
pub mod error;
pub mod synthesis;

use analysis_c::AnaylsisC;
use error::*;
use std::collections::HashMap;
use std::env;
use std::fs::{self, canonicalize, File};
use std::io::Write;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::process::Command;
use synthesis::*;
use walkdir::WalkDir;

static C_HEADER_EXTENSIONS: &[&str] = &["h", "hpp"];

/// The data which holds parsed function signatures for a file.
#[derive(Debug)]
struct ThinlineData {
    file: PathBuf,
    namespaces: Vec<String>,
    functions: Vec<String>,
}

impl ThinlineData {
    /// Creates a new instance for `ThinlineData`.
    fn new<P: Into<PathBuf>>(file: P) -> Self {
        ThinlineData {
            file: file.into(),
            namespaces: Vec::new(),
            functions: Vec::new(),
        }
    }
}

#[derive(Default)]
/// Global structure representing the Thinline lib.
pub struct Thinline {
    /// The structure holding the analysis data.
    pub analysis: AnaylsisC,

    /// The structure holding the synthesized testdata.
    pub synthesis: Synthesis,

    /// The tree structure of the parsed functions.
    data: Vec<ThinlineData>,
}

impl Thinline {
    /// Creates an instance of the lib containing Thinlines functionality.
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyzes the project which should be tested.
    pub fn analyze_project<P: Into<PathBuf>>(&mut self, project_dir: P) -> Result<()> {
        self.analysis = AnaylsisC::new();
        self.analysis.collect_sources(&project_dir.into(), vec!["src", "include"])?;
        self.analysis.extract_entities()
    }
}
