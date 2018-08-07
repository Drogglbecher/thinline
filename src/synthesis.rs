use language_type::LanguageType;
use std::marker::PhantomData;

#[derive(Default, Debug)]
pub struct Synthesis {}

#[derive(Default, Debug)]
pub struct TestFile<T> {
    pub pf_type: PhantomData<T>,
}

/// Reprensents a test file.
impl<T> TestFile<T>
where
    T: LanguageType,
{}
