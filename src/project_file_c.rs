use clang::{Entity, EntityKind};
use error::*;
use function::Function;
use project_file::ProjectFile;
use std::cell::{RefCell, RefMut};
use std::path::PathBuf;

static C_ENTITYKIND_CHECKS: &[EntityKind] = &[EntityKind::FunctionDecl, EntityKind::Method];

macro_rules! unwrap_or_continue {
    ($e:expr) => {
        match $e {
            Some(e) => e,
            None => continue,
        }
    };
}

/// Reprensents a parsed project fike.
#[derive(Default)]
pub struct ProjectFileC {
    path: PathBuf,
    functions: RefCell<Vec<Function>>,
}

impl ProjectFile for ProjectFileC {
    fn new<S: Into<PathBuf>>(path: S) -> Self {
        ProjectFileC {
            path: path.into(),
            functions: RefCell::new(Vec::new()),
        }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn functions(&self) -> RefMut<Vec<Function>> {
        self.functions.borrow_mut()
    }

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
