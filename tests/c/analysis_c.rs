extern crate thinlinelib;

#[cfg(test)]
mod test_collect_sources {

    #[cfg(test)]
    mod should_succeed {

        use std::path::Path;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::language_type::C;

        #[test]
        fn when_directory_is_valid() {
            // Given
            let analysis: Analysis<C> = Analysis::new();

            // When
            let c_test_src_path = Path::new("tests").join("testdata").join("c_sources");
            assert!(analysis.collect_sources(&c_test_src_path, &[String::from(".")]).is_ok());

            // Then
            assert_eq!(analysis.project_files().len(), 6);
        }
    }

    #[cfg(test)]
    mod should_fail {

        use std::path::Path;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::language_type::C;

        #[test]
        fn when_directory_not_existing() {
            // Given
            let analysis: Analysis<C> = Analysis::new();

            // When
            let c_test_src_path = Path::new("not").join("existing");

            // Then
            assert!(analysis.collect_sources(&c_test_src_path, &[String::from(".")]).is_err());
        }

        #[test]
        fn when_path_is_no_directory() {
            // Given
            let analysis: Analysis<C> = Analysis::new();

            // When
            let c_test_src_path = Path::new("tests").join("lib.rs");

            // Then
            assert!(analysis.collect_sources(&c_test_src_path, &[String::from(".")]).is_err());
        }
    }
}

#[cfg(test)]
mod test_extract_entities {

    use std::path::Path;
    use thinlinelib::analysis::{Analysis, ProjectFile};
    use thinlinelib::language_type::C;

    #[test]
    fn should_succeed() {
        {
            // Given
            let analysis: Analysis<C> = Analysis::new();

            // Then
            assert!(analysis.extract_entities().is_ok());
        }
        {
            // Given
            let analysis: Analysis<C> = Analysis::new();
            let c_test_src_path = Path::new("tests").join("testdata").join("c_sources");

            // Then
            assert!(analysis.collect_sources(&c_test_src_path, &[String::from(".")]).is_ok());
            assert!(analysis.extract_entities().is_ok());
        }
        {
            // Given
            let analysis: Analysis<C> = Analysis::new();
            let c_test_src_path = Path::new("tests").join("testdata").join("c_sources");

            assert!(analysis.collect_sources(&c_test_src_path, &[String::from(".")]).is_ok());
            assert!(analysis.extract_entities().is_ok());

            let project_files: Vec<ProjectFile<C>> = analysis.project_files().to_vec();

            let filtered_project_files: Vec<&ProjectFile<C>> = project_files
                .iter()
                .filter(|pf| pf.path.to_str().unwrap().ends_with("test1.c"))
                .collect();

            let project_file = filtered_project_files[0];
            assert_eq!(filtered_project_files.len(), 1);

            let index = &project_file.entities()[0];
            if let Some(functions) = &index.functions() {
                assert_eq!(functions.len(), 4);

                let fct = &functions[0];
                assert_eq!(fct.name, "test_int_no1");
                assert_eq!(fct.return_type, Some(String::from("int")));
                assert!(fct.arguments.is_some());
                if let Some(args) = &fct.arguments {
                    assert_eq!(args.len(), 2);
                }
                assert!(fct.description.is_some());
                if let Some(desc) = &fct.description {
                    assert_eq!(desc.len(), 6);
                }
            }
        }
    }
}
