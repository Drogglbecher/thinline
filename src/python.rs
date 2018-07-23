use analysis::Analysis;
use error::*;
use function::Function;
use language_type::LanguageType;
use python_parser::{file_input, make_strspan, ast::*};
use std::fs::File;
use std::io::Read;

static PYTHON_FILE_EXTENSIONS: &[&str] = &["*.py"];

#[derive(Default, Clone, Debug)]
pub struct Python {}

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
                        match entity {
                            Statement::Assignment(ent_v, _) => {
                                for ent in ent_v.iter() {
                                    match ent {
                                        Expression::String(expr_v) => {
                                            for expr in expr_v.iter() {
                                                println!("{:?}", expr.content);
                                            }
                                        }
                                        _ => continue,
                                    }
                                }
                            }
                            Statement::Compound(e) => println!("{:?}", e),
                            _ => continue,
                        }
                    }
                },
                Err(_) => bail!("Unable to create python AST."),
            }
        }

        Ok(())
    }
}
