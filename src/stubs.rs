use failure::Fallible;
use std::{collections::HashMap, fs::read_to_string};
use value_parser::ValueParser;
use yaml_rust::YamlLoader;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
/// A parsed stub.
pub struct Stub {
    content: String,
}

impl Stub {
    /// Creates a `Stub` instance from a `&str`.
    pub fn from_str(content: &str) -> Self {
        Self {
            content: String::from(content),
        }
    }

    pub fn format(&self, dict: HashMap<&str, &str>) -> Fallible<Option<String>> {
        Ok(Some(String::new()))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
/// The parsed stubs.
pub struct Stubs {
    pub file: Option<Stub>,
    pub class: Option<Stub>,
    pub function: Option<Stub>,
    pub output_format: Option<String>,
    pub test_functions: Vec<Stub>,
}

impl Stubs {
    /// Creates a new `Stubs` instance.
    pub fn new() -> Self {
        Self {
            file: None,
            class: None,
            function: None,
            output_format: None,
            test_functions: Vec::new(),
        }
    }

    /// Parses all available stub signatures from the given yaml file.
    pub fn parse(&mut self, yml: &str) -> Fallible<()> {
        if let Ok(yml_params) = YamlLoader::load_from_str(read_to_string(yml)?.as_str()) {
            if let Some(yml_param) = yml_params.get(0) {
                let mut stubs = Self::new();

                if let Some(file) = yml_param.get_str(&["file"]) {
                    stubs.file = Some(Stub::from_str(file));
                }
                if let Some(class) = yml_param.get_str(&["class"]) {
                    stubs.class = Some(Stub::from_str(class));
                }
                if let Some(function) = yml_param.get_str(&["function"]) {
                    stubs.function = Some(Stub::from_str(function));
                }
                if let Some(output_format) = yml_param.get_str(&["output_format"]) {
                    stubs.output_format = Some(String::from(output_format));
                }
            }
        }

        Ok(())
    }
}
