use clang::Entity;
use error::*;
use function::Function;
use std::cell::{Ref, RefMut};
use std::path::PathBuf;

pub trait ProjectFile {
    fn new<S: Into<PathBuf>>(path: S) -> Self;

    fn path(&self) -> &PathBuf;

    fn functions(&self) -> Ref<Vec<Function>>;

    fn functions_mut(&self) -> RefMut<Vec<Function>>;

    fn add_function(&self, function: Function) {
        self.functions_mut().push(function);
    }

    fn filter_for_functions(&self, entity: &Entity) -> Result<()>;
}
