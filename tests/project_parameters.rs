extern crate thinlinelib;
extern crate yaml_rust;

#[cfg(test)]
mod project_parameters {

    #[cfg(test)]
    mod should_succeed {
        use std::path::Path;
        use thinlinelib::project_parameters::ProjectParameters;

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

            assert!(parameters.build_script.windows.is_empty());
            assert_eq!(parameters.build_script.linux.len(), 2);

            assert_eq!(
                parameters.include_dirs,
                vec![String::from("include"), String::from("src")]
            );

            assert_eq!(parameters.lib_paths, vec![String::from("build/libtest.so")]);
        }
    }

    mod should_fail {
        use std::path::Path;
        use thinlinelib::project_parameters::ProjectParameters;

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
