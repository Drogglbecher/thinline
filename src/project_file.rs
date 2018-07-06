use clang::Entity;
use error::*;
use function::Function;
use std::cell::RefMut;
use std::path::PathBuf;

pub trait ProjectFile {
    fn new<S: Into<PathBuf>>(path: S) -> Self;

    fn path(&self) -> &PathBuf;

    fn functions(&self) -> RefMut<Vec<Function>>;

    fn add_function(&self, function: Function) {
        self.functions().push(function);
    }

    fn filter_for_functions(&self, entity: &Entity) -> Result<()>;
}
