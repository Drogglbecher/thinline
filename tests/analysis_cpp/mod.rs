use thinlinelib::entity::{Argument, Entity, EntityType, Function};

lazy_static! {
#[derive(Debug)]
    pub static ref ANALYSIS1_RESULT: Option<Vec<EntityType>> = Some(vec![
        EntityType::Entity(
            Entity {
                name: String::from("ns1"),
                entities: Some(
                    vec![
                        EntityType::Entity(
                            Entity {
                                name: String::from("c1"),
                                entities: Some(
                                    vec![
                                        EntityType::Function(
                                            Function {
                                                name: String::from("add_two_numbers"),
                                                return_type: Some(
                                                    String::from("unsigned int")
                                                ),
                                                arguments: Some(
                                                    vec![
                                                        Argument {
                                                            name: String::from("no1"),
                                                            atype: Some(
                                                                String::from("unsigned int")
                                                            ),
                                                            value: None
                                                        },
                                                        Argument {
                                                            name: String::from("no2"),
                                                            atype: Some(
                                                                String::from("unsigned int")
                                                            ),
                                                            value: None
                                                        }
                                                    ]
                                                ),
                                                description: Some(
                                                    vec![
                                                        String::from("#TL_TESTCASE(c1::AddTwoNumbers)"),
                                                        String::from("unsigned int no1 = 5;"),
                                                        String::from("#TL_EQ[this->class_inst->TL_FCT(no1: no1, no2: 10) => 15]"),
                                                        String::from("#TL_LT[this->class_inst->TL_FCT(no1: no1, no2: 10) => 30]"),
                                                        String::from("#!TL_TESTCASE")
                                                    ]
                                                )
                                            }
                                        ),
                                        EntityType::Function(
                                            Function {
                                                name: String::from("c1"),
                                                return_type: Some(
                                                    String::from("void")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        ),
                                        EntityType::Function(
                                            Function {
                                                name: String::from("~c1"),
                                                return_type: Some(
                                                    String::from("void")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        )
                                    ]
                                )
                            }
                        ),
                        EntityType::Entity(
                            Entity {
                                name: String::from("c2"),
                                entities: Some(
                                    vec![
                                        EntityType::Function(
                                            Function {
                                                name: String::from("c2"),
                                                return_type: Some(
                                                    String::from("void")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        ),
                                        EntityType::Function(
                                            Function {
                                                name: String::from("~c2"),
                                                return_type: Some(
                                                    String::from("void")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        ),
                                        EntityType::Function(
                                            Function {
                                                name: String::from("add_three_numbers"),
                                                return_type: Some(
                                                    String::from("unsigned int")
                                                ),
                                                arguments: Some(
                                                    vec![
                                                        Argument {
                                                            name: String::from("no1"),
                                                            atype: Some(
                                                                String::from("unsigned int")
                                                            ),
                                                            value: None
                                                        },
                                                        Argument {
                                                            name: String::from("no2"),
                                                            atype: Some(
                                                                String::from("unsigned int")
                                                            ),
                                                            value: None
                                                        },
                                                        Argument {
                                                            name: String::from("no3"),
                                                            atype: Some(
                                                                String::from("unsigned int")
                                                            ),
                                                            value: None
                                                        }
                                                    ]
                                                ),
                                                description: Some(
                                                    vec![
                                                        String::from("#TL_TESTCASE(c2::AddThreeNumbers)"),
                                                        String::from("unsigned int no1 = 5;"),
                                                        String::from("unsigned int no2 = 10;"),
                                                        String::from("#TL_EQ[this->class_inst->TL_FCT(no1: no1, no2: no2, no3: 5) => 20]"),
                                                        String::from("#!TL_TESTCASE")
                                                    ]
                                                )
                                            }
                                        )
                                    ]
                                )
                            }
                        )
                    ]
                )
            }
        ),
        EntityType::Entity(
            Entity {
                name: String::from("ns2"),
                entities: Some(
                    vec![
                        EntityType::Entity(
                            Entity {
                                name: String::from("c3"),
                                entities: Some(
                                    vec![
                                        EntityType::Function(
                                            Function {
                                                name: String::from("c3"),
                                                return_type: Some(
                                                    String::from("void")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        ),
                                        EntityType::Function(
                                            Function {
                                                name: String::from("~c3"),
                                                return_type: Some(
                                                    String::from("void")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        ),
                                        EntityType::Function(
                                            Function {
                                                name: String::from("return5"),
                                                return_type: Some(
                                                    String::from("unsigned int")
                                                ),
                                                arguments: None,
                                                description: None
                                            }
                                        )
                                    ]
                                )
                            }
                        )
                    ]
                )
            }
        )
    ]);
}
