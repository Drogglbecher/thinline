use failure::Fallible;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
/// A parsed stub.
pub struct Stub;

impl Stub {
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
    pub fn parse(self, yml: &str) -> Fallible<()> {
        Ok(())
    }
}
