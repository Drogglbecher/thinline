use clang::{Entity, EntityKind};
use error::*;
use function::Function;
use project_file::{ProjectFile, ProjectFileT};
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::path::PathBuf;

static C_ENTITYKIND_CHECKS: &[EntityKind] = &[EntityKind::FunctionDecl, EntityKind::Method];

/// Reprensents a parsed project fike.

impl<T> ProjectFileT<T> for ProjectFile<T>
where
    T: Default,
{
    fn new<S: Into<PathBuf>>(path: S) -> Self {
        ProjectFile {
            pf_type: PhantomData,
            path: path.into(),
            functions: RefCell::new(Vec::new()),
        }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn functions(&self) -> Ref<Vec<Function>> {
        self.functions.borrow()
    }

    fn functions_mut(&self) -> RefMut<Vec<Function>> {
        self.functions.borrow_mut()
    }

    fn extract_functions(&self, entity: &Entity) -> Result<()> {
        // Iterate through the child entities of the current entity
        for child in entity.get_children() {
            let child_kind = child.get_kind();

            // Search for methods and constructors outside the system headers
            if !child.is_in_system_header() && C_ENTITYKIND_CHECKS.contains(&child_kind) {
                let function_type =
                    unwrap_or_return!(child.get_type(), continue).get_display_name();
                let function_name = unwrap_or_return!(child.get_name(), continue);
                let function_desc = unwrap_or_return!(child.get_comment(), continue);
                let function_args = unwrap_or_return!(child.get_arguments(), continue);

                println!(
                    "Create child '{}' with type '{}'",
                    function_name, function_type
                );

                let function = Function::new(
                    child.get_semantic_parent().and_then(|sp| sp.get_name()),
                    function_name,
                    Function::format_type(function_type.as_str())?,
                    Function::format_arguments(&function_args)?,
                    Function::format_description(function_desc.as_str())?,
                );

                self.add_function(function);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_extract_functions {
    use super::*;
    use c::*;
    use clang::{Clang, Index};
    use std::path::PathBuf;

    #[test]
    fn should_succeed() {
        // Given
        let c_test_src_path: ProjectFile<c::C> = ProjectFile::new(
            PathBuf::from("tests")
                .join("testdata")
                .join("c_sources")
                .join("test1.c"),
        );

        match Clang::new() {
            Ok(clang) => {
                let index = Index::new(&clang, false, false);
                match &index.parser(c_test_src_path.path()).parse() {
                    Ok(index_parsed) => assert!(
                        c_test_src_path
                            .extract_functions(&index_parsed.get_entity())
                            .is_ok()
                    ),
                    Err(_) => assert!(false),
                }

                assert_eq!(c_test_src_path.functions().len(), 4);
                assert!(c_test_src_path.functions()[0].class.is_some());
                assert_eq!(
                    c_test_src_path.functions()[0].clone().class.unwrap(),
                    "tests/testdata/c_sources/test1.c"
                );
                assert_eq!(c_test_src_path.functions()[0].name, "test_int_no1");
                assert_eq!(c_test_src_path.functions()[0].ftype, "int");
                assert_eq!(c_test_src_path.functions()[0].arguments.len(), 2);
                assert_eq!(c_test_src_path.functions()[0].description.len(), 6);
            }
            Err(_) => assert!(false),
        }
    }
}
