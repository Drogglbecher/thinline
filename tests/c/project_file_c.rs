extern crate thinlinelib;

use std::path::PathBuf;
use thinlinelib::project_file::ProjectFile;
use thinlinelib::c::project_file_c::ProjectFileC;

#[test]
fn test_new() {
    // Given
    let project_file = ProjectFileC::new("testpath");

    // Then
    assert_eq!(*project_file.path(), PathBuf::from("testpath"));
    assert_eq!(project_file.functions().len(), 0);
}
