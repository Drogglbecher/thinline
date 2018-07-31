extern crate thinlinelib;

use std::path::PathBuf;
use thinlinelib::analysis::ProjectFile;
use thinlinelib::language_type::Python;

#[test]
fn test_new() {
    // Given
    let project_file: ProjectFile<Python> = ProjectFile::new("testpath");

    // Then
    assert_eq!(*project_file.path(), PathBuf::from("testpath"));
    assert_eq!(project_file.functions().len(), 0);
}
