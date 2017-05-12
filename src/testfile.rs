use std::collections::HashMap;
use std::path::PathBuf;
use testclass::TestClass;

#[derive(Default)]
pub struct TestFile {
    pub path: PathBuf,
    pub preclass_context: Option<String>,
    pub test_namespaces: String,
    pub testclass_map: HashMap<String, TestClass>,
}

impl TestFile {
    pub fn new<P: Into<PathBuf>>(path: P, namespaces: &[String]) -> Self {
        TestFile {
            path: path.into(),
            test_namespaces: if !namespaces.is_empty() {
                namespaces.join("\n")
            } else {
                String::new()
            },
            preclass_context: None,
            testclass_map: HashMap::new(),
        }
    }
}
