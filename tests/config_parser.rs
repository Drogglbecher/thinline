extern crate thinlinelib;
extern crate yaml_rust;

#[cfg(test)]
mod project_parameters {

    #[cfg(test)]
    mod should_succeed {
        use std::path::Path;
        use thinlinelib::config_parser::ProjectParameters;

        #[test]
        fn when_yaml_is_valid() {
            let test_yml_path = Path::new("tests")
                .join("testdata")
                .join("config")
                .join("config1.yml");
            let parameters_res = ProjectParameters::parse(test_yml_path.to_str().unwrap());

            assert!(parameters_res.is_ok());

            let parameters = parameters_res.unwrap();

            assert_eq!(parameters.test_env, "ctest");

            assert!(parameters.build_script.windows.is_none());
            assert!(parameters.build_script.linux.is_some());
            assert_eq!(parameters.build_script.linux.unwrap().len(), 4);

            assert_eq!(
                parameters.include_dirs,
                Some(vec![String::from("include"), String::from("src")])
            );

            assert_eq!(
                parameters.lib_paths,
                Some(vec![String::from("build/libtest.so")])
            );
        }
    }

    mod should_fail {
        use std::path::Path;
        use thinlinelib::config_parser::ProjectParameters;

        #[test]
        fn when_yaml_is_not_existing() {
            let test_yml_path = Path::new("tests")
                .join("testdata")
                .join("config")
                .join("non_existing.yml");
            let parameters_res = ProjectParameters::parse(test_yml_path.to_str().unwrap());

            assert!(parameters_res.is_err());
        }

        #[test]
        fn when_yaml_is_empty() {
            let test_yml_path = Path::new("tests")
                .join("testdata")
                .join("config")
                .join("config3.yml");
            let parameters_res = ProjectParameters::parse(test_yml_path.to_str().unwrap());

            assert!(parameters_res.is_err());
        }

        #[test]
        fn when_no_test_env_could_be_parsed() {
            let test_yml_path = Path::new("tests")
                .join("testdata")
                .join("config")
                .join("config2.yml");
            let parameters_res = ProjectParameters::parse(test_yml_path.to_str().unwrap());

            assert!(parameters_res.is_err());
        }
    }
}
