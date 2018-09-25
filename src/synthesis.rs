use entity::Entity;
use language_type::LanguageType;
use std::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

static STUB_ID_SET_UP_CONTEXT: &str = "#TL_SET_UP_CONTEXT";
static STUB_ID_TEAR_DOWN_CONTEXT: &str = "#TL_TEAR_DOWN_CONTEXT";
static STUB_ID_CONSTRUCTOR_CONTEXT: &str = "#TL_CONSTRUCTOR_CONTEXT";
static STUB_ID_DESTRUCTOR_CONTEXT: &str = "#TL_DESTRUCTOR_CONTEXT";
static STUB_ID_CLASS_CONTEXT: &str = "#TL_CLASS_CONTEXT";

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

/// The representation of a TestClass containing different
/// contexts for con-/destructor, setUp, tearDown methods
/// and so on and a vector of test functions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestClass {
    pub stub_context: Vec<StubContextType>,
    pub test_functions: Vec<TestFunction>,
}

impl TestClass {
    /// Creates a new StubContext instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::synthesis::{TestClass};
    ///
    /// let mut test_class = TestClass::new();
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

    /// Adds a StubContext to the TestClass instance.
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
    pub pf_type: PhantomData<T>,
    pub entities: Vec<Entity>,
}

/// Represents a test file.
impl<T> TestFile<T>
where
    T: LanguageType,
{
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Synthesis {}
