use analysis::Anaylsis;
use clang::{Clang, Entity, EntityKind, Index};
use error::*;
use function::Function;
use std::cell::{RefCell, RefMut};
use std::path::PathBuf;

static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.cc", "*.cpp", "*.h", "*.hpp"];
static C_ENTITYKIND_CHECKS: &[EntityKind] = &[EntityKind::FunctionDecl, EntityKind::Method];

macro_rules! unwrap_or_continue {
    ($e:expr) => {
        match $e {
            Some(e) => e,
            None => continue,
        }
    };
}

#[derive(Default)]
pub struct AnaylsisC {
    file_types: &'static [&'static str],
    project_files: RefCell<Vec<PathBuf>>,
}

impl Anaylsis for AnaylsisC {
    fn new() -> Self {
        AnaylsisC {
            file_types: C_FILE_EXTENSIONS,
            project_files: RefCell::new(Vec::new()),
        }
    }

    fn file_types(&self) -> &[&str] {
        self.file_types
    }

    fn project_files(&self) -> RefMut<Vec<PathBuf>> {
        self.project_files.borrow_mut()
    }

    fn extract_entities(&self) -> Result<()> {
        match Clang::new() {
            Ok(clang) => {
                let index = Index::new(&clang, false, false);
                for project_file in self.project_files().iter() {
                    self.filter_for_functions(
                        &index.parser(&project_file).parse()?.get_entity(),
                    )?;
                }
            }
            Err(e) => bail!(e),
        }

        Ok(())
    }
}

impl AnaylsisC {
    fn filter_for_functions(&self, entity: &Entity) -> Result<()> {

        // Iterate through the child entities of the current entity
        for child in entity.get_children() {
            let child_kind = child.get_kind();

            // Search for methods and constructors outside the system headers
            if !child.is_in_system_header() && C_ENTITYKIND_CHECKS.contains(&child_kind) {
                let function_type = unwrap_or_continue!(child.get_type()).get_display_name();
                let function_name = unwrap_or_continue!(child.get_name());
                let function_desc = unwrap_or_continue!(child.get_comment());
                let function_args = unwrap_or_continue!(child.get_arguments());

                println!(
                    "Create child '{}' with type '{}'",
                    function_name,
                    function_type
                );

                let mut function = Function::new(
                    child.get_semantic_parent().and_then(|sp| sp.get_name()),
                    function_name,
                    Function::format_type(function_type.as_str())?,
                    Function::format_arguments(&function_args)?,
                    Function::format_description(function_desc.as_str())?,
                );
            }
        }

        Ok(())
    }
}
