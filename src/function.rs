use argument::Argument;
use error::*;

/// Reprensents a parsed function type.
#[derive(Default, Clone, Debug)]
pub struct Function {
    pub class: Option<String>,
    pub name: String,
    pub ftype: Option<String>,
    pub arguments: Vec<Argument>,
    pub description: Vec<String>,
}

impl Function {
    /// Creates a new `Function` instance.
    pub fn new<S: Into<String>>(class: Option<String>, name: S) -> Self {
        Function {
            class: class,
            name: name.into(),
            ftype: None,
            arguments: Vec::new(),
            description: Vec::new(),
        }
    }

    pub fn set_format_type(&mut self, ftype: &str) -> Result<()> {
        if ftype.is_empty() {
            self.ftype = None;
        } else {
            let ftype_vec: Vec<&str> = ftype.split('(').collect();
            self.ftype = Some(String::from(
                ftype_vec
                    .get(0)
                    .ok_or_else(|| "Function type can not be parsed from signature.")?
                    .trim_right(),
            ));
        }

        Ok(())
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = description
            .split('\n')
            .map(|fd| {
                String::from(
                    fd.trim_left()
                        .trim_left_matches('*')
                        .trim_left_matches('/')
                        .trim_left(),
                )
            })
            .filter(|ref c| !c.is_empty() && c.as_str() != "**")
            .collect()
    }

    pub fn set_arguments(&mut self, arguments: &[Argument]) {
        self.arguments = arguments.into();
    }
}
