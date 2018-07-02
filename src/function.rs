use argument::Argument;
use clang::Entity;
use error::*;

/// Reprensents a parsed function type.
pub struct Function {
    pub class: Option<String>,
    pub name: String,
    pub ftype: String,
    pub arguments: Vec<Argument>,
    pub doc_str: Vec<String>,
}

impl<'a> Function {
    /// Creates a new `Function` instance.
    pub fn new<S: Into<String>>(
        class: Option<String>,
        name: S,
        ftype: &str,
        arguments: Vec<Argument>,
        doc_str: Vec<String>,
    ) -> Self {
        Function {
            class: class,
            name: name.into(),
            ftype: ftype.into(),
            arguments: arguments,
            doc_str: doc_str,
        }
    }

    pub fn format_type(ftype: &'a str) -> Result<&'a str> {
        let ftype_vec: Vec<&str> = ftype.split('(').collect();
        Ok(
            ftype_vec
                .get(0)
                .ok_or_else(|| "Function type can not be parsed from signature.")?
                .trim_right(),
        )
    }

    pub fn format_description(description: &'a str) -> Result<Vec<String>> {
        Ok(
            description
                .split('\n')
                .map(|fd| {
                    String::from(
                        fd.trim_left()
                            .trim_left_matches('*')
                            .trim_left_matches('/')
                            .trim_left(),
                    )
                })
                .collect(),
        )
    }

    pub fn format_arguments(arguments: &[Entity]) -> Result<Vec<Argument>> {
        let mut args = Vec::new();

        for fct_arg in arguments {
            args.push(Argument::new(
                fct_arg.get_display_name().unwrap_or(String::new()),
                fct_arg
                    .get_type()
                    .ok_or_else(|| "Argument type can not be parsed from signature.")?
                    .get_display_name(),
            ));
        }

        Ok(args)
    }
}
