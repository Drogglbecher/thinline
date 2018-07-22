use function::Function;
use language_type::LanguageType;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Default, Clone, Debug)]
pub struct ProjectFile<T> {
    pub pf_type: PhantomData<T>,
    pub path: PathBuf,
    pub functions: RefCell<Vec<Function>>,
}

/// Reprensents a parsed project file.
impl<T> ProjectFile<T>
where
    T: LanguageType<T>,
{
    pub fn new<S: Into<PathBuf>>(path: S) -> Self {
        ProjectFile {
            pf_type: PhantomData,
            path: path.into(),
            functions: RefCell::new(Vec::new()),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn functions(&self) -> Ref<Vec<Function>> {
        self.functions.borrow()
    }

    pub fn functions_mut(&self) -> RefMut<Vec<Function>> {
        self.functions.borrow_mut()
    }

    pub fn add_function(&self, function: Function) {
        self.functions_mut().push(function);
    }
}
