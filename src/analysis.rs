use entity::Entity;
use failure::{err_msg, Fallible};
use language_type::LanguageType;
use std::{
    cell::{Ref, RefCell, RefMut}, ffi::OsStr, fmt::{Display, Formatter, Result}, fs::read_link,
    marker::PhantomData, path::PathBuf,
};
use walkdir::WalkDir;

////////////////////////////////////////////////////////////////////////////////

/// Represents a parsed entity description.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub lines: Vec<String>,
}

impl Description {
    /// Creates a new Description instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Description;
    ///
    /// let description = Description::new();
    ///
    /// assert!(description.lines.is_empty());
    /// ```
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    /// Sets and formats the description.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Description;
    ///
    /// let mut description = Description::new();
    /// description.set("EQ[this->class_inst->TL_FCT(no1: no1, no2: 10) => 15]");
    ///
    /// assert_eq!(description.lines.len(), 1);
    /// ```
    pub fn set(&mut self, description: &str) {
        self.lines = description
            .split('\n')
            .map(|desc| {
                String::from(
                    desc.trim_left()
                        .trim_left_matches('*')
                        .trim_left_matches('/')
                        .trim_left(),
                )
            })
            .filter(|ref desc| !desc.is_empty() && desc.as_str() != "**")
            .map(|desc| {
                if desc.chars().next() == Some('#') {
                    desc.replace(" ", "")
                } else {
                    desc
                }
            })
            .collect();
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a parsed function argument.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub atype: Option<String>,
    pub value: Option<String>,
}

impl Argument {
    /// Creates a new Argument instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Argument;
    ///
    /// let argument = Argument::new("int1", Some("int"));
    ///
    /// assert_eq!(argument.name, "int1");
    /// assert!(argument.atype.is_some());
    /// assert_eq!(argument.atype.unwrap(), "int");
    /// ```
    pub fn new<S: Into<String>>(name: S, atype: Option<S>) -> Self {
        Self {
            name: name.into(),
            atype: atype.map(S::into),
            value: None,
        }
    }

    /// Sets a value to the argument.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Argument;
    ///
    /// let mut argument = Argument::new("arg", Some("std::string"));
    /// argument.set_value("FirstArg");
    ///
    /// assert!(argument.value.is_some());
    ///
    /// ```
    pub fn set_value(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a parsed function type.
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub return_type: Option<String>,
    pub arguments: Vec<Argument>,
    pub description: Option<Description>,
}

impl Function {
    /// Creates a new Function instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Function;
    ///
    /// let function = Function::new("testFunction");
    ///
    /// assert_eq!(function.name, String::from("testFunction"));
    /// assert!(function.return_type.is_none());
    /// assert!(function.arguments.is_empty());
    /// assert!(function.description.is_none());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            return_type: None,
            arguments: Vec::new(),
            description: None,
        }
    }

    /// Creates the format type for the Function.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Function;
    ///
    /// let mut function = Function::new("testFunction");
    /// function.set_return_type("int");
    ///
    /// assert_eq!(function.return_type, Some(String::from("int")));
    ///
    /// function.set_return_type("");
    ///
    /// assert_eq!(function.return_type, None);
    /// ```
    pub fn set_return_type(&mut self, ftype: &str) -> Fallible<()> {
        if ftype.is_empty() {
            self.return_type = None;
        } else {
            let ftype_vec: Vec<&str> = ftype.split('(').collect();
            self.return_type = Some(String::from(
                ftype_vec
                    .get(0)
                    .ok_or_else(|| err_msg("Function type can not be parsed from signature."))?
                    .trim_right(),
            ));
        }

        Ok(())
    }

    /// Sets the description for the Function.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Function;
    ///
    /// let mut function = Function::new("testFunction");
    /// function.set_description("
    /// # TESTCASE(check_if_sum_works)
    ///    int test_no = 2;
    ///    #EQ[TL_FCT(no1: test_no, no2: 5) => 7]
    ///    #EQ[TL_FCT(no1: 5, no2: 2) => 7]
    ///    EXPECT_EQ(11, test_int_no1(9, 2));
    /// ");
    ///
    /// assert!(function.description.is_some());
    /// ```
    pub fn set_description(&mut self, description: &str) {
        if self.description.is_none() {
            self.description = Some(Description::new());
        }

        if let Some(desc) = &mut self.description {
            desc.set(description);
        }
    }

    /// Sets arguments for the Function.
    pub fn set_arguments(&mut self, arguments: &[Argument]) {
        self.arguments = arguments.into();
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a parsed enum argument.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub etype: Option<String>,
    pub arguments: Vec<Argument>,
}

impl Enum {
    /// Creates a new Enum instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Enum;
    ///
    /// let enumeration = Enum::new("testEnum");
    ///
    /// assert_eq!(enumeration.name, String::from("testEnum"));
    /// assert!(enumeration.etype.is_none());
    /// assert!(enumeration.arguments.is_empty());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            etype: None,
            arguments: Vec::new(),
        }
    }

    /// Sets arguments for the Enum.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::{Argument, Enum};
    ///
    /// let mut enumeration = Enum::new("testEnum");
    /// let args = vec![Argument::new("Zero", Some("0")), Argument::new("Two", Some("2"))];
    /// enumeration.set_arguments(&args);
    ///
    /// assert_eq!(enumeration.arguments.len(), 2);
    /// ```
    pub fn set_arguments(&mut self, arguments: &[Argument]) {
        self.arguments = arguments.into();
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Represents a parsed project file.
#[derive(Default, Clone, Debug)]
pub struct ProjectFile<T> {
    pub path: PathBuf,
    pub entities: RefCell<Vec<Entity>>,
    pub pf_type: PhantomData<T>,
}

impl<T> ProjectFile<T>
where
    T: LanguageType,
{
    /// Creates a new ProjectFile instance.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use thinlinelib::analysis::ProjectFile;
    /// use thinlinelib::language_type::C;
    ///
    /// let project_file: ProjectFile<C> = ProjectFile::new("test/project_file");
    ///
    /// assert_eq!(project_file.path, PathBuf::from("test/project_file"));
    /// assert_eq!(project_file.entities().len(), 0);
    /// ```
    pub fn new<S: Into<PathBuf>>(path: S) -> Self {
        Self {
            path: path.into(),
            entities: RefCell::new(Vec::new()),
            pf_type: PhantomData,
        }
    }

    /// Returns a reference to the entities list.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::ProjectFile;
    /// use thinlinelib::entity::Entity;
    /// use thinlinelib::language_type::C;
    ///
    /// let project_file: ProjectFile<C> = ProjectFile::new("test/project_file");
    /// project_file.entities_mut().push(Entity::new("testEntity"));
    ///
    /// assert_eq!(project_file.entities().len(), 1);
    /// ```
    pub fn entities(&self) -> Ref<Vec<Entity>> {
        self.entities.borrow()
    }

    /// Returns a mutable reference to the entities list.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::ProjectFile;
    /// use thinlinelib::entity::Entity;
    /// use thinlinelib::language_type::C;
    ///
    /// let project_file: ProjectFile<C> = ProjectFile::new("test/project_file");
    /// project_file.entities_mut().push(Entity::new("testEntity"));
    ///
    /// let mut entities = project_file.entities_mut();
    /// assert_eq!(entities.len(), 1);
    ///
    /// entities.clear();
    /// assert_eq!(entities.len(), 0);
    /// ```
    pub fn entities_mut(&self) -> RefMut<Vec<Entity>> {
        self.entities.borrow_mut()
    }
}

impl<T> Display for ProjectFile<T>
where
    T: LanguageType,
{
    /// Formats a ProjectFile to be displayed by std output.
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(path) = self.path.to_str() {
            return write!(f, "{}", path);
        }
        write!(f, "Unable to stringify filename")
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The analyzer which fulfills parsing and entity extraction tasks.
#[derive(Default, Debug)]
pub struct Analysis<T>
where
    T: LanguageType,
{
    pub file_types: &'static [&'static str],
    project_files: RefCell<Vec<ProjectFile<T>>>,
}

impl<T> Analysis<T>
where
    T: LanguageType,
{
    /// Creates a new Analysis instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::Analysis;
    /// use thinlinelib::language_type::{C, LanguageType};
    ///
    /// let analysis: Analysis<C> = Analysis::new();
    ///
    /// assert_eq!(analysis.file_types, C::file_types());
    /// assert_eq!(analysis.project_files().len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            file_types: T::file_types(),
            project_files: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the collected project files for analysis.
    pub fn project_files(&self) -> Ref<Vec<ProjectFile<T>>> {
        self.project_files.borrow()
    }

    /// Returns a mutable reference to the collected project files for analysis.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::analysis::{Analysis, ProjectFile};
    /// use thinlinelib::language_type::C;
    ///
    /// let analysis: Analysis<C> = Analysis::new();
    /// let mut project_files = analysis.project_files_mut();
    /// assert_eq!(project_files.len(), 0);
    ///
    /// project_files.push(ProjectFile::new("test/anotherFile"));
    /// assert_eq!(project_files.len(), 1);
    /// ```
    pub fn project_files_mut(&self) -> RefMut<Vec<ProjectFile<T>>> {
        self.project_files.borrow_mut()
    }

    /// Collects all the sources within the given project dir.
    /// Does also work for symlinked files.
    pub fn collect_sources(&self, project_dir: &PathBuf, search_dirs: &[String]) -> Fallible<()> {
        debug!("Collecting analysis sources.");

        // Check the given project directory
        if !project_dir.exists() || !project_dir.is_dir() {
            return Err(format_err!(
                "The given project dir '{}' does not exist.",
                project_dir
                    .to_str()
                    .ok_or_else(|| err_msg("Unable to stringify project dir path."))?
            ));
        }

        // Traverse through the files within the specified source directories
        // and store them for analyzing purposes
        for src_dir in search_dirs {
            for ext in self.file_types {
                for dir in WalkDir::new(project_dir.join(src_dir).to_str().unwrap_or(".")) {
                    let entry = dir?;
                    if !entry.path().is_dir() && entry.path().extension() == Some(OsStr::new(ext)) {
                        let file = if entry.path_is_symlink() {
                            ProjectFile::new(read_link(entry.path())?)
                        } else {
                            ProjectFile::new(entry.path())
                        };
                        debug!("Add target project file '{}' to analysis.", file);
                        self.project_files_mut().push(file);
                    }
                }
            }
        }

        Ok(())
    }

    /// Extracts function signatures and comments of thinlines parsed files.
    pub fn extract_entities(&self) -> Fallible<()> {
        T::extract_entities(&self)
    }
}
