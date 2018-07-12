extern crate thinlinelib;

use thinlinelib::analysis::Analysis;
use thinlinelib::c::analysis_c::{AnalysisC, C_FILE_EXTENSIONS};

#[test]
fn test_new() {
    let analysis = AnalysisC::new();

    assert_eq!(analysis.file_types(), C_FILE_EXTENSIONS);
    assert_eq!(analysis.project_files().len(), 0);
}

#[cfg(test)]
mod test_collect_sources {

    #[cfg(test)]
    mod should_succeed {

        use std::path::Path;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::c::analysis_c::AnalysisC;

        // Should succeed
        #[test]
        fn when_directory_is_valid() {
            // Given
            let analysis = AnalysisC::new();

            // When
            let c_test_src_path = Path::new("tests").join("testdata").join("c_sources");
            assert!(analysis.collect_sources(&c_test_src_path, &["."]).is_ok());

            // Then
            assert_eq!(analysis.project_files().len(), 5);
        }
    }

    #[cfg(test)]
    mod should_fail {

        use std::path::Path;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::c::analysis_c::AnalysisC;

        // Directory not existing
        #[test]
        fn when_directory_not_existing() {
            // Given
            let analysis = AnalysisC::new();
            let c_test_src_path = Path::new("not").join("existing");

            // Then
            assert!(analysis.collect_sources(&c_test_src_path, &["."]).is_err());
        }

        // Path is no directory
        #[test]
        fn when_path_is_no_directory() {
            // Given
            let analysis = AnalysisC::new();
            let c_test_src_path = Path::new("tests").join("lib.rs");

            // Then
            assert!(analysis.collect_sources(&c_test_src_path, &["."]).is_err());
        }
    }
}
