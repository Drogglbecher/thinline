use error::*;
use glob::glob;
use std::cell::RefMut;
use std::path::PathBuf;

/// A generic trait for source file analysis>
pub trait Anaylsis {
    /// Creates a new Anaylsis instance.
    fn new() -> Self;

    /// Returns the file types which should be analyzed.
    fn file_types(&self) -> &[&str];

    /// Returns the collected project files for anaylsis.
    fn project_files(&self) -> RefMut<Vec<PathBuf>>;

    /// Collect all the sources within the given project dir.
    fn collect_sources(&self, project_dir: &PathBuf, search_dirs: &[&str]) -> Result<()> {
        // Check the given project directory
        if !project_dir.exists() || !project_dir.is_dir() {
            return Err(Error::from(format!(
                "The given project dir '{}' does not exist.",
                project_dir.to_str().ok_or_else(
                    || "Unable to stringify project dir path.",
                )?
            )));
        }

        // Traverse through the files within the specified source directories
        // and store them for analyzing purposes
        for src_dir in search_dirs {
            for ext in self.file_types() {
                for entry in glob(
                    project_dir
                        .join(src_dir)
                        .join("**")
                        .join(ext)
                        .to_str()
                        .unwrap_or("."),
                )?
                {
                    self.project_files().push(entry?);
                }
            }
        }

        Ok(())
    }

    /// Extracts function signatures and comments of thinlines parsed files.
    fn extract_entities(&self) -> Result<()>;
}
