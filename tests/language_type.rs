extern crate thinlinelib;

#[cfg(test)]
mod language_type {

    #[cfg(test)]
    mod c {
        use std::path::Path;
        use std::panic;
        use thinlinelib::analysis::Analysis;
        use thinlinelib::language_type::{C, LanguageType};

        #[test]
        fn analyse_clang_entity() {
            let analysis: Analysis<C> = Analysis::new();
            let c_test_src_path = Path::new("tests").join("testdata").join("c_sources").join("language_type");
            assert!(analysis.collect_sources(&c_test_src_path, &[String::from(".")]).is_ok());

            assert!(C::extract_functions(&analysis).is_ok());

            let project_files = analysis.project_files();

            assert_eq!(project_files.len(), 1);

            let entities = project_files[0].entities();
            assert_eq!(entities.len(), 1);

            let index_option = entities[0].clone();
            assert!(index_option.entities.is_some());

            let functions = index_option.functions().unwrap();
            assert_eq!(functions.len(), 4);

            assert_eq!(functions[0].name, "test_int_no1");
            assert_eq!(functions[1].name, "test_ptr");
            assert_eq!(functions[2].name, "test_empty_fct");
            assert_eq!(functions[3].name, "main");
        }
    }
}
