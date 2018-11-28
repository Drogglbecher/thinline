extern crate thinlinelib;

pub static MULTILINE_COMMENT: &str = "
**this
is
a

multiline
**
//comment";

#[cfg(test)]
mod analysis {
    use thinlinelib::analysis::{Argument, Enum, Function};
    use MULTILINE_COMMENT;

    #[test]
    fn argument() {
        // new
        {
            let argument = Argument::new("arg", Some("std::string"));

            assert_eq!(argument.name, String::from("arg"));
            assert_eq!(argument.atype, Some(String::from("std::string")));
            assert!(argument.value.is_none());
        }

        // set_value
        {
            let mut argument = Argument::new("arg", Some("std::string"));
            argument.set_value("FirstArg");

            assert_eq!(argument.value, Some(String::from("FirstArg")));
        }
    }

    #[test]
    fn function() {
        // new
        {
            let function = Function::new("fct");

            assert_eq!(function.name, String::from("fct"));
            assert!(function.return_type.is_none());
            assert!(function.arguments.is_empty());
            assert!(function.description.is_none());
        }

        // set_return_type
        {
            let mut fct = Function::new("fct");

            {
                assert!(fct.set_return_type("").is_ok());
                assert_eq!(fct.return_type, None);
            }

            {
                assert!(fct.set_return_type("int").is_ok());
                assert_eq!(fct.return_type, Some(String::from("int")));
            }
        }

        // set_description
        {
            let mut fct = Function::new("fct");

            {
                fct.set_description(MULTILINE_COMMENT);

                assert!(fct.description.is_some());
                let fct_desc = fct.description.unwrap().lines;

                assert_eq!(fct_desc.len(), 5);
                assert_eq!(fct_desc[0], "this");
                assert_eq!(fct_desc[1], "is");
                assert_eq!(fct_desc[2], "a");
                assert_eq!(fct_desc[3], "multiline");
                assert_eq!(fct_desc[4], "comment");
            }
        }

        // set_arguments()
        {
            let mut fct = Function::new("fct");

            {
                fct.set_arguments(&vec![]);
                assert!(fct.arguments.is_empty());
            }

            {
                let arg1 = Argument::new("arg1", Some("std::string"));
                let arg2 = Argument::new("arg2", Some("std::uint32"));
                fct.set_arguments(&vec![arg1, arg2]);

                assert_eq!(fct.arguments.len(), 2);
                assert_eq!(fct.arguments[0].name, "arg1");
                assert_eq!(fct.arguments[1].name, "arg2");
            }
        }
    }

    #[test]
    fn enumeration() {
        // new
        {
            let enumeration = Enum::new("enum");

            assert_eq!(enumeration.name, String::from("enum"));
            assert!(enumeration.arguments.is_empty());
            assert!(enumeration.etype.is_none());
        }

        // set_arguments
        {
            let mut enumeration = Enum::new("enum");

            {
                enumeration.set_arguments(&vec![]);
                assert!(enumeration.arguments.is_empty());
            }

            {
                let args = vec![
                    Argument::new("Zero", Some("0")),
                    Argument::new("Two", Some("2")),
                ];
                enumeration.set_arguments(&args);

                assert_eq!(enumeration.arguments.len(), 2);
                assert_eq!(enumeration.arguments[0].name, "Zero");
                assert_eq!(enumeration.arguments[1].name, "Two");
            }
        }

        // push_argument()
        {
            let mut enumeration = Enum::new("enum");
            assert!(enumeration.arguments.is_empty());

            enumeration
                .arguments
                .push(Argument::new("arg", Some("uint32")));
            assert_eq!(enumeration.arguments.len(), 1);

            enumeration
                .arguments
                .push(Argument::new("new_arg", Some("uint64")));
            assert_eq!(enumeration.arguments.len(), 2);
        }
    }

}
