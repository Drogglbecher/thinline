use analysis::Analysis;
use clang::{Clang, Index};
use error::*;
use project_file::ProjectFile;
use project_file_c::ProjectFileC;
use std::cell::{RefCell, RefMut};

pub static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.h"];

#[derive(Default)]
pub struct AnalysisC {
    file_types: &'static [&'static str],
    project_files: RefCell<Vec<ProjectFileC>>,
}

impl Analysis for AnalysisC {
    fn new() -> Self {
        AnalysisC {
            file_types: C_FILE_EXTENSIONS,
            project_files: RefCell::new(Vec::new()),
        }
    }

    fn file_types(&self) -> &[&str] {
        self.file_types
    }

    fn project_files(&self) -> RefMut<Vec<ProjectFileC>> {
        self.project_files.borrow_mut()
    }

    fn extract_entities(&self) -> Result<()> {
        match Clang::new() {
            Ok(clang) => {
                let index = Index::new(&clang, false, false);
                for project_file in self.project_files().iter() {
                    project_file.filter_for_functions(&index
                        .parser(project_file.path())
                        .parse()?
                        .get_entity())?;
                }
            }
            Err(e) => bail!(e),
        }

        Ok(())
    }
}
