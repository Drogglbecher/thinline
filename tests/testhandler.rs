extern crate thinlinelib;

use thinlinelib::testhandler::*;

static COMMENT_WITH_TL_HOOKS: &'static [&str] = &[
    "# TL_TESTCLASS",
    "TL_NAME: Source1",
    "TL_SETUP_CONTEXT:",
    "conts u32 no1 = 2;",
    "# !TL_TESTCLASS",
    "# TL_TESTCASE(Source1::CheckIfSumWorks)",
    "TL_EQ:",
    "TL_FCT(no1: 2, no2: 5) => 7",
    "TL_RAW:",
    "EXPECT_EQ(11, test_int_fct(9, 2));",
    "# !TL_TESTCASE",
];

#[test]
fn check_tl_hooks() {
    let test_content: Vec<String> = COMMENT_WITH_TL_HOOKS
        .iter()
        .map(|tc| tc.to_string())
        .collect();
    let th_hc_raw = TestHandler::check_for_tl_hooks(&test_content, "#");
    assert!(th_hc_raw.is_ok());
    let th_hc = th_hc_raw.unwrap();
    assert_eq!(th_hc.len(), 2);
    assert_eq!(th_hc[0].hook_type, ThinlineHookType::TestClass);
    assert_eq!(th_hc[0].context, &test_content[0..4]);
    assert_eq!(th_hc[1].hook_type, ThinlineHookType::TestCase);
    assert_eq!(th_hc[1].context, &test_content[5..10]);
}
