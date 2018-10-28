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
            // Should succeed
            {
                let mut thinline: Thinline<C> =
                    Thinline::new(Path::new("examples").join("c_project"));
                assert!(thinline.analyze("thinline.yml", true).is_ok());
            }

            // Should fail
            {
                let mut thinline: Thinline<C> =
                    Thinline::new(Path::new("non_existing").join("path"));
                assert!(thinline.analyze("thinline.yml", true).is_err());
            }
        }

        // Test C++ analysis
        {
            // Should succeed
            {
                let mut thinline: Thinline<Cpp> =
                    Thinline::new(Path::new("examples").join("cpp_project"));
                assert!(thinline.analyze("thinline.yml", true).is_ok());
            }

            // Should fail
            {
                let mut thinline: Thinline<Cpp> =
                    Thinline::new(Path::new("non_existing").join("path"));
                assert!(thinline.analyze("thinline.yml", true).is_err());
            }
        }

        // Test Python analysis
        {
            // Should succeed with dir
            {
                let mut thinline: Thinline<Python> =
                    Thinline::new(Path::new("examples").join("python_project"));
                assert!(thinline.analyze("thinline.yml", true).is_ok());
            }

            // Should succeed with file
            {
                let mut thinline: Thinline<Python> =
                    Thinline::new(Path::new("examples").join("python_project").join("src1.py"));
                assert!(thinline.analyze("thinline.yml", false).is_ok());
            }

            // Should fail
            {
                let mut thinline: Thinline<Python> =
                    Thinline::new(Path::new("non_existing").join("path"));
                assert!(thinline.analyze("thinline.yml", true).is_err());
            }
        }
    }

    #[test]
    fn synthesize() {
        // Test C synthesis
        {
            let mut thinline: Thinline<C> = Thinline::new(Path::new("examples").join("c_project"));

            // Should succeed
            {
                assert!(
                    thinline
                        .synthesize(Path::new("stubs").join("environment"))
                        .is_ok()
                );
            }

            // Should fail
            {
                assert!(
                    thinline
                        .synthesize(Path::new("non_existing").join("path"))
                        .is_ok()
                );
            }
        }

        // Test C++ analysis
        {
            let mut thinline: Thinline<Cpp> =
                Thinline::new(Path::new("examples").join("cpp_project"));

            // Should succeed
            {
                assert!(
                    thinline
                        .synthesize(Path::new("stubs").join("environment"))
                        .is_ok()
                );
            }

            // Should fail
            {
                assert!(
                    thinline
                        .synthesize(Path::new("non_existing").join("path"))
                        .is_ok()
                );
            }
        }

        // Test Python analysis
        {
            let mut thinline: Thinline<Python> =
                Thinline::new(Path::new("examples").join("python_project"));

            // Should succeed
            {
                assert!(
                    thinline
                        .synthesize(Path::new("stubs").join("environment"))
                        .is_ok()
                );
            }

            // Should fail
            {
                assert!(
                    thinline
                        .synthesize(Path::new("non_existing").join("path"))
                        .is_ok()
                );
            }
        }
    }
}
