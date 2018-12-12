extern crate thinlinelib;
extern crate yaml_rust;
#[cfg(test)]
mod value_parser {

    static CLASS_STUB: &str = "class #TEST_CLASSTest : public ::testing::Test {

    public:
        #TEST_CLASSTest() {#CONSTRUCTOR_CONTEXT
        }

        virtual ~#TEST_CLASSTest() {#DESTRUCTOR_CONTEXT
        }

    protected:
        virtual void SetUp() {#SET_UP_CONTEXT
        }

        virtual void TearDown() {#TEAR_DOWN_CONTEXT
        }

#CLASS_CONTEXT
};
";

    static FUNCTION_STUB: &str = "TEST_F(#TEST_CLASSTest, #TEST_NAME) {
    #TEST_CONTEXT
}
";

    use std::fs::read_to_string;
    use std::path::Path;
    use thinlinelib::value_parser::ValueParser;
    use yaml_rust::YamlLoader;

    #[test]
    fn get_bool() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        // Should succeed
        {
            let build_log = yml_param.unwrap().get_bool(&["build_script", "log"], false);
            assert_eq!(build_log, true);
        }

        // Should fail
        {
            let build_log = yml_param
                .unwrap()
                .get_bool(&["build_script", "none_existing"], true);
            assert_eq!(build_log, true);
        }
    }

    #[test]
    fn get_str() {
        let yml_path = Path::new("tests")
            .join("testdata")
            .join("config")
            .join("config1.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        // Should succeed
        {
            let test_env = yml_param.unwrap().get_str(&["test_env"]);
            assert_eq!(test_env, Some("ctest"));

            let language = yml_param.unwrap().get_str(&["language"]);
            assert_eq!(language, Some("c"));
        }

        // Should fail
        {
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
    }

    #[test]
    fn get_str_or_file_content() {
        let yml_path = Path::new("stubs").join("environment").join("env_stubs.yml");
        let yml_params =
            YamlLoader::load_from_str(read_to_string(yml_path).unwrap().as_str()).unwrap();
        let yml_param = yml_params.get(0);

        // Should succeed reading raw
        {
            {
                let function = yml_param.unwrap().get_str_or_file_content(
                    &["google_test", "function"],
                    &Path::new("stubs").join("environment"),
                    "stub",
                );
                assert_eq!(function, Some(FUNCTION_STUB));
            }
        }

        // Should succeed reading file with valid extension
        {
            {
                let class = yml_param.unwrap().get_str_or_file_content(
                    &["google_test", "class"],
                    &Path::new("stubs").join("environment"),
                    "stub",
                );
                assert_eq!(class, Some(CLASS_STUB));
            }
        }

        // Should fail reading file with invalid extension
        {
            {
                let class = yml_param.unwrap().get_str_or_file_content(
                    &["google_test", "class"],
                    &Path::new("stubs").join("environment"),
                    "xyz",
                );
                assert_eq!(class, Some("google_test/class.stub"));
            }
        }

        // Should fail for non existing key path
        {
            {
                let class = yml_param.unwrap().get_str_or_file_content(
                    &["google_test", "non_existing"],
                    &Path::new("stubs").join("environment"),
                    "xyz",
                );
                assert_eq!(class, None);
            }
        }
    }

    #[test]
    fn get_str_vec() {
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
