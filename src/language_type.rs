use analysis::Analysis;
use error::*;

pub trait LanguageType: Default {
    fn file_types() -> &'static [&'static str];
    fn extract_functions<T: LanguageType>(analysis: &Analysis<T>) -> Result<()>;
}
