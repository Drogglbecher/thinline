#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clang;
#[macro_use]
extern crate error_chain;
extern crate glob;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mowl;
extern crate regex;
extern crate walkdir;
extern crate yaml_rust;

pub mod environment;
use environment::Environment;

pub mod error;
use error::*;

pub mod filter;

pub mod function;
use function::Function;

pub mod parameters;
use parameters::{ProjectParameters, EnvironmentParameters};

pub mod testcase;
use testcase::TestCase;

pub mod testclass;
use testclass::{TestClass, ContextType};

pub mod testfile;
use testfile::TestFile;

pub mod testhandler;
use testhandler::{TestHandler, ThinlineHookType};

use clang::*;
use glob::glob;
use log::LogLevel;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File, canonicalize};
use std::io::Write;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use std::process::Command;
use walkdir::WalkDir;

static C_FILE_EXTENSIONS: &[&str] = &["*.c", "*.cc", "*.cpp", "*.h", "*.hpp"];
static C_HEADER_EXTENSIONS: &[&str] = &["h", "hpp"];

static TLG_HEADER_STUB_INCLUDE_PH: &str = "// INCLUDES //";
static TLG_HEADER_STUB_USPC_PH: &str = "// USER_SPECIFIC_PREFIX_CONTENT //";
static TLG_HEADER_STUB_USSC_PH: &str = "// USER_SPECIFIC_SUFFIX_CONTENT //";

static TLG_FILE_NAME: &str = "__tlg__.h";
static TLG_FILE_STR: &str = include_str!("../stubs/system/tlg.h");

macro_rules! unwrap_or_continue {
    ($e: expr) => (
        match $e {
            Some(e) => e,
            None => continue,
        }
    )
}

lazy_static! {
    static ref CONTEXT_ID_MAP: HashMap<ContextType, &'static str> = {
        let mut hm = HashMap::new();
        hm.insert(ContextType::SetUp, "//#CONSTRUCTOR_CONTEXT#/");
        hm.insert(ContextType::TearDown, "//#DESTRUCTOR_CONTEXT#/");
        hm.insert(ContextType::Constructor, "//#SET_UP_CONTEXT#/");
        hm.insert(ContextType::Destructor, "//#TEAR_DOWN_CONTEXT#/");
        hm.insert(ContextType::Class, "//#CLASS_CONTEXT#/");
        hm
    };
}

/// The data which holds parsed function signatures for a file.
struct ThinlineData {
    file: PathBuf,
    namespaces: Vec<String>,
    functions: Vec<Function>,
}

impl ThinlineData {
    /// Creates a new instace for `ThinlineData`.
    fn new<P: Into<PathBuf>>(file: P) -> Self {
        ThinlineData {
            file: file.into(),
            namespaces: Vec::new(),
            functions: Vec::new(),
        }
    }

    /// Adds a namespace to the `ThinlineData` instance.
    fn add_namespace(&mut self, namespace: &str) {
        self.namespaces.push(
            String::from("using namespace ") +
                namespace + ";",
        );
    }

    /// Adds a function to the `ThinlineData` instance.
    fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    /// Checks whether the `ThinlineData` instance already has this function.
    fn has_function(&self, fctr: &Function) -> bool {
        for fctl in &self.functions {
            if fctl.name == fctr.name && fctl.ctype == fctr.ctype &&
                fctl.parameter.len() == fctr.parameter.len()
            {

                return fctl.parameter.iter().zip(&fctr.parameter).all(|(l, r)| {
                    l.name == r.name && l.ctype == r.ctype
                });
            }
        }
        false
    }
}

#[derive(Default)]
/// Global structure representing the Thinline lib.
pub struct Thinline {
    /// The tree structure of the parsed functions.
    data: Vec<ThinlineData>,

    /// The project-specific parameters for the `Thinline` instance.
    project_parameters: ProjectParameters,

    /// The paths for stubs environment.
    environment: Environment,

    /// The environment-specific parameters as interface to the several
    /// unit test environments
    env_parameters: EnvironmentParameters,

    /// Holds all test files
    testfile_map: HashMap<PathBuf, TestFile>,
}

impl Thinline {
    /// Creates an instance of the lib contatining thinlines functionality.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the logging behaviour of the thinline lib depending on
    /// the log level.
    pub fn init_logging(&mut self, level: LogLevel) -> Result<()> {
        match mowl::init_with_level(level) {
            Ok(_) => info!("Log level set to: {}", level),
            Err(_) => bail!("Initialization of mowl logger failed."),
        }

        Ok(())
    }

    pub fn init_environment(&mut self) -> Result<()> {
        self.environment.create_config_directory()?;
        self.environment.create_environment_config()?;
        self.environment.create_google_test_stubs()?;

        Ok(())
    }

    /// Collects all source and header files of the given directory recursively.
    pub fn collect_sources<P: Into<PathBuf>>(
        &mut self,
        project_dir: P,
        config_name: &str,
    ) -> Result<()> {
        self.data.clear();

        let project_dir_path = project_dir.into();

        if !project_dir_path.exists() || !project_dir_path.is_dir() {
            return Err(Error::from(format!(
                "The given project dir '{}' does not exist.",
                project_dir_path.to_str().ok_or_else(
                    || "Unable to stringify project dir path.",
                )?
            )));
        }
        let project_config = project_dir_path.join(config_name);
        if !project_config.exists() || !project_config.is_file() {
            return Err(Error::from(format!(
                "The given project config file '{}' does not exist or is a directory.",
                project_config.to_str().ok_or_else(
                    || "Unable to stringify project config file.",
                )?
            )));
        }

        self.project_parameters = ProjectParameters::parse(project_config.to_str().ok_or_else(
            || "Unable to stringify project config file.",
        )?)?;

        self.env_parameters = EnvironmentParameters::parse(
            self.environment.env_config_path.as_str(),
            self.environment.env_config_file.as_str(),
            self.project_parameters.test_env.as_str(),
        )?;

        for ext in C_FILE_EXTENSIONS {
            if let Some(ref src_dirs) = self.project_parameters.src_dirs {
                for src_dir in src_dirs {
                    let c_src_paths = project_dir_path.join(src_dir).join("**").join(ext);
                    for entry in glob(c_src_paths.to_str().unwrap_or("."))? {
                        self.data.push(ThinlineData::new(entry?));
                    }
                }
            }
        }

        Ok(())
    }

    /// Checks whether the function is already covered.
    fn has_function(&self, fct: &Function) -> bool {
        for data in &self.data {
            if data.has_function(fct) {
                return true;
            }
        }
        false
    }

    fn filter_for_functions(&mut self, idx: usize, entity: &Entity) -> Result<()> {

        // Iterate through the child entities of the current entity
        for child in entity.get_children() {
            let child_kind = child.get_kind();
            if !child.is_in_system_header() &&
                (child_kind == EntityKind::FunctionDecl || child_kind == EntityKind::Method ||
                     child_kind == EntityKind::Constructor ||
                     child_kind == EntityKind::Destructor)
            {

                let _fct_type = unwrap_or_continue!(child.get_type());
                let fct_type = _fct_type.get_display_name();
                let fct_name = unwrap_or_continue!(child.get_name());
                let fct_desc = unwrap_or_continue!(child.get_comment());

                // Everything seen to create a new child -> do it!
                debug!(
                    "Create child '{:?}' with type '{:?}'",
                    unwrap_or_continue!(child.get_name()),
                    child_kind
                );

                let mut fct_parsed: Function =
                    Function::new(
                        child.get_semantic_parent().and_then(|sp| sp.get_name()),
                        fct_name,
                    );
                fct_parsed.set_type(fct_type.as_str())?;
                fct_parsed.set_description(fct_desc.as_str());
                let fct_args = unwrap_or_continue!(child.get_arguments());
                fct_parsed.set_args(&fct_args)?;

                // Adds function to the thinline data when not already covered
                if !self.has_function(&fct_parsed) {
                    self.data[idx].add_function(fct_parsed);
                }

            // A namesapce or class is found -> Check their child entities by recursive call
            } else if (child_kind == EntityKind::Namespace &&
                           self.project_parameters
                               .language_features
                               .cpp_features
                               .namespace_filter
                               .filter(unwrap_or_continue!(child.get_name()).as_str())) ||
                       (child_kind == EntityKind::ClassDecl &&
                            self.project_parameters
                                .language_features
                                .cpp_features
                                .class_filter
                                .filter(unwrap_or_continue!(child.get_name()).as_str()))
            {

                if child_kind == EntityKind::Namespace {
                    let ns_name = child.get_name();
                    if let Some(test_ns) = ns_name {
                        self.data[idx].add_namespace(test_ns.as_str());
                    }
                }

                self.filter_for_functions(idx, &child)?;
            }
        }

        Ok(())
    }

    /// Extracts function signatures and comments of thinlines parsed files.
    pub fn extract_fct_symbols(&mut self) -> Result<()> {
        match Clang::new() {
            Ok(clang) => {
                let index = Index::new(&clang, false, false);
                for i in 0..self.data.len() {
                    // Makes slicing below pseudo-secure ^^
                    if self.data.get(i).is_none() {
                        continue;
                    }
                    let tu = index.parser(&self.data[i].file).parse()?;

                    // Extract all FunctionDecl Entities
                    self.filter_for_functions(i, &tu.get_entity())?;
                }
            }
            Err(e) => bail!(e),
        }

        Ok(())
    }

    /// Execute the given arguments os-dependent
    fn execute_args(args: &[String]) -> Result<()> {
        // Create OS dependent argument set
        let cmd = if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "sh"
        };

        let cmd_c = if cfg!(target_os = "windows") {
            "/C"
        } else {
            "-c"
        };

        // Execute the given build steps
        for build_step in args {
            info!("Execute build step '{}'", build_step);
            let output = Command::new(cmd)
                .args(&[cmd_c, build_step.as_str()])
                .output()?;
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    /// Executes the build steps given in .thinline.yml project file.
    pub fn execute_build_steps(&self, dir: &str) -> Result<()> {
        let current_working_dir = env::current_dir()?;
        env::set_current_dir(&dir)?;

        let build_script = if cfg!(target_os = "windows") {
            self.project_parameters.build_script.windows.as_ref()
        } else {
            self.project_parameters.build_script.linux.as_ref()
        };

        if let Some(args) = build_script {
            Thinline::execute_args(args)?;
        } else {
            warn!("Project build triggered but no build steps given");
            return Ok(());
        }

        env::set_current_dir(&current_working_dir)?;

        Ok(())
    }

    /// Synthesizes the general header file. Within this all files found in
    /// the .thinline.yml `include_dirs` paths are included. Afterwards
    /// this header is used to get the dependencies to the test functions.
    pub fn synthesize_general_header(&self, dir: &str) -> Result<()> {
        let output_folder = Path::new(dir).join(".thinline");
        if output_folder.exists() && output_folder.is_dir() {
            fs::remove_dir_all(&output_folder)?;
        }

        fs::create_dir_all(&output_folder)?;

        // Generate include string for tlg header stub
        let mut include_str = String::new();
        if let Some(ref include_dirs) = self.project_parameters.include_dirs {
            for include_dir in include_dirs {
                let include_dir_p = Path::new(dir).join(include_dir);
                let include_dir_s = include_dir_p.to_str().ok_or_else(
                    || "Unable to stringify include dir",
                )?;
                for include in WalkDir::new(include_dir_s).into_iter().filter_map(
                    |i_dir| i_dir.ok(),
                )
                {

                    let include_path_ext =
                        &unwrap_or_continue!(include.path().extension())
                            .to_str()
                            .ok_or_else(|| "Unable to stringify include path extension")?;

                    if include.path().is_file() && C_HEADER_EXTENSIONS.contains(include_path_ext) {

                        let include_n = canonicalize(&include.path())?;
                        let include_n_s = include_n.to_str().ok_or_else(
                            || "Unable to stringify canonical normal form of inclue path",
                        )?;
                        let include_dir_n = canonicalize(&Path::new(include_dir_s))?;
                        let mut include_dir_n_s = String::from(include_dir_n.to_str().ok_or_else(
                            || "Unable to stringify normal form of include dir",
                        )?);
                        include_dir_n_s.push(MAIN_SEPARATOR);

                        let include_h_fin =
                            format!(
                                "\n#include <{}>",
                                String::from(include_n_s).replace(include_dir_n_s.as_str(), "")
                            );
                        include_str.push_str(include_h_fin.as_str());
                    }
                }
            }
        }

        let mut tlg_header = TLG_FILE_STR.replace(TLG_HEADER_STUB_INCLUDE_PH, include_str.as_str());
        if let Some(ref user_prefix) = self.project_parameters.user_prefix {
            tlg_header = tlg_header.replace(TLG_HEADER_STUB_USPC_PH, user_prefix.as_str());
        }
        if let Some(ref user_suffix) = self.project_parameters.user_suffix {
            tlg_header = tlg_header.replace(TLG_HEADER_STUB_USSC_PH, user_suffix.as_str());
        }
        File::create(output_folder.join(TLG_FILE_NAME))?.write_all(
            tlg_header.as_bytes(),
        )?;

        Ok(())
    }

    /// Synthesizes the test cases based on the extracted unittests written in
    /// the several C-comment sections.
    pub fn synthesize_testcases(&mut self, dir: &str) -> Result<()> {
        let output_folder = Path::new(dir).join(".thinline");
        for mut data in &mut self.data {
            let output_test_file = String::from(
                data.file
                    .file_stem()
                    .ok_or_else(|| "Unable to get the test filestem.")?
                    .to_str()
                    .ok_or_else(|| "Unable to stringify the test filename.")?,
            ) + "." + self.env_parameters.output_format.as_str();
            let output_file = output_folder.join(output_test_file.as_str());

            self.testfile_map
                .entry(output_file.clone())
                .or_insert_with(|| TestFile::new(&output_file, &data.namespaces));
            let testfile = self.testfile_map.get_mut(&output_file).ok_or_else(|| {
                format!("Unable to get testfile entry for '{:?}'.", output_file)
            })?;

            for function in &mut data.functions {
                let test_contexts = TestHandler::check_for_tl_hooks(&function.doc_str, "#")?;
                for test_context in test_contexts {
                    match test_context.hook_type {
                        ThinlineHookType::TestCase => {
                            let mut testcase = TestCase::new(
                                test_context
                                    .context
                                    .get(0)
                                    .ok_or_else(|| "Unable to get test case ID line.")?
                                    .as_str(),
                            )?;
                            testcase.process(
                                &test_context.context.get(1..test_context.context.len())
                                                     .ok_or_else(|| "Unable to export hooked test instructions.")?
                                                     .to_vec(),
                                &mut function.format_as_call(),
                                &mut self.env_parameters)?;
                            let testclass = testfile
                                .testclass_map
                                .entry(testcase.class_str.clone())
                                .or_insert_with(TestClass::new);
                            testclass.testcases.push(testcase);
                        }
                        ThinlineHookType::TestClass => {
                            let tc_name = TestClass::extract_name(
                                test_context
                                    .context
                                    .get(0)
                                    .ok_or_else(|| "Unable to get test class ID line.")?
                                    .as_str(),
                            )?;
                            let testclass = testfile
                                .testclass_map
                                .entry(tc_name.clone())
                                .or_insert_with(TestClass::new);
                            testclass.process(&test_context
                                .context
                                .get(1..test_context.context.len())
                                .ok_or_else(|| "Unable to export hooked test class parameters.")?
                                .to_vec())?;
                        }
                        ThinlineHookType::NoType => {}
                    }
                }
            }
        }

        Ok(())
    }

    /// Creates the test files, which form the generated test suite at least.
    pub fn synthesize_testfiles(&self) -> Result<()> {
        for (testfile_path, testfile) in &self.testfile_map {

            let mut functions_in = false;
            let mut testcases_str = String::new();
            let mut testclasses_str = String::new();

            for (testclass_name, testclass) in &testfile.testclass_map {
                if !testclass.testcases.is_empty() {
                    functions_in = true;
                }

                // Generate filled testclass stub for testfile
                let mut testclass_stub = self.env_parameters.class_sig.clone();
                for (context_type, testclass_element) in &testclass.context_map {
                    if let Some(context_str) = CONTEXT_ID_MAP.get(context_type) {
                        match testclass_element.context_str {
                            Some(ref context) => {
                                testclass_stub =
                                    testclass_stub.replace(context_str, context.as_str())
                            }
                            None => testclass_stub = testclass_stub.replace(context_str, ""),
                        }
                    } else {
                        return Err(Error::from(format!(
                            "Context type '{:?}' not found in context ID map.",
                            context_type
                        )));
                    }
                }
                testclasses_str.push_str(
                    testclass_stub
                        .replace("//#TEST_CLASS#/", testclass_name.as_str())
                        .as_str(),
                );

                // Generate filled testcase stub for testfile
                for testcase in &testclass.testcases {
                    let mut testcontent = String::new();
                    for tc in &testcase.content {
                        testcontent.push_str("    ");
                        testcontent.push_str(tc.as_str());
                        testcontent.push('\n');
                    }
                    testcontent.pop();
                    let testcase_str = self.env_parameters
                        .fct_sig
                        .clone()
                        .replace("//#TEST_CLASS#/", testclass_name.as_str())
                        .replace("//#TEST_NAME#/", testcase.name.as_str())
                        .replace("//#TEST_CONTEXT#/", testcontent.as_str());
                    testcases_str.push_str(testcase_str.as_str());
                    testcases_str.push('\n');
                }
            }

            if functions_in {
                // Testcases found -> A testfile needs to be created
                let testfile_stub = self.env_parameters.file_stub.clone();
                let testfile_str = testfile_stub
                    .replace("//#TEST_NAMESPACES#/", testfile.test_namespaces.as_str())
                    .replace("//#TEST_CASES#/", testcases_str.as_str())
                    .replace("//#TEST_CLASSES#/", testclasses_str.as_str());
                File::create(testfile_path)?.write_all(
                    testfile_str.as_bytes(),
                )?;
            }
        }

        Ok(())
    }

    /// Creates the g++ build arguments.
    pub fn synthesize_gpp_args(&self, dir: &str) -> Result<()> {
        let output_folder = Path::new(dir).join(".thinline");
        let current_working_dir = env::current_dir()?;
        env::set_current_dir(&output_folder)?;
        let mut gpp_arg = String::from("g++");
        for (src, val) in &self.testfile_map {
            if !val.testclass_map.is_empty() {
                gpp_arg.push(' ');
                gpp_arg.push_str(src.file_name()
                                    .ok_or_else(|| format!("Unable to get file name of src path {:?}.", src))?
                                    .to_str()
                                    .ok_or_else(|| format!("Unable to stringify test file path {:?}.", src))?);
            }
        }
        if let Some(ref include_dirs) = self.project_parameters.include_dirs {
            for include_dir in include_dirs {
                gpp_arg.push_str(" -I");
                gpp_arg.push_str(Path::new("..")
                    .join(include_dir.as_str())
                    .to_str()
                    .ok_or_else(|| "Unable to stringify include path buf")?);
            }
        }
        for llib in &self.project_parameters.lib_paths {
            let filename = Path::new(llib)
                .file_name()
                .ok_or_else(|| format!("Unable to get file name of lib path {}.", llib))?
                .to_str()
                .ok_or_else(|| format!("Unable to stringify lib path {}.", llib))?;
            gpp_arg.push_str(
                format!(
                    " -Wl,-rpath,{p} -L{p} -l{s}",
                    p = Path::new("..")
                        .join(llib.replace(filename, ""))
                        .to_str()
                        .ok_or_else(|| "Unable to stringify lib path buf.")?,
                    s = Path::new(llib)
                        .file_stem()
                        .ok_or_else(|| format!("Unable to get file stem of lib path {}.", llib))?
                        .to_str()
                        .ok_or_else(|| {
                            format!("Unable to stringify file stem of path {}.", llib)
                        })?
                        .to_owned()
                        .split_off(3)
                ).as_str(),
            );
        }
        gpp_arg.push_str(" -lgtest -lgtest_main -o thinline_test");

        Thinline::execute_args(&[gpp_arg])?;
        Thinline::execute_args(&[String::from("./thinline_test")])?;

        env::set_current_dir(current_working_dir)?;

        Ok(())
    }

    /// Reconstructs the parsed function signatures. This function is more
    /// for debug purposes.
    pub fn reconstruct_fn(&self) -> Result<()> {
        for data in &self.data {
            debug!("In file {}", data.file.to_str().unwrap());
            for function in &data.functions {
                for doc in &function.doc_str {
                    debug!("/// {}", doc);
                }
                let class = match function.class_name {
                    Some(ref c) => c.to_owned() + "::",
                    None => String::new(),
                };
                debug!(
                    "- {} {}{}{}",
                    function.ctype,
                    class,
                    function.name,
                    function.get_args_as_str()
                )
            }
        }

        Ok(())
    }
}
