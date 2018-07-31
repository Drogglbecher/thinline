extern crate thinlinelib;

#[cfg(test)]
mod test_collect_sources {

    #[cfg(test)]
    mod should_succeed {

        use std::path::Path;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::language_type::Python;

        #[test]
        fn when_directory_is_valid() {
            // Given
            let analysis: Analysis<Python> = Analysis::new();

            // When
            let python_test_src_path = Path::new("tests").join("testdata").join("python_sources");
            assert!(
                analysis
                    .collect_sources(&python_test_src_path, &["."])
                    .is_ok()
            );

            // Then
            assert_eq!(analysis.project_files().len(), 2);
        }
    }

    #[cfg(test)]
    mod should_fail {

        use std::path::Path;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::language_type::Python;

        #[test]
        fn when_directory_not_existing() {
            // Given
            let analysis: Analysis<Python> = Analysis::new();

            // When
            let python_test_src_path = Path::new("not").join("existing");

            // Then
            assert!(
                analysis
                    .collect_sources(&python_test_src_path, &["."])
                    .is_err()
            );
        }

        #[test]
        fn when_path_is_no_directory() {
            // Given
            let analysis: Analysis<Python> = Analysis::new();

            // When
            let python_test_src_path = Path::new("tests").join("lib.rs");

            // Then
            assert!(
                analysis
                    .collect_sources(&python_test_src_path, &["."])
                    .is_err()
            );
        }
    }
}

#[cfg(test)]
mod test_extract_entities {

    use std::path::Path;
    use thinlinelib::analysis::{Analysis, ProjectFile};
    use thinlinelib::language_type::Python;

    #[test]
    fn should_succeed() {
        {
            // Given
            let analysis: Analysis<Python> = Analysis::new();

            // Then
            assert!(analysis.extract_entities().is_ok());
        }
        {
            // Given
            let analysis: Analysis<Python> = Analysis::new();
            let python_test_src_path = Path::new("tests").join("testdata").join("python_sources");

            // Then
            assert!(
                analysis
                    .collect_sources(&python_test_src_path, &["."])
                    .is_ok()
            );
            assert!(analysis.extract_entities().is_ok());
        }
        {
            // Given
            let analysis: Analysis<Python> = Analysis::new();
            let python_test_src_path = Path::new("tests").join("testdata").join("python_sources");

            assert!(
                analysis
                    .collect_sources(&python_test_src_path, &["."])
                    .is_ok()
            );
            assert!(analysis.extract_entities().is_ok());

            let project_files: Vec<ProjectFile<Python>> = analysis.project_files().to_vec();

            let filtered_project_files: Vec<&ProjectFile<Python>> = project_files
                .iter()
                .filter(|pf| pf.path().to_str().unwrap().ends_with("/test1.py"))
                .collect();

            let project_file = filtered_project_files[0];
            assert_eq!(filtered_project_files.len(), 1);

            assert_eq!(project_file.functions().len(), 4);
            let functions = project_file.functions();
            let ffct = functions
                .iter()
                .filter(|fctf| fctf.name.contains("test_str"))
                .next();

            assert!(ffct.is_some());
            let fct = ffct.unwrap();

            assert!(fct.class.is_some());
            assert_eq!(fct.name, "test_str");
            assert_eq!(fct.ftype, None);
            assert_eq!(fct.arguments.len(), 2);
            assert_eq!(fct.description.len(), 3);
        }
    }
}
