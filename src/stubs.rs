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

    /// Formats a `Stub` with the given `HashMap`.
    /// The keys within the stub context are replaced with the
    /// connected values.
    pub fn format(&self, dict: &HashMap<&str, &str>) -> Fallible<Option<String>> {
        let mut form_stub: String = self.content.clone();
        for (key, val) in dict.iter() {
            form_stub = form_stub.replace(format!("{}", key).as_str(), val);
        }
        Ok(Some(form_stub))
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
        let yml_params = YamlLoader::load_from_str(read_to_string(yml)?.as_str())?;

        if let Some(yml_param) = yml_params.get(0) {
            self.file = yml_param
                .get_str_or_file_content(&[test_env, "file"], base_path, STUB_EXTENSION)
                .to_stub();
            self.class = yml_param
                .get_str_or_file_content(&[test_env, "class"], base_path, STUB_EXTENSION)
                .to_stub();
            self.function = yml_param
                .get_str_or_file_content(&[test_env, "function"], base_path, STUB_EXTENSION)
                .to_stub();

            if let Some(output_format) = yml_param.get_str(&[test_env, "output_format"]) {
                self.output_format = Some(String::from(output_format));
            }

            debug!("Parsed stubs: {:#?}", self);
        }

        Ok(())
    }
}
