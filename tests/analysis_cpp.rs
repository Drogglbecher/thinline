extern crate snapshot;
extern crate thinlinelib;

#[cfg(test)]
mod cpp {
    use snapshot::snapshot;
    use std::path::Path;
    use thinlinelib::{analysis::Analysis, entity::EntityType, language_type::Cpp};

    #[test]
    fn cpp() {
        // test_collect_sources
        {
            // should_succeed when_directory_is_valid
            {
                // Given
                let analysis: Analysis<Cpp> = Analysis::new();

                // When
                let cpp_test_src_path = Path::new("tests").join("testdata").join("cpp_sources");
                assert!(
                    analysis
                        .collect_sources(&cpp_test_src_path, &[String::from(".")])
                        .is_ok()
                );

                // Then
                assert_eq!(analysis.project_files().len(), 2);
            }

            // should_fail
            {
                // when_directory_not_existing
                {
                    // Given
                    let analysis: Analysis<Cpp> = Analysis::new();

                    // When
                    let cpp_test_src_path = Path::new("not").join("existing");

                    // Then
                    assert!(
                        analysis
                            .collect_sources(&cpp_test_src_path, &[String::from(".")])
                            .is_err()
                    );
                }

                // when_path_is_no_directory
                {
                    // Given
                    let analysis: Analysis<Cpp> = Analysis::new();

                    // When
                    let cpp_test_src_path = Path::new("tests").join("lib.rs");

                    // Then
                    assert!(
                        analysis
                            .collect_sources(&cpp_test_src_path, &[String::from(".")])
                            .is_err()
                    );
                }
            }
        }
    }

    fn extract_entities_cpp() -> Vec<EntityType> {
        let analysis: Analysis<Cpp> = Analysis::new();
        let cpp_test_src_path = Path::new("tests").join("testdata").join("analysis");
        assert!(
            analysis
                .collect_sources(&cpp_test_src_path, &[String::from(".")])
                .is_ok()
        );

        assert!(analysis.extract_entities().is_ok());

        let project_files = analysis.project_files();

        assert_eq!(project_files.len(), 1);

        let entities = project_files[0].entities();
        assert_eq!(entities.len(), 1);
        entities[0].clone().entities
    }

    #[cfg(target_os = "linux")]
    #[snapshot]
    fn extract_entities_linux_cpp() -> Vec<EntityType> {
        extract_entities_cpp()
    }

    #[cfg(target_os = "windows")]
    #[snapshot]
    fn extract_entities_windows_cpp() -> Vec<EntityType> {
        extract_entities_cpp()
    }
}
