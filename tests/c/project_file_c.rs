extern crate thinlinelib;

use std::path::PathBuf;
use thinlinelib::project_file::{ProjectFileT, ProjectFile};
use thinlinelib::c::C;

#[test]
fn test_new() {
    // Given
    let project_file: ProjectFile<C> = ProjectFile::new("testpath");

    // Then
    assert_eq!(*project_file.path(), PathBuf::from("testpath"));
    assert_eq!(project_file.functions().len(), 0);
}
