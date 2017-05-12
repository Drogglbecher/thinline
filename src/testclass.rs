use error::*;
use regex::Regex;
use std::collections::HashMap;
use testcase::TestCase;

static TL_SET_UP_CONTEXT_ID: &str = "#TL_SET_UP_CONTEXT";
static TL_TEAR_DOWN_CONTEXT_ID: &str = "#TL_TEAR_DOWN_CONTEXT";
static TL_CONSTRUCTOR_CONTEXT_ID: &str = "#TL_CONSTRUCTOR_CONTEXT";
static TL_DESTRUCTOR_CONTEXT_ID: &str = "#TL_DESTRUCTOR_CONTEXT";
static TL_CLASS_CONTEXT: &str = "#TL_CLASS_CONTEXT";

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum ContextType {
    SetUp,
    TearDown,
    Constructor,
    Destructor,
    Class,
    NoType,
}

lazy_static! {
    static ref CONTEXT_TYPE_MAP: HashMap<&'static str, ContextType> = {
        let mut hm = HashMap::new();
        hm.insert(TL_SET_UP_CONTEXT_ID, ContextType::SetUp);
        hm.insert(TL_TEAR_DOWN_CONTEXT_ID, ContextType::TearDown);
        hm.insert(TL_CONSTRUCTOR_CONTEXT_ID, ContextType::Constructor);
        hm.insert(TL_DESTRUCTOR_CONTEXT_ID, ContextType::Destructor);
        hm.insert(TL_CLASS_CONTEXT, ContextType::Class);
        hm
    };
}

pub struct TestClassElement {
    pub context_type: ContextType,
    pub context_str: Option<String>,
}

impl TestClassElement {
    pub fn new(context_type: ContextType) -> Self {
        TestClassElement {
            context_type: context_type,
            context_str: None,
        }
    }

    pub fn set_context(&mut self, tc_idx: usize, i: usize, tabs: usize, tc_cmd: &str) {
        if self.context_str.is_none() {
            self.context_str = Some(String::new());
        }
        if let Some(ref mut sc) = self.context_str {
            if i > tc_idx {
                sc.push('\n');
                sc.push_str("    ".repeat(tabs).as_str());
            }
            sc.push_str(tc_cmd);
        }
    }
}

#[derive(Default)]
pub struct TestClass {
    pub context_map: HashMap<ContextType, TestClassElement>,
    pub testcases: Vec<TestCase>,
}

impl TestClass {
    pub fn new() -> Self {
        let mut tc = TestClass {
            context_map: HashMap::new(),
            testcases: Vec::new(),
        };

        tc.context_map.insert(
            ContextType::Constructor,
            TestClassElement::new(ContextType::Constructor),
        );
        tc.context_map.insert(
            ContextType::Destructor,
            TestClassElement::new(ContextType::Destructor),
        );
        tc.context_map.insert(
            ContextType::SetUp,
            TestClassElement::new(ContextType::SetUp),
        );
        tc.context_map.insert(
            ContextType::TearDown,
            TestClassElement::new(ContextType::TearDown),
        );
        tc.context_map.insert(
            ContextType::Class,
            TestClassElement::new(ContextType::Class),
        );

        tc
    }

    pub fn extract_name(id_line: &str) -> Result<String> {
        if let Ok(re) = Regex::new(r".*\((.*?)\)") {
            if let Some(caps) = re.captures(id_line) {
                let class_str = caps.get(1).map_or(
                    String::new(),
                    |m| String::from(m.as_str()),
                );
                if !class_str.is_empty() {
                    return Ok(class_str);
                }
                return Err(Error::from("Parsing testclass name failed."));
            }
            return Err(Error::from("No regex match for testclass name."));
        }
        Err(Error::from("Creating regex failed."))
    }

    pub fn process(&mut self, tc_cmds: &[String]) -> Result<()> {
        let mut tc_idx = 0;
        let mut context_type = ContextType::NoType;
        for (i, tc_cmd) in tc_cmds.iter().enumerate() {
            let mut cmd = tc_cmd.clone();
            if cmd.ends_with(':') {
                cmd.pop().ok_or_else(|| {
                    format!("Unable to remove last char from testclass line {}.", tc_cmd)
                })?;
            }
            cmd = String::from(cmd.trim());
            if let Some(ct) = CONTEXT_TYPE_MAP.get(cmd.as_str()) {
                tc_idx = i;
                context_type = (*ct).clone();
                continue;
            } else if context_type != ContextType::NoType {
                if let Some(tc_element) = self.context_map.get_mut(&context_type) {
                    let tabs = if context_type == ContextType::Class {
                        1
                    } else {
                        3
                    };
                    tc_element.set_context(tc_idx, i, tabs, tc_cmd);
                }
            }
        }
        Ok(())
    }
}
