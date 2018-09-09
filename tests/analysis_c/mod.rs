use thinlinelib::entity::{Argument, EntityType, Function};

lazy_static! {
#[derive(Debug)]
    pub static ref ANALYSIS1_RESULT: Option<Vec<EntityType>> = Some(vec![
        EntityType::Function(Function {
            name: String::from("test_int_no1"),
            return_type: Some(String::from("int")),
            arguments: Some(vec![
                Argument {
                    name: String::from("no1"),
                    atype: Some(String::from("int")),
                    value: None,
                },
                Argument {
                    name: String::from("no2"),
                    atype: Some(String::from("int")),
                    value: None,
                },
            ]),
            description: Some(vec![
                String::from("#TL_TESTCASE(Source1::CheckIfSumWorks)"),
                String::from("int test_no = 2;"),
                String::from(
                    "#TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]"
                ),
                String::from("#TL_EQ[TL_FCT(no1: 5, no2: 2) => 7]"),
                String::from("EXPECT_EQ(11, test_int_no1(9, 2));"),
                String::from("#!TL_TESTCASE"),
            ]),
        }),
        EntityType::Function(Function {
            name: String::from("test_ptr"),
            return_type: Some(String::from("int")),
            arguments: Some(vec![
                Argument {
                    name: String::from("no1"),
                    atype: Some(String::from("const int *const")),
                    value: None,
                },
                Argument {
                    name: String::from("no2"),
                    atype: Some(String::from("const int *const")),
                    value: None,
                },
            ]),
            description: Some(vec![
                String::from("#TL_TESTCASE(Source1::TestPtr)"),
                String::from("int test_no = 2;"),
                String::from("int test_no2 = 5;"),
                String::from(
                    "#TL_EQ[TL_FCT(no1: &test_no, no2: &test_no2) => 7]"
                ),
                String::from("#!TL_TESTCASE"),
            ]),
        }),
        EntityType::Function(Function {
            name: String::from("test_empty_fct"),
            return_type: Some(String::from("int")),
            arguments: None,
            description: Some(vec![
                String::from("#TL_TESTCASE(Source1::EmptyFct)"),
                String::from("#TL_EQ[TL_FCT() => 7]"),
                String::from("#TL_NE[TL_FCT() => 4]"),
                String::from("#!TL_TESTCASE"),
            ]),
        }),
        EntityType::Function(Function {
            name: String::from("main"),
            return_type: Some(String::from("int")),
            arguments: Some(vec![
                Argument {
                    name: String::from("argc"),
                    atype: Some(String::from("const int")),
                    value: None,
                },
                Argument {
                    name: String::from("argv"),
                    atype: Some(String::from("char *const []")),
                    value: None,
                },
            ]),
            description: Some(vec![String::from("This function has parameters, yeah")]),
        }),
    ]);
}
