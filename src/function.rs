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

#[derive(Clone, Debug, PartialEq)]
/// The different types an Entitiy can have.
pub enum EntityType {
    /// A class definition.
    Class,

    /// A namespace.
    Namespace,

    /// The index of a new entity hierarchy.
    Index,
}

#[derive(Clone, Debug)]
/// The representation of an Entity as a possbile generic node on the
/// abstract syntax tree. An Entity has to be kind of a EntityType.
pub struct Entity {
    name: String,
    entities: Option<Vec<Entity>>,
    functions: Option<Vec<Function>>,
    etype: EntityType,
}

impl Entity {
    /// Creates a new Entity instance.
    ///
    /// # Example
    ///
    /// ```
    /// let class = Class::new(Some("testClass"));
    /// assert_eq!(class.functions.len(), 0);
    /// ```
    pub fn new<S: Into<String>>(etype: EntityType, name: S) -> Self {
        Entity {
            name: name.into(),
            entities: None,
            functions: None,
            etype: etype,
        }
    }

    /// Adds an Entity to the Entity instance.
    ///
    /// # Example
    ///
    /// ```
    /// let outer_entity = Entity::new("outer_entity", EntityType::Namespace);
    /// let inner_entity = Entity::new("inner_entity", EntityType::Class);
    /// outer_entity.add_entity(inner_entity);
    ///
    /// assert_eq!(outer_entity.entities.is_some());
    /// ```
    pub fn add_entity(&mut self, entity: Entity) -> Option<&mut Entity> {
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
    /// let entity = Entity::new("entity", EntityType::Namespace);
    /// outer_entity.add_entity(inner_entity);
    ///
    /// assert_eq!(outer_entity.entities.is_some());
    /// ```
    pub fn add_function(&mut self, function: Function) {
        if self.functions.is_none() {
            self.functions = Some(Vec::new());
        }

        if let Some(functions) = &mut self.functions {
            functions.push(function);
        }
    }

    /// Returns an Entity with the given name or None when nothing is found.
    ///
    /// # Example
    ///
    /// ```
    /// let outer_entity = Entity::new("outer_entity", EntityType::Namespace);
    /// let inner_entity = Entity::new("inner_entity", EntityType::Class);
    /// outer_entity.add_entity(inner_entity);
    ///
    /// assert!(outer_entity.entity(EntityType::Class, "inner_entity").is_some());
    /// ```
    pub fn entity(&self, etype: EntityType, name: &str) -> Option<&Entity> {
        if let Some(entities) = &self.entities {
            let filtered_entities: Vec<&Entity> = entities
                .into_iter()
                .filter(|c| c.name == name && c.etype == etype)
                .collect();

            if let Some(entity) = filtered_entities.get(0) {
                return Some(&entity);
            }
        }

        None
    }
}
