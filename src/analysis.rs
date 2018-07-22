use error::*;
use glob::glob;
use language_type::LanguageType;
use project_file::ProjectFile;
use std::cell::{Ref, RefCell, RefMut};
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Analysis<T>
where
    T: LanguageType<T>,
{
    file_types: &'static [&'static str],
    project_files: RefCell<Vec<ProjectFile<T>>>,
}

impl<T> Analysis<T>
where
    T: LanguageType<T>,
{
    /// Creates a new Analysis instance.
    pub fn new() -> Self {
        Analysis {
            file_types: T::file_types(),
            project_files: RefCell::new(Vec::new()),
        }
    }

    /// Returns the file types which should be analyzed.
    pub fn file_types(&self) -> &[&str] {
        self.file_types
    }

    /// Returns a reference to the collected project files for anaylsis.
    pub fn project_files(&self) -> Ref<Vec<ProjectFile<T>>> {
        self.project_files.borrow()
    }

    /// Returns a mutable reference to the collected project files for anaylsis.
    pub fn project_files_mut(&self) -> RefMut<Vec<ProjectFile<T>>> {
        self.project_files.borrow_mut()
    }

    /// Collect all the sources within the given project dir.
    pub fn collect_sources(&self, project_dir: &PathBuf, search_dirs: &[&str]) -> Result<()> {
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
                    self.project_files_mut().push(ProjectFile::new(entry?));
                }
            }
        }

        Ok(())
    }

    /// Extracts function signatures and comments of thinlines parsed files.
    pub fn extract_entities(&self) -> Result<()> {
        T::extract_functions(&self)
    }
}
