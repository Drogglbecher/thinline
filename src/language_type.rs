use analysis::Analysis;
use clang::{Clang, Entity, EntityKind, Index};
use error::*;
use function::{Argument, Function};
use python_parser::ast::{CompoundStatement, Expression, Statement};
use python_parser::{file_input, make_strspan};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub trait LanguageType: Default {
    fn file_types() -> &'static [&'static str];
    fn extract_functions<T: LanguageType>(analysis: &Analysis<T>) -> Result<()>;
}

static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.h"];
static C_ENTITYKIND_CHECKS: &[EntityKind] = &[EntityKind::FunctionDecl, EntityKind::Method];
static PYTHON_FILE_EXTENSIONS: &[&str] = &["*.py"];

#[derive(Default, Clone, Debug)]
pub struct C {}

impl C {
    fn format_arguments(arguments: &[Entity]) -> Result<Vec<Argument>> {
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

                            let mut function = Function::new(None, function_name);

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

#[derive(Default, Clone, Debug)]
pub struct Python {}

impl Python {
    /// Builds a hash key for local python function hashmap
    fn build_hash_key(class: Option<&str>, function: &str) -> String {
        [class.unwrap_or(""), function].join("::")
    }

    fn extract_statement(
        functions: &mut HashMap<String, Function>,
        class: Option<&str>,
        function: Option<&str>,
        statement: &Statement,
    ) -> Result<()> {
        match statement {

            // Statement is a function documentation
            Statement::Assignment(ent_v, _) => {
                for ent in ent_v.iter() {
                    if let Some(func) = function {
                        if let Expression::String(expr_v) = ent {
                            for expr in expr_v.iter() {
                                let function =
                                    functions
                                        .entry(Self::build_hash_key(class, func))
                                        .or_insert(Function::new(class.map(str::to_string), func));
                                function.set_description(&expr.content.to_string_lossy());
                            }
                        }
                    }
                }
            }
            Statement::Compound(ent_box) => {
                match Box::leak((*ent_box).clone()) {
                    // Statement is a function definition
                    CompoundStatement::Funcdef(expr) => {
                        let key = Self::build_hash_key(class, expr.name.as_str());

                        // Function is not already in local function hashmap?
                        if !functions.contains_key(&key) {
                            let mut function: Function =
                                Function::new(class.map(str::to_string), expr.name.as_str());

                            // Split arguments and add them to the function
                            let mut arguments: Vec<Argument> = Vec::new();
                            for arg in &expr.parameters.positional_args {
                                arguments.push(Argument::new(arg.0.as_str(), None));
                            }
                            function.set_arguments(&arguments);

                            // Add the function to the local function hashmap
                            functions.insert(key, function);
                        }

                        // Recursive call to extraction when the function contains a documentation part
                        if expr.code.get(0).is_some() {
                            Self::extract_statement(
                                functions,
                                class,
                                Some(expr.name.as_str()),
                                &expr.code[0],
                            )?;
                        }
                    }

                    // Statement is a class definition
                    CompoundStatement::Classdef(expr) => {
                        for code in &expr.code {
                            Self::extract_statement(
                                functions,
                                Some(expr.name.as_str()),
                                None,
                                &code,
                            )?;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
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

            let mut functions: HashMap<String, Function> = HashMap::new();

            match file_input(make_strspan(content.as_str())) {
                Ok(ast) => {
                    for entity in ast.1.iter() {
                        Self::extract_statement(&mut functions, None, None, entity)?;
                    }
                }
                Err(_) => bail!("Unable to create python AST."),
            }

            for (_, function) in functions.into_iter() {
                println!("{:?}", function);
                project_file.add_function(function);
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
