use thinlinelib::entity::{Argument, Entity, EntityType, Function};

lazy_static! {
#[derive(Debug)]
pub static ref ANALYSIS1_RESULT: Option<Vec<EntityType>> = Some(vec![
    EntityType::Function(Function {
        name: String::from("test_int_no1"),
        return_type: None,
        arguments: Some(vec![
            Argument {
                name: String::from("no1"),
                atype: None,
                value: None,
            },
            Argument {
                name: String::from("no2"),
                atype: None,
                value: None,
            },
        ]),
        description: Some(vec![
            String::from("#TL_TESTCASE(check_if_sum_works)"),
            String::from("int test_no = 2;"),
            String::from("#TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]"),
            String::from("EXPECT_EQ(11, test_int_no1(9, 2));"),
            String::from("#!TL_TESTCASE"),
        ]),
    }),
    EntityType::Entity(Entity {
        name: String::from("class1"),
        entities: Some(vec![
            EntityType::Function(Function {
                name: String::from("test_float"),
                return_type: None,
                arguments: Some(vec![
                    Argument {
                        name: String::from("float1"),
                        atype: None,
                        value: None,
                    },
                    Argument {
                        name: String::from("float2"),
                        atype: None,
                        value: None,
                    },
                ]),
                description: Some(vec![
                    String::from("#TL_TESTCASE(check_if_sum_works)"),
                    String::from(
                        "#TL_EQ[TL_FCT(float1: 4.2, float2: 3.2) => 7.4]"
                    ),
                    String::from("#!TL_TESTCASE"),
                ]),
            }),
            EntityType::Function(Function {
                name: String::from("test_nodoc"),
                return_type: None,
                arguments: None,
                description: None,
            }),
            EntityType::Function(Function {
                name: String::from("test_str"),
                return_type: None,
                arguments: Some(vec![
                    Argument {
                        name: String::from("str1"),
                        atype: None,
                        value: None,
                    },
                    Argument {
                        name: String::from("str2"),
                        atype: None,
                        value: None,
                    },
                ]),
                description: Some(vec![
                    String::from(
                        "#TL_TESTCASE(check_if_str_concat_works)"
                    ),
                    String::from(
                        "#TL_EQ[TL_FCT(str1: \'bla\', str2: \'blub\') => \'blablub\']"
                    ),
                    String::from("#!TL_TESTCASE"),
                ]),
            }),
        ]),
    })
]);
}
