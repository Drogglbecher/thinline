extern crate thinlinelib;

#[cfg(test)]
mod analysis {

    #[cfg(test)]
    mod c {
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
                    assert_eq!(analysis.project_files().len(), 3);
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
    }
}
