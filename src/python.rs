use analysis::Analysis;
use argument::Argument;
use error::*;
use function::Function;
use language_type::LanguageType;
use python_parser::{ast::*, file_input, make_strspan};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

static PYTHON_FILE_EXTENSIONS: &[&str] = &["*.py"];

#[derive(Default, Clone, Debug)]
pub struct Python {}

impl Python {
    fn extract_statement(
        functions: &mut HashMap<String, Function>,
        class: &Option<String>,
        function: &Option<String>,
        statement: &Statement,
    ) -> Result<()> {
        match statement {
            Statement::Assignment(ent_v, _) => for ent in ent_v.iter() {
                if let Some(func) = function.clone() {
                    if let Expression::String(expr_v) = ent {
                        for expr in expr_v.iter() {
                            let key = class.clone().unwrap_or(String::new()) + "::" + func.as_str();
                            let function = functions
                                .entry(key)
                                .or_insert(Function::new(class.clone(), func.as_str()));
                            function.set_description(&expr.content.to_string_lossy());
                        }
                    }
                }
            },
            Statement::Compound(ent_box) => match Box::leak((*ent_box).clone()) {
                CompoundStatement::Funcdef(expr) => {
                    let key = class.clone().unwrap_or(String::new()) + "::" + expr.name.as_str();

                    if !functions.contains_key(&key) {
                        let mut function: Function = Function::new(class.clone(), expr.name.as_str());
                        let mut arguments: Vec<Argument> = Vec::new();

                        for arg in &expr.parameters.positional_args {
                            arguments.push(Argument::new(arg.0.as_str(), None));
                        }
                        function.set_arguments(&arguments);

                        functions.insert(key, function);
                    }

                    // println!("{:?}", expr);
                    Self::extract_statement(
                        functions,
                        class,
                        &Some(expr.name.clone()),
                        &expr.code[0],
                    )?;
                }
                CompoundStatement::Classdef(expr) => {
                    let classdef = expr.clone();
                    for code in &classdef.code {
                        Self::extract_statement(
                            functions,
                            &Some(classdef.name.clone()),
                            &None,
                            &code,
                        )?;
                    }
                }
                _ => {}
            },
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
                        Self::extract_statement(&mut functions, &None, &None, entity)?;
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
