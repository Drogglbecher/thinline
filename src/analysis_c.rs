use analysis::*;
use clang::{Clang, Entity, EntityKind, Index};
use error::*;
use std::path::PathBuf;

macro_rules! unwrap_or_continue {
    ($e:expr) => {
        match $e {
            Some(e) => e,
            None => continue,
        }
    };
}

static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.cc", "*.cpp", "*.h", "*.hpp"];

#[derive(Default)]
pub struct AnaylsisC {
    file_types: &'static [&'static str],
    project_files: Vec<PathBuf>,
}

impl Anaylsis for AnaylsisC {
    fn new() -> Self {
        AnaylsisC {
            file_types: C_FILE_EXTENSIONS,
            project_files: Vec::new(),
        }
    }

    fn file_types(&self) -> &[&'static str] {
        self.file_types
    }

    fn project_files(&self) -> &mut [PathBuf] {
        &mut self.project_files
    }

    fn extract_entities(&mut self) -> Result<()> {
        match Clang::new() {
            Ok(clang) => {
                let index = Index::new(&clang, false, false);
                for i in 0..self.project_files.len() {
                    // Makes slicing below pseudo-secure ^^
                    if self.project_files.get(i).is_none() {
                        continue;
                    }
                    let tu = index.parser(&self.project_files[i]).parse()?;

                    // Extract all FunctionDecl Entities
                    self.filter_for_functions(i, &tu.get_entity())?;
                }
            }
            Err(e) => bail!(e),
        }

        Ok(())
    }
}

impl AnaylsisC {
    fn filter_for_functions(&mut self, idx: usize, entity: &Entity) -> Result<()> {
        // Iterate through the child entities of the current entity
        for child in entity.get_children() {
            let child_kind = child.get_kind();

            // Search for methods and constructors outside the system headers
            if !child.is_in_system_header()
                && (child_kind == EntityKind::FunctionDecl
                    || child_kind == EntityKind::Method
                    || child_kind == EntityKind::Constructor
                    || child_kind == EntityKind::Destructor)
            {
                let fct_type = unwrap_or_continue!(child.get_type()).get_display_name();
                let fct_name = unwrap_or_continue!(child.get_name());
                let fct_desc = unwrap_or_continue!(child.get_comment());

                println!(
                    "Function '{}' with type '{}' and comment '\n{}'",
                    fct_name, fct_type, fct_desc
                );

            // A namesapce or class is found -> Check their child entities by recursive call
            } else if (child_kind == EntityKind::Namespace) {
                if child_kind == EntityKind::Namespace {
                    let ns_name = child.get_name();
                    if let Some(test_ns) = ns_name {
                        //  println!("Namespace {}", test_ns.as_str());
                    }
                }
                self.filter_for_functions(idx, &child)?;
            }
        }

        Ok(())
    }
}
