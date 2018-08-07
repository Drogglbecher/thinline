use error::*;
use function::Entity;
use glob::glob;
use language_type::LanguageType;
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Default, Clone, Debug)]
pub struct ProjectFile<T> {
    pub pf_type: PhantomData<T>,
    pub path: PathBuf,
    pub entities: RefCell<Vec<Entity>>,
}

/// Reprensents a parsed project file.
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
        ProjectFile {
            pf_type: PhantomData,
            path: path.into(),
            entities: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the entities list.
    ///
    /// # Example
    ///
    /// ```
    /// use std::cell::Ref;
    /// use thinlinelib::analysis::ProjectFile;
    /// use thinlinelib::language_type::C;
    /// use thinlinelib::function::Entity;
    ///
    /// let project_file: ProjectFile<C> = ProjectFile::new("test/project_file");
    /// project_file.add_entity(Entity::new("testEntity"));
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
    /// use std::cell::Ref;
    /// use thinlinelib::analysis::ProjectFile;
    /// use thinlinelib::language_type::C;
    /// use thinlinelib::function::Entity;
    ///
    /// let project_file: ProjectFile<C> = ProjectFile::new("test/project_file");
    /// project_file.add_entity(Entity::new("testEntity"));
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

    /// Adds an Entity to the entities list.
    ///
    /// # Example
    ///
    /// ```
    /// use std::cell::Ref;
    /// use thinlinelib::analysis::ProjectFile;
    /// use thinlinelib::language_type::C;
    /// use thinlinelib::function::Entity;
    ///
    /// let project_file: ProjectFile<C> = ProjectFile::new("test/project_file");
    /// assert_eq!(project_file.entities().len(), 0);
    ///
    /// project_file.add_entity(Entity::new("testEntity"));
    /// assert_eq!(project_file.entities().len(), 1);
    /// ```
    pub fn add_entity(&self, entity: Entity) {
        self.entities_mut().push(entity);
    }
}

#[derive(Default, Debug)]
pub struct Analysis<T>
where
    T: LanguageType,
{
    pub file_types: &'static [&'static str],
    pub project_files: RefCell<Vec<ProjectFile<T>>>,
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
        Analysis {
            file_types: T::file_types(),
            project_files: RefCell::new(Vec::new()),
        }
    }

    /// Returns a reference to the collected project files for anaylsis.
    pub fn project_files(&self) -> Ref<Vec<ProjectFile<T>>> {
        self.project_files.borrow()
    }

    /// Returns a mutable reference to the collected project files for anaylsis.
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

    /// Collect all the sources within the given project dir.
    pub fn collect_sources(&self, project_dir: &PathBuf, search_dirs: &[&str]) -> Result<()> {
        // Check the given project directory
        if !project_dir.exists() || !project_dir.is_dir() {
            return Err(Error::from(format!(
                "The given project dir '{}' does not exist.",
                project_dir
                    .to_str()
                    .ok_or_else(|| "Unable to stringify project dir path.")?
            )));
        }

        // Traverse through the files within the specified source directories
        // and store them for analyzing purposes
        for src_dir in search_dirs {
            for ext in self.file_types {
                for entry in glob(
                    project_dir
                        .join(src_dir)
                        .join("**")
                        .join(String::from("*.") + ext)
                        .to_str()
                        .unwrap_or("."),
                )? {
                    self.project_files_mut().push(ProjectFile::new(entry?));
                }
            }
        }

        Ok(())
    }

    /// Extracts function signatures and comments of thinlines parsed files.
    pub fn extract_entities(&self) -> Result<()> {
        T::extract_functions(&self)
    }
}
