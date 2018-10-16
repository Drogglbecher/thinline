extern crate thinlinelib;
extern crate yaml_rust;

#[cfg(test)]
mod value_parser {
    use std::fs::read_to_string;
    use std::path::Path;
    use thinlinelib::value_parser::ValueParser;
    use yaml_rust::YamlLoader;

    #[test]
    fn parse_yaml_config_bool_succeed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        let build_log = yml_param.unwrap().get_bool(&["build_script", "log"], false);
        assert_eq!(build_log, true);
    }

    #[test]
    fn parse_yaml_config_bool_failed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        let build_log = yml_param
            .unwrap()
            .get_bool(&["build_script", "none_existing"], true);
        assert_eq!(build_log, true);
    }

    #[test]
    fn parse_yaml_config_str_succeed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        let test_env = yml_param.unwrap().get_str(&["test_env"]);
        assert_eq!(test_env, Some("ctest"));

        let language = yml_param.unwrap().get_str(&["language"]);
        assert_eq!(language, Some("c"));
    }

    #[test]
    fn parse_yaml_config_str_failed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        {
            let test_env = yml_param.unwrap().get_str(&["none_existing"]);
            assert!(test_env.is_none());
        }

        {
            let test_env = yml_param.unwrap().get_str(&[]);
            assert!(test_env.is_none());
        }

        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config4.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        {
            let test_env = yml_param.unwrap().get_str(&["test_env"]);
            assert!(test_env.is_none());
        }
    }

    #[test]
    fn parse_yaml_config_str_vec_failed() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        {
            let test_env = yml_param.unwrap().get_str_vec(&["include_dirs"]);
            assert_eq!(test_env, Some(vec!["include", "src"]));
        }

        {
            let test_env = yml_param.unwrap().get_str_vec(&["none_existing"]);
            assert!(test_env.is_none());
        }

        {
            let test_env = yml_param.unwrap().get_str_vec(&[]);
            assert!(test_env.is_none());
        }
    }
}
