extern crate thinlinelib;

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

            #[cfg(test)]
            mod extract_entities {
                use std::path::Path;
                use thinlinelib::analysis::Analysis;
                use thinlinelib::entity::{Argument, EntityType, Function};
                use thinlinelib::language_type::C;

                #[test]
                fn extract_entities() {
                    let analysis: Analysis<C> = Analysis::new();
                    let c_test_src_path = Path::new("tests")
                        .join("testdata")
                        .join("c_sources")
                        .join("language_type");
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

                    let index_option = entities[0].clone();
                    assert_eq!(
                        index_option.entities,
                        Some(vec![
                            EntityType::Function(Function {
                                name: String::from("test_int_no1"),
                                return_type: Some(String::from("int")),
                                arguments: Some(vec![
                                    Argument {
                                        name: String::from("no1"),
                                        atype: Some(String::from("int")),
                                        value: None,
                                    },
                                    Argument {
                                        name: String::from("no2"),
                                        atype: Some(String::from("int")),
                                        value: None,
                                    },
                                ]),
                                description: Some(vec![
                                    String::from("#TL_TESTCASE(Source1::CheckIfSumWorks)"),
                                    String::from("int test_no = 2;"),
                                    String::from(
                                        "#TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]"
                                    ),
                                    String::from("#TL_EQ[TL_FCT(no1: 5, no2: 2) => 7]"),
                                    String::from("EXPECT_EQ(11, test_int_no1(9, 2));"),
                                    String::from("#!TL_TESTCASE"),
                                ]),
                            }),
                            EntityType::Function(Function {
                                name: String::from("test_ptr"),
                                return_type: Some(String::from("int")),
                                arguments: Some(vec![
                                    Argument {
                                        name: String::from("no1"),
                                        atype: Some(String::from("const int *const")),
                                        value: None,
                                    },
                                    Argument {
                                        name: String::from("no2"),
                                        atype: Some(String::from("const int *const")),
                                        value: None,
                                    },
                                ]),
                                description: Some(vec![
                                    String::from("#TL_TESTCASE(Source1::TestPtr)"),
                                    String::from("int test_no = 2;"),
                                    String::from("int test_no2 = 5;"),
                                    String::from(
                                        "#TL_EQ[TL_FCT(no1: &test_no, no2: &test_no2) => 7]"
                                    ),
                                    String::from("#!TL_TESTCASE"),
                                ]),
                            }),
                            EntityType::Function(Function {
                                name: String::from("test_empty_fct"),
                                return_type: Some(String::from("int")),
                                arguments: None,
                                description: Some(vec![
                                    String::from("#TL_TESTCASE(Source1::EmptyFct)"),
                                    String::from("#TL_EQ[TL_FCT() => 7]"),
                                    String::from("#TL_NE[TL_FCT() => 4]"),
                                    String::from("#!TL_TESTCASE"),
                                ]),
                            }),
                            EntityType::Function(Function {
                                name: String::from("main"),
                                return_type: Some(String::from("int")),
                                arguments: Some(vec![
                                    Argument {
                                        name: String::from("argc"),
                                        atype: Some(String::from("const int")),
                                        value: None,
                                    },
                                    Argument {
                                        name: String::from("argv"),
                                        atype: Some(String::from("char *const []")),
                                        value: None,
                                    },
                                ]),
                                description: Some(
                                    vec![String::from("This function has parameters, yeah")],
                                ),
                            }),
                        ]),
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

        #[cfg(test)]
        mod extract_entities {
            use std::path::Path;
            use thinlinelib::analysis::Analysis;
            use thinlinelib::entity::{Argument, Entity, EntityType, Function};
            use thinlinelib::language_type::Python;

            #[test]
            fn extract_entities() {
                let analysis: Analysis<Python> = Analysis::new();
                let py_test_src_path = Path::new("tests")
                    .join("testdata")
                    .join("python_sources")
                    .join("language_type");
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

                let functions = entities[0].functions();
                assert!(functions.is_some());
                assert_eq!(functions.unwrap().len(), 1);

                let index_option = entities[0].clone();
                assert!(index_option.entities.is_some());

                let index_entities = index_option.entities.unwrap();
                assert_eq!(index_entities.len(), 2);

                assert_eq!(
                    index_entities[1],
                    EntityType::Entity(Entity {
                        name: String::from("class1"),
                        entities: Some(vec![
                            EntityType::Function(Function {
                                name: String::from("test_float"),
                                return_type: None,
                                arguments: Some(vec![
                                    Argument {
                                        name: String::from("float1"),
                                        atype: None,
                                        value: None,
                                    },
                                    Argument {
                                        name: String::from("float2"),
                                        atype: None,
                                        value: None,
                                    },
                                ]),
                                description: Some(vec![
                                    String::from("#TL_TESTCASE(check_if_sum_works)"),
                                    String::from(
                                        "#TL_EQ[TL_FCT(float1: 4.2, float2: 3.2) => 7.4]"
                                    ),
                                    String::from("#!TL_TESTCASE"),
                                ]),
                            }),
                            EntityType::Function(Function {
                                name: String::from("test_nodoc"),
                                return_type: None,
                                arguments: None,
                                description: None,
                            }),
                            EntityType::Function(Function {
                                name: String::from("test_str"),
                                return_type: None,
                                arguments: Some(vec![
                                    Argument {
                                        name: String::from("str1"),
                                        atype: None,
                                        value: None,
                                    },
                                    Argument {
                                        name: String::from("str2"),
                                        atype: None,
                                        value: None,
                                    },
                                ]),
                                description: Some(vec![
                                    String::from(
                                        "#TL_TESTCASE(check_if_str_concat_works)"
                                    ),
                                    String::from(
                                        "#TL_EQ[TL_FCT(str1: \'bla\', str2: \'blub\') => \'blablub\']"
                                    ),
                                    String::from("#!TL_TESTCASE"),
                                ]),
                            }),
                        ]),
                    })
                );
            }
        }
    }
}
