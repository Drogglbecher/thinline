use clang::Entity;
use error::*;
use function::Function;
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

    fn extract_functions(&self, entity: &Entity) -> Result<()>;
}

#[derive(Default, Clone)]
pub struct ProjectFile<T> {
    pub pf_type: PhantomData<T>,
    pub path: PathBuf,
    pub functions: RefCell<Vec<Function>>,
}
