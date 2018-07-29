use analysis::Analysis;
use argument::Argument;
use error::*;
use function::Function;
use language_type::LanguageType;
use python_parser::ast::{CompoundStatement, Expression, Statement};
use python_parser::{file_input, make_strspan};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

static PYTHON_FILE_EXTENSIONS: &[&str] = &["*.py"];

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
                project_file.add_function(function);
            }
        }

        Ok(())
    }
}
