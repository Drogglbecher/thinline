use analysis::Analysis;
use argument::Argument;
use clang::{Clang, Entity, EntityKind, Index};
use error::*;
use function::Function;
use language_type::LanguageType;

static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.h"];
static C_ENTITYKIND_CHECKS: &[EntityKind] = &[EntityKind::FunctionDecl, EntityKind::Method];

#[derive(Default, Clone, Debug)]
pub struct C {}

impl C {
    fn format_arguments(arguments: &[Entity]) -> Result<Vec<Argument>> {
        let mut args = Vec::new();

        for argument in arguments {
            args.push(Argument::new(
                argument.get_display_name().unwrap_or(String::new()),
                Some(argument
                    .get_type()
                    .ok_or_else(|| "Argument type can not be parsed from signature.")?
                    .get_display_name()),
            ));
        }

        Ok(args)
    }
}

impl LanguageType for C {
    fn file_types() -> &'static [&'static str] {
        C_FILE_EXTENSIONS
    }

    fn extract_functions<C: LanguageType>(analysis: &Analysis<C>) -> Result<()> {
        match Clang::new() {
            Ok(clang) => {
                let index = Index::new(&clang, false, false);
                for project_file in analysis.project_files().iter() {
                    let parsed_path = &index.parser(project_file.path()).parse()?;
                    let entity = parsed_path.get_entity();
                    // Iterate through the child entities of the current entity
                    for child in entity.get_children() {
                        let child_kind = child.get_kind();

                        // Search for methods and constructors outside the system headers
                        if !child.is_in_system_header() && C_ENTITYKIND_CHECKS.contains(&child_kind)
                        {
                            let function_type =
                                unwrap_or_return!(child.get_type(), continue).get_display_name();
                            let function_name = unwrap_or_return!(child.get_name(), continue);
                            let function_args = unwrap_or_return!(child.get_arguments(), continue);
                            let function_desc = unwrap_or_return!(child.get_comment(), continue);

                            println!(
                                "Create child '{}' with type '{}'",
                                function_name, function_type
                            );

                            let mut function = Function::new(
                                child.get_semantic_parent().and_then(|sp| sp.get_name()),
                                function_name,
                            );

                            function.set_format_type(function_type.as_str());
                            function.set_arguments(&Self::format_arguments(&function_args)?);
                            function.set_description(function_desc.as_str());

                            project_file.add_function(function);
                        }
                    }
                }
            }
            Err(e) => bail!(e),
        }
        Ok(())
    }
}

#[test]
fn test_new() {
    // Given
    let analysis: Analysis<C> = Analysis::new();

    // Then
    assert_eq!(analysis.file_types(), C_FILE_EXTENSIONS);
    assert_eq!(analysis.project_files().len(), 0);
}
