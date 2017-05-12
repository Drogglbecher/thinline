use error::*;
use function::FunctionCall;
use parameters::{EnvironmentParameters, TestFunctionSignature};
use regex::Regex;

static TL_TEST_CASE_FUNCTION_WRAP: &str = "TL_FCT";
static TL_STUB_ARG_SPECIFIER: &str = "//#ARG_";
static TL_STUB_ARG_SUFFIX: &str = "#/";

#[derive(Default)]
/// Represents one test case of the given test class.
pub struct TestCase {
    pub name: String,
    pub class_str: String,
    pub content: Vec<String>,
}

impl TestCase {
    /// Creates a new instance of `TestCase`.
    pub fn new(id_line: &str) -> Result<TestCase> {
        let mut testcase = TestCase::default();

        // Parsed testclass and testname out of the signature
        if let Ok(re) = Regex::new(r".*\((.*?)::(.*?)\)") {
            if let Some(caps) = re.captures(id_line) {
                testcase.class_str = caps.get(1).map_or(
                    String::new(),
                    |m| String::from(m.as_str()),
                );
                testcase.name = caps.get(2).map_or(
                    String::new(),
                    |m| String::from(m.as_str()),
                );
                if !testcase.class_str.is_empty() && !testcase.name.is_empty() {
                    return Ok(testcase);
                }
                return Err(Error::from("Parsing name and class parameters failed."));
            }
            return Err(Error::from("No regex match for test case parameters."));
        }
        Err(Error::from("Creating regex failed."))
    }

    fn replace_tl_fct<S: Into<String>>(
        value_str: S,
        fct_call_raw: &mut FunctionCall,
    ) -> Result<FunctionCall> {

        // Count number of given arguments separated by ','
        let parsed_arg_str = value_str.into();
        let mut fct_call = fct_call_raw.clone();
        let no_of_commas = fct_call.arg_str.matches(',').count();
        let mut parsed_args: Vec<&str> = parsed_arg_str.split(',').collect();

        // Check if an empty function gets tested
        if parsed_args.len() == 1 &&
            parsed_args
                .get(0)
                .ok_or_else(|| "Unable to acccess first parsed arg.")?
                .trim()
                .is_empty()
        {

            parsed_args.clear();
        }

        // Check for current parsed value and estimation strings
        if !parsed_arg_str.is_empty() {

            // Check if number of commaseparated arguments is equal to functions parameter amount
            if no_of_commas != parsed_args.len() - 1 {
                return Err(Error::from(
                    format!(
                        "{} parameter given but function '{}' has {}.",
                        parsed_args.len(),
                        fct_call_raw.name,
                        no_of_commas + 1
                    ).as_str(),
                ));
            }

            // Resolve the parsed 'parametername:value'-pairs
            for arg in parsed_args {
                let arg_sig: Vec<&str> = arg.split(':').collect();

                // Get the parameter name
                let arg_name = arg_sig
                    .get(0)
                    .ok_or_else(|| {
                        format!("Unable to access argument name for arg '{}'.", arg)
                    })?
                    .trim();
                // A value for an parameter was given which actually isn't one of this function
                if !fct_call.arg_str.contains(arg_name) {
                    return Err(Error::from(format!(
                        "Function does not contain argument named '{}'.",
                        arg_name
                    )));
                }

                // Get the parameter value
                let arg_val = arg_sig
                    .get(1)
                    .ok_or_else(|| {
                        format!("Unable to access argument '{}' value.", arg_name)
                    })?
                    .trim();
                // The given value for the paramter is empty
                if arg_val.is_empty() {
                    return Err(Error::from(
                        format!("Value for argument '{}' is empty.", arg_name),
                    ));
                }

                // Replace the function signature with the parsed values
                fct_call.arg_str = fct_call.arg_str.replace(arg_name, arg_val);
            }
        }

        Ok(fct_call)
    }

    /// Creates the final formatted test function call out of the thinline test command.
    fn process_test_fct(
        test_cmd: &str,
        fct_call: &mut FunctionCall,
        fct_sig: &TestFunctionSignature,
    ) -> Result<String> {

        let mut re_inline_sig_raw = fct_sig.inline_sig.clone();

        // Iterates through the number of arguments, e.g. /#ARG_1#/
        for i in 0..fct_sig.arg_no {
            // Replaces the /#ARG_i#/ with a regex capture pattern '(.*?)'
            re_inline_sig_raw =
                re_inline_sig_raw.replace(
                    format!("{}{}{}", TL_STUB_ARG_SPECIFIER, i, TL_STUB_ARG_SUFFIX)
                        .as_str(),
                    "(.*?)",
                );
        }

        // Replaces spaces with the regex representation '\s*'
        re_inline_sig_raw = re_inline_sig_raw.replace(" ", "\\s*");
        if let Ok(re_inline_sig) = Regex::new(format!(r"\s*{}\s*$", re_inline_sig_raw).as_str()) {
            if let Some(caps) = re_inline_sig.captures(test_cmd) {
                let mut test_fct_call = fct_sig.stub.clone();

                // Parses the values for the specific parameter number
                for i in 0..fct_sig.arg_no {
                    let value = caps.get(i as usize + 1).map_or("", |m| m.as_str().trim());
                    if value.is_empty() {
                        return Err(Error::from(format!(
                            "{}{}{} is empty.",
                            TL_STUB_ARG_SPECIFIER,
                            i,
                            TL_STUB_ARG_SUFFIX
                        )));
                    }

                    // Does the value contain a "TL_FCT" identifier?
                    if value.contains("TL_FCT") {
                        if let Ok(re_tl_fct) = Regex::new(
                            format!(
                                r"(.*?){}\s*\((.*?)\).*$",
                                TL_TEST_CASE_FUNCTION_WRAP
                            ).as_str(),
                        )
                        {
                            if let Some(caps_tl_fct) = re_tl_fct.captures(value) {

                                // Get the prefix and the content of "TL_FCT"
                                let pre_value_str = String::from(caps_tl_fct.get(1).map_or(
                                    "",
                                    |m| m.as_str().trim(),
                                ));
                                let value_str =
                                    caps_tl_fct.get(2).map_or("", |m| m.as_str().trim());

                                // Assign the replaced function signature struct to `fct_call_cp`
                                let mut fct_call_cp =
                                    TestCase::replace_tl_fct(value_str, fct_call)?;
                                fct_call_cp.name.push_str(fct_call_cp.arg_str.as_str());

                                // Format the test function call by replacing "TL_FCT" with
                                // function call as string and its prefix `pre_value_str`
                                test_fct_call = test_fct_call.replace(
                                    format!(
                                        "{}{}{}",
                                        TL_STUB_ARG_SPECIFIER,
                                        i,
                                        TL_STUB_ARG_SUFFIX
                                    ).as_str(),
                                    (pre_value_str +
                                         fct_call_cp.name.as_str())
                                        .as_str(),
                                );
                                continue;
                            }
                            return Err(Error::from(format!(
                                "No regex match for test function sig '{}'.",
                                test_cmd
                            )));
                        }
                        return Err(Error::from(
                            "Creating regex for extracting TL_FCT arguments failed.",
                        ));

                    // We have a raw value here, so it can be directly replaced without
                    // transforming
                    } else {
                        test_fct_call = test_fct_call.replace(
                            format!(
                                "{}{}{}",
                                TL_STUB_ARG_SPECIFIER,
                                i,
                                TL_STUB_ARG_SUFFIX
                            ).as_str(),
                            value,
                        );
                    }
                }
                return Ok(test_fct_call);
            }
            return Err(Error::from(format!(
                "No regex match for test function sig '{}'.",
                test_cmd
            )));
        }
        Err(Error::from("Creating regex failed."))
    }

    /// Parsed the thinline function type and the inner description of a thinline
    /// test function.
    fn get_test_fct(test_cmd: &str) -> Result<(String, String)> {
        if let Ok(re_tl_fct) = Regex::new(r"#(.*?)\[(.*?)]\s*$") {
            if let Some(caps) = re_tl_fct.captures(test_cmd) {
                let test_cmd = caps.get(1).map_or("", |m| m.as_str().trim());
                let test_fct = caps.get(2).map_or("", |m| m.as_str().trim());
                return Ok((String::from(test_cmd), String::from(test_fct)));
            }
            return Err(Error::from(format!(
                "No regex match to extract function out of '{}'.",
                test_cmd
            )));
        }
        Err(Error::from(
            "Creating regex for extracting function name failed.",
        ))

    }

    /// Processes a test case to get the test content between thinline hooks.
    pub fn process(
        &mut self,
        test_cmds: &[String],
        fct_call: &mut FunctionCall,
        env_parameters: &mut EnvironmentParameters,
    ) -> Result<()> {

        for test_cmd in test_cmds {
            // Does the line start with an hook identifier?
            if test_cmd.starts_with('#') {
                let test_fct = TestCase::get_test_fct(test_cmd.as_str())?;
                if test_fct.0.is_empty() {
                    warn!("Could not extract function name out of '{}'.", test_cmd);
                }

                // Checks whether the test_cmd key is already known to test signature map
                // or adds this key when it could be parsed from the specified environment yaml
                if env_parameters.test_sig.contains_key(test_cmd) ||
                    env_parameters.add_test_fct_sig(test_fct.0.as_str()).is_ok()
                {
                    let test_fct_sig = env_parameters.get_test_fct_sig(test_fct.0.as_str())?;
                    self.content.push(TestCase::process_test_fct(
                        test_fct.1.as_str(),
                        fct_call,
                        &test_fct_sig,
                    )?);
                } else {
                    warn!(
                        "Test command '{}' not found in '{}' environments function description ==> Skipping.",
                        test_cmd,
                        env_parameters.env
                    );
                    continue;
                }

            // This command is not a special thinline one but a raw command and can get pushed
            // to the test content directly
            } else {
                self.content.push(test_cmd.to_owned());
            }
        }

        Ok(())
    }
}
