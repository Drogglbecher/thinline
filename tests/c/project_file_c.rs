extern crate thinlinelib;

use std::path::PathBuf;
use thinlinelib::analysis::ProjectFile;
use thinlinelib::language_type::C;

#[test]
fn test_new() {
    // Given
    let project_file: ProjectFile<C> = ProjectFile::new("testpath");

    // Then
    assert_eq!(*project_file.path(), PathBuf::from("testpath"));
    assert_eq!(project_file.entit().len(), 0);
}
