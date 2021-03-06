use analysis::ProjectFile;
use entity::{Entity, EntityType};
use failure::Fallible;
use language_type::LanguageType;
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};
use stubs::Stubs;

////////////////////////////////////////////////////////////////////////////////

static STUB_ID_SET_UP_CONTEXT: &str = "#SET_UP";
static STUB_ID_TEAR_DOWN_CONTEXT: &str = "#TEAR_DOWN";
static STUB_ID_CONSTRUCTOR_CONTEXT: &str = "#CONSTRUCTOR";
static STUB_ID_DESTRUCTOR_CONTEXT: &str = "#DESTRUCTOR";
static STUB_ID_CLASS_CONTEXT: &str = "#CLASS_CONTEXT";

static STUB_ID_TEST_CLASS: &str = "#TEST_CLASS";
static STUB_ID_TEST_NAME: &str = "#TEST_NAME";
static STUB_ID_TEST_CONTEXT: &str = "#TEST_CONTEXT";

type StubContext = String;

pub trait StubContextConversion {
    fn convert(stub_context_type: &StubContextType) -> Option<&StubContext>;
}

impl StubContextConversion for StubContext {
    fn convert(stub_contex_type: &StubContextType) -> Option<&StubContext> {
        match stub_contex_type {
            StubContextType::SetUpContext(stub_context)
            | StubContextType::TearDownContext(stub_context)
            | StubContextType::ConstructorContext(stub_context)
            | StubContextType::DestructorContext(stub_context)
            | StubContextType::ClassContext(stub_context) => Some(stub_context),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StubContextType {
    SetUpContext(StubContext),
    TearDownContext(StubContext),
    ConstructorContext(StubContext),
    DestructorContext(StubContext),
    ClassContext(StubContext),
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestFunction {
    pub name: String,
}

////////////////////////////////////////////////////////////////////////////////

/// The representation of a `TestClass` containing different
/// contexts for con-/destructor, setUp, tearDown methods
/// and so on and a vector of test functions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestClass {
    pub stub_context: Vec<StubContextType>,
    pub test_functions: Vec<TestFunction>,
}

impl TestClass {
    /// Creates a new `StubContext` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::synthesis::{TestClass};
    ///
    /// let test_class = TestClass::new();
    ///
    /// assert!(test_class.stub_context.is_empty());
    /// assert!(test_class.test_functions.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            stub_context: Vec::new(),
            test_functions: Vec::new(),
        }
    }

    /// Adds a `StubContext` to the `TestClass` instance.
    ///
    /// With setting stub contexts to certain values, it is ensured, that these
    /// contexts (e.g. test class constructor or setUp function) are replaced
    /// with the values to generate specific test classes.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::synthesis::{StubContextType, TestClass};
    ///
    /// let mut test_class = TestClass::new();
    /// let stub_context_type = StubContextType::SetUpContext(String::from("setup = new Setup();"));
    /// test_class.add_stub_context(stub_context_type);
    ///
    /// assert_eq!(test_class.stub_context.len(), 1);
    /// ```
    pub fn add_stub_context(&mut self, context: StubContextType) -> Option<&StubContext> {
        self.stub_context.push(context);
        if let Some(stub_context) = self.stub_context.last() {
            return StubContext::convert(stub_context);
        }

        None
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct TestFile<T> {
    pub path: PathBuf,
    pub entities: Vec<Entity>,
    pub pf_type: PhantomData<T>,
}

/// Represents a test file.
impl<T> TestFile<T>
where
    T: LanguageType,
{
    /// Creates a new `TestFile` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use thinlinelib::synthesis::TestFile;
    /// use thinlinelib::language_type::C;
    ///
    /// let test_file: TestFile<C> = TestFile::new("test_file");
    ///
    /// assert!(test_file.entities.is_empty());
    /// ```
    pub fn new<S: Into<PathBuf>>(path: S) -> Self {
        Self {
            path: path.into(),
            entities: Vec::new(),
            pf_type: PhantomData,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Synthesis<T>
where
    T: LanguageType,
{
    stubs: Stubs,
    pub test_files: Vec<TestFile<T>>,
}

impl<T> Synthesis<T>
where
    T: LanguageType,
{
    /// Creates a new `Synthesis` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::synthesis::Synthesis;
    /// use thinlinelib::language_type::C;
    ///
    /// let synthesis: Synthesis<C> = Synthesis::new();
    ///
    /// assert_eq!(synthesis.test_files.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            stubs: Stubs::new(),
            test_files: Vec::new(),
        }
    }

    /// Parses all available `Stubs` from the given yaml file.
    pub fn parse_stubs(&mut self, yml: &str, test_env: &str, base_path: &PathBuf) -> Fallible<()> {
        self.stubs.parse(yml, test_env, base_path)
    }

    fn process_entities(&self, parent: &Entity, children: &[EntityType]) -> Fallible<()> {
        for child in children {
            match child {
                EntityType::Function(function) => {
                    if let Some(description) = &function.description {
                        trace!(
                            "Description found for function {} with parent {}: {:?}",
                            function.name,
                            parent.name,
                            description.lines
                        );
                        if let Some(function_stub) = &self.stubs().function {
                            let function_stub_format_hashes: HashMap<
                                &str,
                                &str,
                            > = [
                                (STUB_ID_TEST_CLASS, parent.name.as_str()),
                                (STUB_ID_TEST_NAME, function.name.as_str()),
                            ].iter()
                                .cloned()
                                .collect();
                            debug!(
                                "formatted stub: {:?}",
                                function_stub.format(&function_stub_format_hashes)?
                            );
                        }
                    }
                }
                EntityType::Entity(entity) => {
                    if let Some(description) = &entity.description {
                        trace!(
                            "Description found for entity {} with parent {}: {:?}",
                            entity.name,
                            parent.name,
                            description.lines
                        );
                    }
                    self.process_entities(entity, &entity.entities)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn process_testfile(&self, project_file: &ProjectFile<T>) -> Fallible<()> {
        for entity in project_file.entities().iter() {
            self.process_entities(&entity, &entity.entities)?;
        }

        Ok(())
    }

    /// Returns a reference to the synthesis `Stubs`.
    pub fn stubs(&self) -> &Stubs {
        &self.stubs
    }
}
