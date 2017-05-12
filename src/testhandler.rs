use error::*;

static TL_HOOK_ID_TESTCASE: &str = "TL_TESTCASE";
static TL_HOOK_ID_TESTCLASS: &str = "TL_TESTCLASS";

#[derive(Debug, PartialEq)]
/// Represents the kinds of hooks which can be interpreted by thinline.
pub enum ThinlineHookType {
    /// No hook type registered now
    NoType,

    /// A test class hook (TL_TESTCASE) was seen
    TestClass,

    /// A test case hook (TL_TESTCLASS) was seen
    TestCase,
}

pub struct HookedContext<'hc> {
    pub hook_type: ThinlineHookType,
    pub context: &'hc [String],
}

impl<'hc> HookedContext<'hc> {
    pub fn new(hook_type: ThinlineHookType, context: &'hc [String]) -> HookedContext<'hc> {
        HookedContext {
            hook_type: hook_type,
            context: context,
        }
    }
}

#[derive(Default)]
/// This is the glue holding together test cases, classes and the parsed functions.
pub struct TestHandler;

impl<'hc> TestHandler {
    /// Goes through the parsed function comments and searches for thinline hooks.
    /// When found the test class or case will be created.
    pub fn check_for_tl_hooks(
        doc_str: &'hc [String],
        identifier: &str,
    ) -> Result<Vec<HookedContext<'hc>>> {
        let mut tl_hook_type = ThinlineHookType::NoType;
        let mut tl_hook_idx = 0;
        let mut tl_hooked_context: Vec<HookedContext<'hc>> = Vec::new();
        for (i, doc_line) in doc_str.iter().enumerate() {
            if doc_line.trim_left().starts_with(identifier) {
                let mut tl_hook = doc_line.replace(identifier, "");
                tl_hook = String::from(tl_hook.trim_left());
                if tl_hook.starts_with(TL_HOOK_ID_TESTCASE) {
                    tl_hook_type = ThinlineHookType::TestCase;
                    tl_hook_idx = i;
                } else if tl_hook.starts_with(TL_HOOK_ID_TESTCLASS) {
                    tl_hook_type = ThinlineHookType::TestClass;
                    tl_hook_idx = i;
                } else if tl_hook.starts_with('!') {
                    if tl_hook_type != ThinlineHookType::NoType {
                        tl_hooked_context.push(HookedContext::new(
                            tl_hook_type,
                            doc_str.get(tl_hook_idx..i).ok_or_else(
                                || "Unable to export hooked test context",
                            )?,
                        ));
                    }
                    tl_hook_type = ThinlineHookType::NoType;
                }
            }
        }

        Ok(tl_hooked_context)
    }
}
