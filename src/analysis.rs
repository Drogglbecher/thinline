use error::*;
use glob::glob;
use std::path::PathBuf;

/// A generic trait for source file analysis>
pub trait Anaylsis {
    fn new() -> Self;

    fn file_types(&self) -> &[&'static str];
    fn project_files(&self) -> &mut [PathBuf];

    /// Collect all the sources within the given project dir.
    fn collect_sources(&mut self, project_dir: &PathBuf, search_dirs: &[String]) -> Result<()> {
        // Check the given project directory
        if !project_dir.exists() || !project_dir.is_dir() {
            return Err(Error::from(format!(
                "The given project dir '{}' does not exist.",
                project_dir
                    .to_str()
                    .ok_or_else(|| "Unable to stringify project dir path.")?
            )));
        }

        // Traverse through the files within the specified source directories
        // and store them for analyzing purposes
        for ext in search_dirs {
            for src_dir in self.file_types() {
                let c_src_paths = project_dir.join(src_dir).join("**").join(ext);
                for entry in glob(c_src_paths.to_str().unwrap_or("."))? {
                    self.project_files().push(entry?);
                }
            }
        }

        Ok(())
    }

    /// Extracts function signatures and comments of thinlines parsed files.
    fn extract_entities(&mut self) -> Result<()>;
}
