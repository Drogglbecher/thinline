use error::*;
use clang::Entity;

pub struct Parameter {
    pub name: String,
    pub ctype: String,
}

/// Represents a parsed parameter from a function.
impl Parameter {
    pub fn new<S: Into<String>>(name: S, ctype: S) -> Self {
        Parameter {
            name: name.into(),
            ctype: ctype.into(),
        }
    }
}

#[derive(Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arg_str: String,
}

impl FunctionCall {
    pub fn new<S: Into<String>>(name: S, arg_str: S) -> Self {
        FunctionCall {
            name: name.into(),
            arg_str: arg_str.into(),
        }
    }
}

/// Reprensents a parsed function type.
pub struct Function {
    pub class_name: Option<String>,
    pub name: String,
    pub ctype: String,
    pub parameter: Vec<Parameter>,
    pub doc_str: Vec<String>,
}

impl Function {
    /// Creates a new `Function` instance.
    pub fn new<S: Into<String>>(class: Option<String>, name: S) -> Self {
        Function {
            class_name: class,
            name: name.into(),
            ctype: String::new(),
            parameter: Vec::new(),
            doc_str: Vec::new(),
        }
    }

    /// Sets the type of the function parsed from the signature.
    pub fn set_type(&mut self, fct_type: &str) -> Result<()> {
        let fct_type_vec: Vec<&str> = fct_type.split('(').collect();
        self.ctype = String::from(
            fct_type_vec
                .get(0)
                .ok_or_else(|| "Function type can not be parsed from signature.")?
                .trim_right(),
        );

        Ok(())
    }

    /// Set the parsed description of the function.
    pub fn set_description(&mut self, fct_desc: &str) {
        self.doc_str.extend(fct_desc.split('\n').map(|fd| {
            String::from(
                fd.trim_left()
                    .trim_left_matches('*')
                    .trim_left_matches('/')
                    .trim_left(),
            )
        }));
    }

    /// Set the parsed arguments of the function.
    pub fn set_args(&mut self, fct_args: &[Entity]) -> Result<()> {
        for fct_arg in fct_args {
            let mut arg_dpn = String::new();
            if let Some(tp) = fct_arg.get_type() {
                if let Some(dpn) = fct_arg.get_display_name() {
                    arg_dpn = dpn;
                }
                self.parameter.push(
                    Parameter::new(arg_dpn, tp.get_display_name()),
                );
            }
        }

        Ok(())
    }

    /// Returns the argument list as string.
    pub fn get_args_as_str(&self) -> String {
        let mut s = String::from("(");
        for arg in &self.parameter {
            s.push_str(&format!("{} {},", arg.ctype, arg.name))
        }
        if !self.parameter.is_empty() {
            s.pop();
        }
        s.push(')');

        s
    }

    /// Returns the function in form of a call with value names and positions.
    pub fn format_as_call(&self) -> FunctionCall {
        let mut arg_str = String::from("(");
        for arg in &self.parameter {
            arg_str.push_str(&format!("{},", arg.name))
        }
        if !self.parameter.is_empty() {
            arg_str.pop();
        }
        arg_str.push(')');

        FunctionCall::new(self.name.to_owned(), arg_str)
    }
}
