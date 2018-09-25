extern crate snapshot;
extern crate thinlinelib;

pub static MULTILINE_COMMENT: &str = "
**this
is
a

multiline
**
//comment";

#[cfg(test)]
mod analysis {

    #[cfg(test)]
    mod argument {
        use thinlinelib::analysis::Argument;

        #[test]
        fn new() {
            let argument = Argument::new("arg", Some("std::string"));

            assert_eq!(argument.name, String::from("arg"));
            assert_eq!(argument.atype, Some(String::from("std::string")));
            assert!(argument.value.is_none());
        }

        #[test]
        fn set_value() {
            let mut argument = Argument::new("arg", Some("std::string"));
            argument.set_value("FirstArg");

            assert_eq!(argument.value, Some(String::from("FirstArg")));
        }
    }

    #[cfg(test)]
    mod function {
        use thinlinelib::analysis::{Argument, Function};
        use MULTILINE_COMMENT;

        #[test]
        fn new() {
            let function = Function::new("fct");

            assert_eq!(function.name, String::from("fct"));
            assert!(function.return_type.is_none());
            assert!(function.arguments.is_empty());
            assert!(function.description.is_none());
        }

        #[test]
        fn set_return_type() {
            let mut fct = Function::new("fct");

            {
                assert!(fct.set_return_type("").is_ok());
                assert_eq!(fct.return_type, None);
            }

            {
                assert!(fct.set_return_type("int").is_ok());
                assert_eq!(fct.return_type, Some(String::from("int")));
            }
        }

        #[test]
        fn set_description() {
            let mut fct = Function::new("fct");

            {
                fct.set_description(MULTILINE_COMMENT);

                assert!(fct.description.is_some());
                let fct_desc = fct.description.unwrap().description;

                assert_eq!(fct_desc.len(), 5);
                assert_eq!(fct_desc[0], "this");
                assert_eq!(fct_desc[1], "is");
                assert_eq!(fct_desc[2], "a");
                assert_eq!(fct_desc[3], "multiline");
                assert_eq!(fct_desc[4], "comment");
            }
        }

        #[test]
        fn set_arguments() {
            let mut fct = Function::new("fct");

            {
                fct.set_arguments(&vec![]);
                assert!(fct.arguments.is_empty());
            }

            {
                let arg1 = Argument::new("arg1", Some("std::string"));
                let arg2 = Argument::new("arg2", Some("std::uint32"));
                fct.set_arguments(&vec![arg1, arg2]);

                assert_eq!(fct.arguments.len(), 2);
                assert_eq!(fct.arguments[0].name, "arg1");
                assert_eq!(fct.arguments[1].name, "arg2");
            }
        }
    }

    #[cfg(test)]
    mod enumeration {
        use thinlinelib::analysis::{Argument, Enum};

        #[test]
        fn new() {
            let enumeration = Enum::new("enum");

            assert_eq!(enumeration.name, String::from("enum"));
            assert!(enumeration.arguments.is_empty());
            assert!(enumeration.etype.is_none());
        }

        #[test]
        fn set_arguments() {
            let mut enumeration = Enum::new("enum");

            {
                enumeration.set_arguments(&vec![]);
                assert!(enumeration.arguments.is_empty());
            }

            {
                let args = vec![
                    Argument::new("Zero", Some("0")),
                    Argument::new("Two", Some("2")),
                ];
                enumeration.set_arguments(&args);

                assert_eq!(enumeration.arguments.len(), 2);
                assert_eq!(enumeration.arguments[0].name, "Zero");
                assert_eq!(enumeration.arguments[1].name, "Two");
            }
        }

        #[test]
        fn push_argument() {
            let mut enumeration = Enum::new("enum");
            assert!(enumeration.arguments.is_empty());

            enumeration.push_argument(Argument::new("arg", Some("uint32")));
            assert_eq!(enumeration.arguments.len(), 1);

            enumeration.push_argument(Argument::new("new_arg", Some("uint64")));
            assert_eq!(enumeration.arguments.len(), 2);
        }
    }

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
                use snapshot::snapshot;
                use std::path::Path;
                use thinlinelib::analysis::Analysis;
                use thinlinelib::entity::EntityType;
                use thinlinelib::language_type::C;

                fn extract_entities_c() -> Vec<EntityType> {
                    let analysis: Analysis<C> = Analysis::new();
                    let c_test_src_path = Path::new("tests").join("testdata").join("analysis");
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
                    entities[0].clone().entities
                }

                #[cfg(target_os = "linux")]
                #[snapshot]
                fn extract_entities_linux_c() -> Vec<EntityType> {
                    extract_entities_c()
                }

                #[cfg(target_os = "windows")]
                #[snapshot]
                fn extract_entities_windows_c() -> Vec<EntityType> {
                    extract_entities_c()
                }
            }
        }
    }

    #[cfg(test)]
    mod cpp {

        #[cfg(test)]
        mod test_collect_sources {

            #[cfg(test)]
            mod should_succeed {

                use std::path::Path;
                use thinlinelib::analysis::Analysis;
                use thinlinelib::language_type::Cpp;

                #[test]
                fn when_directory_is_valid() {
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
            }

            #[cfg(test)]
            mod should_fail {

                use std::path::Path;
                use thinlinelib::analysis::Analysis;
                use thinlinelib::language_type::Cpp;

                #[test]
                fn when_directory_not_existing() {
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

                #[test]
                fn when_path_is_no_directory() {
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

            #[cfg(test)]
            mod extract_entities {
                use snapshot::snapshot;
                use std::path::Path;
                use thinlinelib::analysis::Analysis;
                use thinlinelib::entity::EntityType;
                use thinlinelib::language_type::Cpp;

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
            use snapshot::snapshot;
            use std::path::Path;
            use thinlinelib::analysis::Analysis;
            use thinlinelib::entity::EntityType;
            use thinlinelib::language_type::Python;

            fn extract_entities_python() -> Vec<EntityType> {
                let analysis: Analysis<Python> = Analysis::new();
                let py_test_src_path = Path::new("tests").join("testdata").join("analysis");
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
                entities[0].clone().entities
            }

            #[cfg(target_os = "linux")]
            #[snapshot]
            fn extract_entities_linux_python() -> Vec<EntityType> {
                extract_entities_python()
            }

            #[cfg(target_os = "windows")]
            #[snapshot]
            fn extract_entities_windows_python() -> Vec<EntityType> {
                extract_entities_python()
            }
        }
    }
}
