use analysis::Analysis;
use error::*;

pub trait LanguageType<T>: Default
where
    T: LanguageType<T>,
{
    fn file_types() -> &'static [&'static str];
    fn extract_functions(analysis: &Analysis<T>) -> Result<()>;
}
