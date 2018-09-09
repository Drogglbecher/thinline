#[macro_use]
extern crate lazy_static;
extern crate thinlinelib;

mod analysis_c;
mod analysis_python;

#[cfg(test)]
mod analysis {

    #[cfg(test)]
    mod c {

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
                    assert!(
                        analysis
                            .collect_sources(&c_test_src_path, &[String::from(".")])
                            .is_ok()
                    );

                    // Then
                    assert_eq!(analysis.project_files().len(), 5);
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
                    assert!(
                        analysis
                            .collect_sources(&c_test_src_path, &[String::from(".")])
                            .is_err()
                    );
                }

                #[test]
                fn when_path_is_no_directory() {
                    // Given
                    let analysis: Analysis<C> = Analysis::new();

                    // When
                    let c_test_src_path = Path::new("tests").join("lib.rs");

                    // Then
                    assert!(
                        analysis
                            .collect_sources(&c_test_src_path, &[String::from(".")])
                            .is_err()
                    );
                }
            }

            #[cfg(test)]
            mod extract_entities {
                use std::path::Path;
                use thinlinelib::analysis::Analysis;
                use thinlinelib::language_type::C;
                use analysis_c::ANALYSIS1_RESULT;

                #[test]
                fn extract_entities() {
                    let analysis: Analysis<C> = Analysis::new();
                    let c_test_src_path = Path::new("tests").join("analysis_c");
                    assert!(
                        analysis
                            .collect_sources(&c_test_src_path, &[String::from(".")])
                            .is_ok()
                    );

                    assert!(analysis.extract_entities().is_ok());

                    let project_files = analysis.project_files();

                    assert_eq!(project_files.len(), 1);

                    let entities = project_files[0].entities();
                    assert_eq!(entities.len(), 1);
                    assert_eq!(entities[0].entities, *ANALYSIS1_RESULT);
                }
            }
        }
    }

    #[cfg(test)]
    mod python {

        #[cfg(test)]
        mod collect_sources {

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
                    let python_test_src_path =
                        Path::new("tests").join("testdata").join("python_sources");
                    assert!(
                        analysis
                            .collect_sources(&python_test_src_path, &[String::from(".")])
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
                            .collect_sources(&python_test_src_path, &[String::from(".")])
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
                            .collect_sources(&python_test_src_path, &[String::from(".")])
                            .is_err()
                    );
                }
            }
        }

        #[cfg(test)]
        mod extract_entities {
            use std::path::Path;
            use thinlinelib::analysis::Analysis;
            use thinlinelib::language_type::Python;
            use analysis_python::ANALYSIS1_RESULT;

            #[test]
            fn extract_entities() {
                let analysis: Analysis<Python> = Analysis::new();
                let py_test_src_path = Path::new("tests").join("analysis_python");
                assert!(
                    analysis
                        .collect_sources(&py_test_src_path, &[String::from(".")])
                        .is_ok()
                );

                assert!(analysis.extract_entities().is_ok());

                let project_files = analysis.project_files();
                assert_eq!(project_files.len(), 1);

                let entities = project_files[0].entities();
                assert_eq!(entities.len(), 1);
                assert_eq!(entities[0].entities, *ANALYSIS1_RESULT);
            }
        }
    }
}
