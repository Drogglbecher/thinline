extern crate log;
extern crate thinlinelib;

use log::LogLevel;
use std::path::Path;
use thinlinelib::Thinline;

#[test]
fn test_main() {

    let mut thinline = Thinline::new();
    let thinline_cfg_file = ".thinline.yml";

    assert!(thinline.init_logging(LogLevel::Trace).is_ok());
    assert!(thinline.init_environment().is_ok());

    {
        let input_dir = Path::new("examples").join("c_project");
        let input_dir_str = input_dir.to_str().unwrap();

        assert!(
            thinline
                .collect_sources(input_dir.clone(), thinline_cfg_file)
                .is_ok()
        );
        assert!(thinline.extract_fct_symbols().is_ok());
        assert!(thinline.execute_build_steps(input_dir_str).is_ok());
        assert!(thinline.synthesize_general_header(input_dir_str).is_ok());
        assert!(thinline.synthesize_testcases(input_dir_str).is_ok());
        assert!(thinline.synthesize_testfiles().is_ok());
        assert!(thinline.synthesize_gpp_args(input_dir_str).is_ok());
        assert!(thinline.reconstruct_fn().is_ok());
    }

    {
        let input_dir = Path::new("examples").join("cpp_project");
        let input_dir_str = input_dir.to_str().unwrap();

        assert!(
            thinline
                .collect_sources(input_dir.clone(), thinline_cfg_file)
                .is_ok()
        );
        assert!(thinline.extract_fct_symbols().is_ok());
        assert!(thinline.execute_build_steps(input_dir_str).is_ok());
        assert!(thinline.synthesize_general_header(input_dir_str).is_ok());
        assert!(thinline.synthesize_testcases(input_dir_str).is_ok());
        assert!(thinline.synthesize_testfiles().is_ok());
        assert!(thinline.reconstruct_fn().is_ok());
    }

    {
        let input_dir = "examples/collect_files.rs";
        let thinline_cfg_file = "examples/c_project/.thinline.yml";
        assert!(
            thinline
                .collect_sources(input_dir, thinline_cfg_file)
                .is_err()
        );
    }

    {
        let input_dir = "stubs";
        let thinline_cfg_file = "examples/c_project/.thinline.yml";
        assert!(
            thinline
                .collect_sources(input_dir, thinline_cfg_file)
                .is_err()
        );
    }
}
