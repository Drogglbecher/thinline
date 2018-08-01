use error::*;

/// Reprensents a parsed function argument.
#[derive(Default, Debug, Clone)]
pub struct Argument {
    name: String,
    atype: Option<String>,
}

impl Argument {
    /// Creates a new Argument instance.
    ///
    /// # Example
    ///
    /// ```
    /// let argument = Argument::new("int1", Some("int")));
    /// assert_eq!(argument.name, "int1");
    /// assert!(argument.atype.is_some());
    /// assert_eq!(argument.atype.unwrap(), "int");
    /// ```
    pub fn new<S: Into<String>>(name: S, atype: Option<S>) -> Self {
        Argument {
            name: name.into(),
            atype: atype.map(S::into),
        }
    }
}

/// Reprensents a parsed function type.
#[derive(Default, Clone, Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Option<String>,
    pub arguments: Vec<Argument>,
    pub description: Vec<String>,
}

impl Function {
    /// Creates a new Function instance.
    ///
    /// # Example
    ///
    /// ```
    /// let function = Function::new("testFunction", Some("int")));
    /// assert_eq!(function.arguments.len(), 0);
    /// assert_eq!(function.description.len(), 0);
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Function {
            name: name.into(),
            return_type: None,
            arguments: Vec::new(),
            description: Vec::new(),
        }
    }

    /// Creates the format type for the Function.
    ///
    /// # Example
    ///
    /// ```
    /// let function = Function::new("testFunction"));
    /// function.set_return_type("int");
    /// assert_eq!(function.return_type, Some("int"));
    ///
    /// function.set_return_type("");
    /// assert_eq!(function.description, None);
    /// ```
    pub fn set_return_type(&mut self, ftype: &str) -> Result<()> {
        if ftype.is_empty() {
            self.return_type = None;
        } else {
            let ftype_vec: Vec<&str> = ftype.split('(').collect();
            self.return_type = Some(String::from(
                ftype_vec
                    .get(0)
                    .ok_or_else(|| "Function type can not be parsed from signature.")?
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
    /// let function = Function::new("testFunction", Some("int")));
    /// function.set_description("""
    /// #TL_TESTCASE(check_if_sum_works)
    ///    int test_no = 2;
    ///    #TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]
    ///    #TL_EQ[TL_FCT(no1: 5, no2: 2) => 7]
    ///    EXPECT_EQ(11, test_int_no1(9, 2));
    /// #!TL_TESTCASE
    /// """);
    /// assert_eq!(function.description.len(), 6);
    /// ```
    pub fn set_description(&mut self, description: &str) {
        self.description = description
            .split('\n')
            .map(|fd| {
                String::from(
                    fd.trim_left()
                        .trim_left_matches('*')
                        .trim_left_matches('/')
                        .trim_left(),
                )
            })
            .filter(|ref c| !c.is_empty() && c.as_str() != "**")
            .collect()
    }

    pub fn set_arguments(&mut self, arguments: &[Argument]) {
        self.arguments = arguments.into();
    }
}

trait Entity: Default {
    fn name(&self) -> Option<&str>;
}

/// Reprensents a parsed class type.
#[derive(Default, Clone, Debug)]
pub struct Class {
    pub name: Option<String>,
    functions: Vec<Function>,
}

impl Entity for Class {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }
}

impl Class {
    /// Creates a new Class instance.
    ///
    /// # Example
    ///
    /// ```
    /// let class = Class::new(Some("testClass"));
    /// assert_eq!(class.functions.len(), 0);
    /// ```
    pub fn new<S: Into<Option<String>>>(name: S) -> Self {
        Class {
            name: name.into(),
            functions: Vec::new(),
        }
    }

    /// Returns the function instance for the given function ID.
    fn function(&self, function_id: &str) -> Option<&mut Function> {
        let filtered_functions: Vec<Function> = self.functions
            .into_iter()
            .filter(|f| f.name.as_str() == function_id)
            .collect();

        if filtered_functions.get(0).is_some() {
            let function: Function = filtered_functions[0];
            return Some(&mut function);
        }

        None
    }
}

/// Reprensents a parsed namespace type.
#[derive(Default, Clone, Debug)]
pub struct Namespace {
    name: Option<String>,
    classes: Vec<Class>,
    namespaces: Option<Vec<Namespace>>,
}

impl Entity for Namespace {
    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }
}

impl Namespace {
    /// Creates a new Namespace instance.
    ///
    /// # Example
    ///
    /// ```
    /// let namespace = Namespace::new(Some("testNamespace"));
    /// assert_eq!(namespace.classes.len(), 0);
    /// ```
    pub fn new<S: Into<Option<String>>>(name: S) -> Self {
        Namespace {
            name: name.into(),
            classes: Vec::new(),
            namespaces: None,
        }
    }

    fn entity<T>(entity_vec: Vec<T>, entity: &str) -> Option<&mut T>
    where
        T: Entity,
    {
        let filtered_entities: Vec<T> = entity_vec
            .into_iter()
            .filter(|c| c.name() == Some(entity))
            .collect();

        if filtered_entities.get(0).is_some() {
            let entity: T = filtered_entities[0];
            return Some(&mut entity);
        }

        None
    }

    /// Returns the class instance for the given class ID.
    pub fn class<'a>(&self, class_id: &'a str) -> Option<&'a mut Class> {
        return Namespace::entity(self.classes, class_id)
    }

    /// Returns the namespace instance for the given namespace ID.
    pub fn namespace<'a>(&self, namespace_id: &'a str) -> Option<&'a mut Namespace> {
        match self.namespaces {
            Some(nss) => return Namespace::entity(nss, namespace_id),
            None => return None,
        }
    }
}
