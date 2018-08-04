use analysis::Analysis;
use clang;
use error::*;
use function::{Argument, Entity, EntityType, Function};
use python_parser::ast::{CompoundStatement, Expression, Statement};
use python_parser::{file_input, make_strspan};
use std::fs::File;
use std::io::Read;

pub trait LanguageType: Default {
    fn file_types() -> &'static [&'static str];
    fn extract_functions<T: LanguageType>(analysis: &Analysis<T>) -> Result<()>;
}

static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.h"];
static C_ENTITYKIND_CHECKS: &[clang::EntityKind] =
    &[clang::EntityKind::FunctionDecl, clang::EntityKind::Method];
static PYTHON_FILE_EXTENSIONS: &[&str] = &["*.py"];

#[derive(Default, Clone, Debug)]
pub struct C {}

impl C {
    fn format_arguments(arguments: &[clang::Entity]) -> Result<Vec<Argument>> {
        let mut args = Vec::new();

        for argument in arguments {
            args.push(Argument::new(
                argument.get_display_name().unwrap_or(String::new()),
                Some(
                    argument
                        .get_type()
                        .ok_or_else(|| "Argument type can not be parsed from signature.")?
                        .get_display_name(),
                ),
            ));
        }

        Ok(args)
    }

    fn analyse_clang_entity(entity: &clang::Entity) -> Result<Option<Function>> {
        let entity_kind = entity.get_kind();

        // Search for methods and constructors outside the system headers
        if !entity.is_in_system_header() && C_ENTITYKIND_CHECKS.contains(&entity_kind) {
            let function_type = unwrap_or_return!(entity.get_type(), Ok(None)).get_display_name();
            let function_name = unwrap_or_return!(entity.get_name(), Ok(None));
            let function_args = unwrap_or_return!(entity.get_arguments(), Ok(None));
            let function_desc = unwrap_or_return!(entity.get_comment(), Ok(None));

            let mut function = Function::new(function_name);

            function.set_return_type(function_type.as_str())?;
            function.set_arguments(&Self::format_arguments(&function_args)?);
            function.set_description(function_desc.as_str());

            return Ok(Some(function));
        }

        Ok(None)
    }
}

impl LanguageType for C {
    fn file_types() -> &'static [&'static str] {
        C_FILE_EXTENSIONS
    }

    fn extract_functions<C: LanguageType>(analysis: &Analysis<C>) -> Result<()> {
        match clang::Clang::new() {
            Ok(clang) => {
                let clang_index = clang::Index::new(&clang, false, false);
                for project_file in analysis.project_files().iter() {
                    if let EntityType::Index(mut index) = EntityType::Index(Entity::new("")) {
                        let parsed_path = &clang_index.parser(project_file.path()).parse()?;
                        let clang_entity = parsed_path.get_entity();
                        // Iterate through the child entities of the current entity
                        for child in clang_entity.get_children() {
                            if let Ok(Some(function)) = Self::analyse_clang_entity(&child) {
                                index.add_function(function);
                            }
                        }
                        project_file.add_entity(index);
                    }
                }
            }
            Err(e) => bail!(e),
        }

        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
pub struct Python {}

impl Python {
    fn extract_function_doc(function: &mut Function, statement: &Statement) {
        if let Statement::Assignment(ent_v, _) = statement {
            for ent in ent_v.iter() {
                if let Expression::String(expr_v) = ent {
                    for expr in expr_v.iter() {
                        function.set_description(&expr.content.to_string_lossy());
                    }
                }
            }
        }
    }

    fn analyse_statement(entity: &mut Entity, statement: &Statement) -> Result<()> {
        if let Statement::Compound(ent_box) = statement {
            match Box::leak((*ent_box).clone()) {
                // Statement is a statement definition
                CompoundStatement::Funcdef(expr) => {
                    let mut function: Function = Function::new(expr.name.as_str());

                    // Split arguments and add them to the function
                    let mut arguments: Vec<Argument> = Vec::new();
                    for arg in &expr.parameters.positional_args {
                        arguments.push(Argument::new(arg.0.as_str(), None));
                    }
                    function.set_arguments(&arguments);

                    if let Some(mut function_inst) = entity.add_function(function) {
                        Self::extract_function_doc(&mut function_inst, &expr.code[0]);
                    }
                }

                // Statement is a class definition
                CompoundStatement::Classdef(expr) => {
                    if let Some(EntityType::Class(ref mut class_entity)) =
                        entity.add_entity(EntityType::Class(Entity::new(expr.name.as_str())))
                    {
                        for code in &expr.code {
                            Self::analyse_statement(class_entity, &code)?;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl LanguageType for Python {
    fn file_types() -> &'static [&'static str] {
        PYTHON_FILE_EXTENSIONS
    }

    fn extract_functions<Python: LanguageType>(analysis: &Analysis<Python>) -> Result<()> {
        for project_file in analysis.project_files().iter() {
            // Parse file to string
            let mut file = File::open(project_file.path())?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            if let EntityType::Index(mut index) = EntityType::Index(Entity::new("")) {
                match file_input(make_strspan(content.as_str())) {
                    Ok(ast) => {
                        for entity in ast.1.iter() {
                            Self::analyse_statement(&mut index, entity)?;
                        }
                    }
                    Err(_) => bail!("Unable to create python AST."),
                }
                println!("{:?}", index);
                project_file.add_entity(index);
            }
        }

        Ok(())
    }
}

#[test]
fn new_c() {
    // Given
    let analysis: Analysis<C> = Analysis::new();

    // Then
    assert_eq!(analysis.file_types(), C_FILE_EXTENSIONS);
    assert_eq!(analysis.project_files().len(), 0);
}

#[test]
fn new_python() {
    // Given
    let analysis: Analysis<Python> = Analysis::new();

    // Then
    assert_eq!(analysis.file_types(), PYTHON_FILE_EXTENSIONS);
    assert_eq!(analysis.project_files().len(), 0);
}
