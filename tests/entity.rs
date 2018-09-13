extern crate thinlinelib;

#[cfg(test)]
mod entity {

    #[cfg(test)]
    mod argument {
        use thinlinelib::entity::Argument;

        #[test]
        fn new() {
            let argument = Argument::new("arg", Some("std::string"));

            assert_eq!(argument.name, String::from("arg"));
            assert_eq!(argument.atype, Some(String::from("std::string")));
            assert!(argument.value.is_none());
        }

        #[test]
        fn set_value() {
            let mut argument = Argument::new("arg", Some("std::string"));
            argument.set_value("FirstArg");

            assert_eq!(argument.value, Some(String::from("FirstArg")));
        }
    }

    #[cfg(test)]
    mod function {
        use thinlinelib::entity::{Argument, Function};

        #[test]
        fn new() {
            let function = Function::new("fct");

            assert_eq!(function.name, String::from("fct"));
            assert!(function.return_type.is_none());
            assert!(function.arguments.is_none());
            assert!(function.description.is_none());
        }

        #[test]
        fn set_return_type() {
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

        #[test]
        fn set_description() {
            let mut fct = Function::new("fct");

            {
                fct.set_description(
                    "
                    **this
                    is
                    a

                    multiline
                    **
                    //comment",
                );

                assert!(fct.description.is_some());
                let fct_desc = fct.description.unwrap();

                assert_eq!(fct_desc.len(), 5);
                assert_eq!(fct_desc[0], "this");
                assert_eq!(fct_desc[1], "is");
                assert_eq!(fct_desc[2], "a");
                assert_eq!(fct_desc[3], "multiline");
                assert_eq!(fct_desc[4], "comment");
            }
        }

        #[test]
        fn set_arguments() {
            let mut fct = Function::new("fct");

            {
                fct.set_arguments(&vec![]);
                assert!(fct.arguments.is_none());
            }

            {
                let arg1 = Argument::new("arg1", Some("std::string"));
                let arg2 = Argument::new("arg2", Some("std::uint32"));
                fct.set_arguments(&vec![arg1, arg2]);

                assert!(fct.arguments.is_some());
                let fct_args = fct.arguments.unwrap();

                assert_eq!(fct_args.len(), 2);
                assert_eq!(fct_args[0].name, "arg1");
                assert_eq!(fct_args[1].name, "arg2");
            }
        }
    }

    #[cfg(test)]
    mod enumeration {
        use thinlinelib::entity::{Argument, Enum};

        #[test]
        fn new() {
            let enumeration = Enum::new("enum");

            assert_eq!(enumeration.name, String::from("enum"));
            assert!(enumeration.arguments.is_none());
            assert!(enumeration.etype.is_none());
        }

        #[test]
        fn set_arguments() {
            let mut enumeration = Enum::new("enum");

            {
                enumeration.set_arguments(&vec![]);
                assert!(enumeration.arguments.is_none());
            }

            {
                let args = vec![
                    Argument::new("Zero", Some("0")),
                    Argument::new("Two", Some("2")),
                ];
                enumeration.set_arguments(&args);

                assert!(enumeration.arguments.is_some());
                let enum_args = enumeration.arguments.unwrap();

                assert_eq!(enum_args.len(), 2);
                assert_eq!(enum_args[0].name, "Zero");
                assert_eq!(enum_args[1].name, "Two");
            }
        }

        #[test]
        fn push_argument() {
            let mut enumeration = Enum::new("enum");
            assert!(enumeration.arguments.is_none());

            enumeration.push_argument(Argument::new("arg", Some("uint32")));

            assert!(enumeration.arguments.is_some());
            let mut enum_args = enumeration.arguments.clone().unwrap();

            assert_eq!(enum_args.len(), 1);

            enumeration.push_argument(Argument::new("new_arg", Some("uint64")));
            enum_args = enumeration.arguments.unwrap();

            assert_eq!(enum_args.len(), 2);
        }
    }

    #[cfg(test)]
    mod entity {
        use thinlinelib::entity::{Entity, EntityType, Enum, Function};

        #[test]
        fn new() {
            let entity = Entity::new("ent");

            assert_eq!(entity.name, String::from("ent"));
            assert!(entity.entities.is_none());
        }

        #[test]
        fn add_entity() {
            {
                let mut entity = Entity::new("ent");
                assert!(entity.entities.is_none());

                let entity_type = EntityType::Entity(Entity::new("inner_entity"));
                assert!(entity.add_entity::<Entity>(entity_type).is_some());

                let fct = EntityType::Function(Function::new("fct"));
                assert!(entity.add_entity::<Function>(fct).is_some());

                assert!(entity.entities.is_some());

                let inner_entities = entity.entities.unwrap();
                assert_eq!(inner_entities.len(), 2);
            }

            {
                let mut entity = Entity::new("ent");
                assert!(entity.entities.is_none());

                let enumeration = EntityType::Enum(Enum::new("enumeration"));
                let ret = entity.add_entity::<Enum>(enumeration);

                assert!(ret.is_some());
                assert_eq!(ret.unwrap().name, "enumeration");
            }
        }

        #[test]
        fn functions() {
            let mut entity = Entity::new("ent");

            {
                assert!(entity.functions().is_none());
            }

            {
                let fct1 = EntityType::Function(Function::new("fct1"));
                assert!(entity.add_entity::<Function>(fct1).is_some());

                let fct2 = EntityType::Function(Function::new("fct2"));
                assert!(entity.add_entity::<Function>(fct2).is_some());

                let functions_option = entity.functions();

                assert!(functions_option.is_some());
                let functions = functions_option.unwrap();

                assert_eq!(functions.len(), 2);
                assert_eq!(functions[0].name, "fct1");
                assert_eq!(functions[1].name, "fct2");
            }
        }
    }
}
