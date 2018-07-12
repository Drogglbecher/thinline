extern crate thinlinelib;

use std::path::Path;
use thinlinelib::analysis::Analysis;
use thinlinelib::c::analysis_c::{AnalysisC, C_FILE_EXTENSIONS};

#[test]
fn test_new() {
    let analysis = AnalysisC::new();

    assert_eq!(analysis.file_types(), C_FILE_EXTENSIONS);
    assert_eq!(analysis.project_files().len(), 0);
}

#[test]
fn test_collect_sources() {
    let c_test_src_path = Path::new("tests").join("testdata").join("c_sources");
    let analysis = AnalysisC::new();

    assert!(analysis.collect_sources(&c_test_src_path, &["."]).is_ok());

    assert_eq!(analysis.project_files().len(), 3);
}
