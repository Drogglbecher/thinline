use failure::Fallible;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use value_parser::ValueParser;
use yaml_rust::YamlLoader;

////////////////////////////////////////////////////////////////////////////////

static STUB_EXTENSION: &str = "stub";

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

trait StubConversion {
    /// Consumes an instance and returns it's elements as `Vec<String>`.
    fn to_stub(self) -> Option<Stub>;
}

impl<'a> StubConversion for Option<&'a str> {
    fn to_stub(self) -> Option<Stub> {
        if let Some(string) = self {
            return Some(Stub::from_str(string));
        }

        None
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
    pub fn parse(&mut self, yml: &str, test_env: &str, base_path: &PathBuf) -> Fallible<()> {
        if let Ok(yml_params) = YamlLoader::load_from_str(read_to_string(yml)?.as_str()) {
            if let Some(yml_param) = yml_params.get(0) {
                let mut stubs = Self::new();

                stubs.file = yml_param
                    .get_str_or_file_content(&[test_env, "file"], base_path, STUB_EXTENSION)
                    .to_stub();
                stubs.class = yml_param
                    .get_str_or_file_content(&[test_env, "class"], base_path, STUB_EXTENSION)
                    .to_stub();
                stubs.function = yml_param
                    .get_str_or_file_content(&[test_env, "function"], base_path, STUB_EXTENSION)
                    .to_stub();

                if let Some(output_format) = yml_param.get_str(&[test_env, "output_format"]) {
                    stubs.output_format = Some(String::from(output_format));
                }

                debug!("Parsed stubs: {:#?}", stubs);
            }
        }

        Ok(())
    }
}
