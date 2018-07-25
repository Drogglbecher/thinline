use analysis::Analysis;
use error::*;
use function::Function;
use language_type::LanguageType;
use python_parser::{ast::*, file_input, make_strspan};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

static PYTHON_FILE_EXTENSIONS: &[&str] = &["*.py"];

#[derive(Default, Clone, Debug)]
pub struct Python {}

impl Python {
    fn extract_test_function(pystr: &str) -> Option<&str> {
        if let Ok(re) = Regex::new(r"#TL_TESTCASE\((.*?)::.*?") {
            if let Some(caps) = re.captures(pystr) {
                return Some(caps.get(1)?.as_str());
            }
        }

        None
    }

    fn extract_statement(
        functions: &mut HashMap<String, Function>,
        class: Option<String>,
        statement: &Statement,
    ) {
        match statement {
            Statement::Assignment(ent_v, _) => for ent in ent_v.iter() {
                if let Expression::String(expr_v) = ent {
                    for expr in expr_v.iter() {
                        if let Some(pystr) =
                            Self::extract_test_function(&expr.content.to_string_lossy())
                        {
                            let key = class.clone().unwrap_or(String::new())
                                + "::"
                                + String::from(pystr).as_str();
                            if !functions.contains_key(&key) {
                                functions.insert(
                                    key,
                                    Function::new(
                                        class.clone(),
                                        pystr,
                                        None,
                                        Vec::new(),
                                        Vec::new(),
                                    ),
                                );
                            }
                        }
                    }
                }
            },
            Statement::Compound(ent_box) => match Box::leak((*ent_box).clone()) {
                CompoundStatement::Funcdef(expr) => println!("{:?}", expr),
                CompoundStatement::Classdef(expr) => {
                    println!("{:?}", expr.name);
                    let classdef = expr.clone();
                    for code in &classdef.code {
                        Self::extract_statement(functions, Some(classdef.name.clone()), &code);
                    }
                }
                _ => {}
            },
            _ => {}
        }
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

            match file_input(make_strspan(content.as_str())) {
                Ok(ast) => {
                    for entity in ast.1.iter() {
                        let mut functions: HashMap<String, Function> = HashMap::new();
                        Self::extract_statement(&mut functions, None, entity);
                    }
                }
                Err(_) => bail!("Unable to create python AST."),
            }
        }

        Ok(())
    }
}
