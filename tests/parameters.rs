extern crate thinlinelib;

use thinlinelib::parameters::*;
use std::path::Path;

static ENV_PARAM_GOOGLE_TEST_FILE_STUB: &str =
    include_str!("../stubs/environment/google_test/file.stub");
static ENV_PARAM_GOOGLE_TEST_CLASS_STUB: &str =
    include_str!("../stubs/environment/google_test/class.stub");
static ENV_PARAM_GOOGLE_TEST_FCT_STUB: &str =
    include_str!("../stubs/environment/google_test/fct.stub");

#[test]
fn test_project_parameters_parsing_ok() {

    let test_yml_path = Path::new("tests").join("files");

    {
        let parameters_res =
            ProjectParameters::parse(test_yml_path.join(".thinline.yml").to_str().unwrap());
        assert!(parameters_res.is_ok());

        let parameters = parameters_res.unwrap();
        assert!(parameters.include_dirs.is_some());
        assert!(parameters.build_script.linux.is_some());
        assert!(parameters.build_script.windows.is_none());

        assert_eq!(parameters.test_env, "ctest");

        let include_dirs = parameters.include_dirs.unwrap();
        assert_eq!(include_dirs.len(), 2);
        assert_eq!(include_dirs[0], "include");
        assert_eq!(include_dirs[1], "src");

        let build_script = parameters.build_script.linux.unwrap();
        assert_eq!(build_script.len(), 4);
        assert_eq!(build_script[0], "mkdir build");
        assert_eq!(build_script[1], "cd build");
        assert_eq!(build_script[2], "cmake ..");
        assert_eq!(build_script[3], "make");
        assert_eq!(parameters.lib_paths.len(), 1);
        assert_eq!(parameters.lib_paths[0], "build/libtest.so");
    }

    {
        let parameters_res =
            ProjectParameters::parse(test_yml_path.join(".thinline2.yml").to_str().unwrap());
        assert!(parameters_res.is_ok());

        let parameters = parameters_res.unwrap();
        assert_eq!(parameters.test_env, "ctest");
    }

    {
        assert!(
            ProjectParameters::parse(test_yml_path.join(".empty_vector.yml").to_str().unwrap())
                .is_ok()
        );
        assert!(
            ProjectParameters::parse(
                test_yml_path
                    .join(".thinline_no_include_dirs.yml")
                    .to_str()
                    .unwrap(),
            ).is_ok()
        );
        assert!(
            ProjectParameters::parse(
                test_yml_path
                    .join(".thinline_no_build_script.yml")
                    .to_str()
                    .unwrap(),
            ).is_ok()
        );
        assert!(
            ProjectParameters::parse(
                test_yml_path
                    .join(".thinline_corrupted_vec.yml")
                    .to_str()
                    .unwrap(),
            ).is_ok()
        );
    }
}

#[test]
fn test_project_parameters_parsing_panic() {
    let test_yml_path = Path::new("tests").join("files");
    assert!(
        ProjectParameters::parse(test_yml_path.join(".not_really_yml.yml").to_str().unwrap())
            .is_err()
    );
    assert!(
        ProjectParameters::parse(
            test_yml_path
                .join(".thinline_no_test_env.yml")
                .to_str()
                .unwrap(),
        ).is_err()
    );
    assert!(
        ProjectParameters::parse(
            test_yml_path
                .join(".thinline_no_lib_paths.yml")
                .to_str()
                .unwrap(),
        ).is_err()
    );
    assert!(
        ProjectParameters::parse(
            test_yml_path
                .join(".thinline_corrupted_str.yml")
                .to_str()
                .unwrap(),
        ).is_err()
    );
    assert!(
        ProjectParameters::parse(
            test_yml_path
                .join(".thinline_not_existing.yml")
                .to_str()
                .unwrap(),
        ).is_err()
    );
}

#[test]
fn test_env_parameters_parsing_ok() {

    let test_yml_path = Path::new("stubs");

    {
        let env_parameters_res = EnvironmentParameters::parse(
            test_yml_path.to_str().unwrap(),
            test_yml_path
                .join("environment")
                .join("env_stubs.yml")
                .to_str()
                .unwrap(),
            "google_test",
        );

        assert!(env_parameters_res.is_ok());

        let env_parameters = env_parameters_res.unwrap();
        assert_eq!(env_parameters.file_stub, ENV_PARAM_GOOGLE_TEST_FILE_STUB);
        assert_eq!(
            env_parameters.fct_sig,
            ENV_PARAM_GOOGLE_TEST_FCT_STUB.replace("    ", "")
        );
        assert_eq!(env_parameters.class_sig, ENV_PARAM_GOOGLE_TEST_CLASS_STUB);

        let test_fct_sig_res =
            TestFunctionSignature::parse(env_parameters.yml.as_str(), "google_test", "TL_EQ");
        assert!(test_fct_sig_res.is_ok());
        let test_fct_sig = test_fct_sig_res.unwrap();
        assert_eq!(test_fct_sig.stub, "EXPECT_EQ(//#ARG_0#/, //#ARG_1#/);");
        assert_eq!(test_fct_sig.inline_sig, "//#ARG_0#/ => //#ARG_1#/");
    }
}
