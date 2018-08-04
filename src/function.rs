use error::*;

/// Reprensents a parsed function argument.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub atype: Option<String>,
}

impl Argument {
    /// Creates a new Argument instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::function::Argument;
    ///
    /// let argument = Argument::new("int1", Some("int"));
    ///
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
#[derive(Default, Clone, Debug, PartialEq)]
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
    /// use thinlinelib::function::Function;
    ///
    /// let function = Function::new("testFunction");
    ///
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
    /// use thinlinelib::function::Function;
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
    /// use thinlinelib::function::Function;
    ///
    /// let mut function = Function::new("testFunction");
    /// function.set_description("
    /// #TL_TESTCASE(check_if_sum_works)
    ///    int test_no = 2;
    ///    #TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]
    ///    #TL_EQ[TL_FCT(no1: 5, no2: 2) => 7]
    ///    EXPECT_EQ(11, test_int_no1(9, 2));
    /// #!TL_TESTCASE
    /// ");
    ///
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

#[derive(Clone, Debug, PartialEq)]
/// The different types an Entitiy can have.
pub enum EntityType {
    /// A class definition.
    Class(Entity),

    /// A namespace.
    Namespace(Entity),

    /// The index of a new entity hierarchy.
    Index(Entity),
}

/// The representation of an Entity as a possbile generic node on the
/// abstract syntax tree. An Entity has to be kind of a EntityType.
#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub name: String,
    pub entities: Option<Vec<EntityType>>,
    pub functions: Option<Vec<Function>>,
}

impl Entity {
    /// Creates a new Entity instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::function::Entity;
    ///
    /// let class = Entity::new("testClass");
    ///
    /// assert!(class.functions.is_none());
    /// assert!(class.entities.is_none());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Entity {
            name: name.into(),
            entities: None,
            functions: None,
        }
    }

    /// Adds an Entity to the Entity instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::function::{Entity, EntityType};
    ///
    /// let mut entity = Entity::new("outer_entity");
    /// let entity_type = EntityType::Class(Entity::new("inner_entity"));
    /// entity.add_entity(entity_type);
    ///
    /// assert!(entity.entities.is_some());
    /// ```
    pub fn add_entity(&mut self, entity: EntityType) -> Option<&mut EntityType> {
        if self.entities.is_none() {
            self.entities = Some(Vec::new());
        }

        if let Some(entities) = &mut self.entities {
            entities.push(entity);
            return entities.last_mut();
        }

        None
    }

    /// Adds a Function to the Entity instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::function::{Entity, Function};
    ///
    /// let mut entity = Entity::new("entity");
    /// let function = Function::new("func");
    /// entity.add_function(function);
    ///
    /// assert!(entity.functions.is_some());
    /// ```
    pub fn add_function(&mut self, function: Function) -> Option<&mut Function> {
        if self.functions.is_none() {
            self.functions = Some(Vec::new());
        }

        if let Some(functions) = &mut self.functions {
            functions.push(function);
            return functions.last_mut();
        }

        None
    }
}
