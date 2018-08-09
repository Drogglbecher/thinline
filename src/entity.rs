use error::*;

/// Reprensents a parsed function argument.
#[derive(Default, Debug, Clone, PartialEq)]
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
    /// use thinlinelib::entity::Argument;
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
            value: None,
        }
    }

    /// Sets a value to the argument.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::Argument;
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

/// Reprensents a parsed function type.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub return_type: Option<String>,
    pub arguments: Option<Vec<Argument>>,
    pub description: Option<Vec<String>>,
}

impl Function {
    /// Creates a new Function instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::Function;
    ///
    /// let function = Function::new("testFunction");
    ///
    /// assert_eq!(function.name, String::from("testFunction"));
    /// assert!(function.return_type.is_none());
    /// assert!(function.arguments.is_none());
    /// assert!(function.description.is_none());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Function {
            name: name.into(),
            return_type: None,
            arguments: None,
            description: None,
        }
    }

    /// Creates the format type for the Function.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::Function;
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
    /// use thinlinelib::entity::Function;
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
    /// assert!(function.description.is_some());
    /// ```
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(
            description
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
                .collect(),
        )
    }

    /// Sets arguments for the Function.
    pub fn set_arguments(&mut self, arguments: &[Argument]) {
        if arguments.is_empty() {
            self.arguments = None;
        } else {
            self.arguments = Some(arguments.into());
        }
    }
}

/// Reprensents a parsed enum argument.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Enum {
    pub name: String,
    pub etype: Option<String>,
    pub arguments: Option<Vec<Argument>>,
}

impl Enum {
    /// Creates a new Enum instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::Enum;
    ///
    /// let enumeration = Enum::new("testEnum");
    ///
    /// assert_eq!(enumeration.name, String::from("testEnum"));
    /// assert!(enumeration.etype.is_none());
    /// assert!(enumeration.arguments.is_none());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Enum {
            name: name.into(),
            etype: None,
            arguments: None,
        }
    }

    /// Sets arguments for the Enum.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::{Argument, Enum};
    ///
    /// let mut enumeration = Enum::new("testEnum");
    /// let args = vec![Argument::new("Zero", Some("0")), Argument::new("Two", Some("2"))];
    /// enumeration.set_arguments(&args);
    ///
    /// assert!(enumeration.arguments.is_some());
    /// assert_eq!(enumeration.arguments.unwrap().len(), 2);
    /// ```
    pub fn set_arguments(&mut self, arguments: &[Argument]) {
        if arguments.is_empty() {
            self.arguments = None;
        } else {
            self.arguments = Some(arguments.into());
        }
    }

    /// Adds an Argument to the Enum argument list.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::{Argument, Enum};
    ///
    /// let mut argument = Argument::new("arg", Some("std::string"));
    /// argument.set_value("FirstArg");
    ///
    /// let mut enumeration = Enum::new("enum");
    /// enumeration.push_argument(argument);
    ///
    /// assert!(enumeration.arguments.is_some());
    /// assert_eq!(enumeration.arguments.unwrap().len(), 1);
    ///
    /// ```
    pub fn push_argument(&mut self, argument: Argument) {
        if self.arguments.is_none() {
            self.arguments = Some(Vec::new());
        }

        if let Some(arguments) = &mut self.arguments {
            arguments.push(argument);
        }
    }
}

/// The different types an Entitiy can have.
#[derive(Clone, Debug, PartialEq)]
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
    /// use thinlinelib::entity::Entity;
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
    /// use thinlinelib::entity::{Entity, EntityType};
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
    /// use thinlinelib::entity::{Entity, Function};
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
