use entity::Entity;
use language_type::LanguageType;
use std::marker::PhantomData;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestClass {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestFunction {
    pub name: String,
}

#[derive(Default, Debug)]
pub struct Synthesis {}

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
