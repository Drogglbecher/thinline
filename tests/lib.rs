extern crate thinlinelib;

#[cfg(test)]
mod lib {
    use std::path::Path;
    use thinlinelib::{
        language_type::{Cpp, Python, C},
        Thinline,
    };

    #[test]
    fn analyze() {
        // Test C analysis
        {
            let mut thinline: Thinline<C> = Thinline::new();
            let path = Path::new("examples").join("c_project");

            assert!(thinline.analyze(&path, ".thinline.yml", true).is_ok());
        }

        // Test C++ analysis
        {
            let mut thinline: Thinline<Cpp> = Thinline::new();
            let path = Path::new("examples").join("cpp_project");

            assert!(thinline.analyze(&path, ".thinline.yml", true).is_ok());
        }

        // Test Python analysis
        {
            let mut thinline: Thinline<Python> = Thinline::new();
            let path = Path::new("examples").join("python_project");

            assert!(thinline.analyze(&path, ".thinline.yml", true).is_ok());
        }
    }
}
