use function::Function;
use language_type::LanguageType;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::path::PathBuf;

pub trait ProjectFileT<T> {
    fn new<S: Into<PathBuf>>(path: S) -> Self;

    fn path(&self) -> &PathBuf;

    fn functions(&self) -> Ref<Vec<Function>>;

    fn functions_mut(&self) -> RefMut<Vec<Function>>;

    fn add_function(&self, function: Function) {
        self.functions_mut().push(function);
    }
}

#[derive(Default, Clone, Debug)]
pub struct ProjectFile<T> {
    pub pf_type: PhantomData<T>,
    pub path: PathBuf,
    pub functions: RefCell<Vec<Function>>,
}

/// Reprensents a parsed project file.
impl<T> ProjectFileT<T> for ProjectFile<T>
where
    T: LanguageType<T>,
{
    fn new<S: Into<PathBuf>>(path: S) -> Self {
        ProjectFile {
            pf_type: PhantomData,
            path: path.into(),
            functions: RefCell::new(Vec::new()),
        }
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn functions(&self) -> Ref<Vec<Function>> {
        self.functions.borrow()
    }

    fn functions_mut(&self) -> RefMut<Vec<Function>> {
        self.functions.borrow_mut()
    }
}
