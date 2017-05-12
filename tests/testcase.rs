extern crate thinlinelib;

use std::path::Path;
use thinlinelib::function::FunctionCall;
use thinlinelib::testcase::*;
use thinlinelib::parameters::EnvironmentParameters;

static TEST_CASE: &str = "# TL_TESTCASE(<Class>::<Name>)";
static TEST_CASE_NO_CLASS: &str = "# TL_TESTCASE(::<Name>)";
static TEST_CASE_NO_REGEX_MATCH: &str = "# TL_TESTCASE(<Name>)";

static TEST_CONTENT: &'static [&str] = &[
    "#TL_EQ[TL_FCT(no1: 2, no2: 5) => 7]",
    "EXPECT_EQ(11, test_int_fct(9, 2));",
];
static TEST_CONTENT_NO_VALUE: &'static [&str] = &["#TL_EQ[TL_FCT(no1) => 7]"];
static TEST_CONTENT_NO_REGEX_MATCH: &'static [&str] = &["#TL_EQ[TL_FCT(no1) == 7]"];
static TEST_CONTENT_EMPTY: &'static [&str] = &["#TL_EQ[TL_FCT() => 7]"];
static TEST_CONTENT_WRONG_SYNTAX: &'static [&str] = &[
    "#TL_EQ[TL_FCT(no1: 2, no2: 5,) => 7]",
    "#TL_EQ[TL_FCT(,no1: 2, no2: 5) => 7]",
    "#TL_EQ[TL_FCT(no1:, no2: 5) => 7]",
    "#TL_EQ[TL_FCT(no1:, no2: 5) =>]",
];


#[test]
fn parse_test_case() {
    let tc_raw = TestCase::new(TEST_CASE);
    assert!(tc_raw.is_ok());
    let tc = tc_raw.unwrap();
    assert_eq!(tc.class_str, "<Class>");
    assert_eq!(tc.name, "<Name>");
}

#[test]
fn parse_test_case_no_class() {
    let tc_raw = TestCase::new(TEST_CASE_NO_CLASS);
    assert!(tc_raw.is_err());
}

#[test]
fn parse_test_case_no_regex_match() {
    let tc_raw = TestCase::new(TEST_CASE_NO_REGEX_MATCH);
    assert!(tc_raw.is_err());
}

#[test]
fn process_test_content() {
    let tc_raw = TestCase::new(TEST_CASE);
    assert!(tc_raw.is_ok());
    let mut tc = tc_raw.unwrap();

    let env_parameters_raw = EnvironmentParameters::parse(
        Path::new("stubs").to_str().unwrap(),
        Path::new("stubs")
            .join("environment")
            .join("env_stubs.yml")
            .to_str()
            .unwrap(),
        "google_test",
    );
    assert!(env_parameters_raw.is_ok());

    let mut fct_call = FunctionCall::new("test_fct", "(no1, no2)");
    let mut fct_call_no_value = FunctionCall::new("test_fct", "(no1)");
    let mut fct_call_empty = FunctionCall::new("test_fct", "()");

    let mut env_parameters = env_parameters_raw.unwrap();
    {
        let test_content: Vec<String> = TEST_CONTENT.iter().map(|tc| tc.to_string()).collect();
        assert!(
            tc.process(&test_content, &mut fct_call, &mut env_parameters)
                .is_ok()
        );

        assert_eq!(tc.content.len(), 2);
        assert_eq!(tc.content[0], "EXPECT_EQ(test_fct(2, 5), 7);");
        assert_eq!(tc.content[1], "EXPECT_EQ(11, test_int_fct(9, 2));");
    }

    {
        let test_content: Vec<String> =
            TEST_CONTENT_EMPTY.iter().map(|tc| tc.to_string()).collect();
        assert!(
            tc.process(&test_content, &mut fct_call_empty, &mut env_parameters)
                .is_ok()
        );
    }

    {
        let test_content: Vec<String> = TEST_CONTENT_NO_VALUE
            .iter()
            .map(|tc| tc.to_string())
            .collect();
        assert!(
            tc.process(&test_content, &mut fct_call_no_value, &mut env_parameters)
                .is_err()
        );
    }

    {
        let test_content: Vec<String> = TEST_CONTENT_NO_REGEX_MATCH
            .iter()
            .map(|tc| tc.to_string())
            .collect();
        assert!(
            tc.process(&test_content, &mut fct_call, &mut env_parameters)
                .is_err()
        );
    }

    {
        let test_content: Vec<String> = TEST_CONTENT.iter().map(|tc| tc.to_string()).collect();
        assert!(
            tc.process(&test_content, &mut fct_call_no_value, &mut env_parameters)
                .is_err()
        );
    }

    {
        let test_content: Vec<String> = TEST_CONTENT.iter().map(|tc| tc.to_string()).collect();
        assert!(
            tc.process(&test_content, &mut fct_call_empty, &mut env_parameters)
                .is_err()
        );
    }
    {
        let test_content: Vec<Vec<String>> = TEST_CONTENT_WRONG_SYNTAX
            .iter()
            .map(|tc| vec![tc.to_string()])
            .collect();
        for wrong_content in test_content {
            assert!(
                tc.process(&wrong_content, &mut fct_call, &mut env_parameters)
                    .is_err()
            );
        }
    }

}
