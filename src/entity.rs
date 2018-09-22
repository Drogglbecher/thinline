use analysis::{Description, Enum, Function};
use synthesis::{TestClass, TestFunction};

macro_rules! implement_conversion {
    ($t:ident) => {
        impl EntityConversion for $t {
            fn convert(entity_type: &mut EntityType) -> Option<&mut $t> {
                match entity_type {
                    EntityType::$t(entity) => Some(entity),
                    _ => None,
                }
            }
        }
    };
}

/// The different types an Entitiy can have.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    /// The index of a new entity hierarchy.
    Entity(Entity),

    /// An enumeration.
    Enum(Enum),

    /// A function.
    Function(Function),

    /// A test class.
    TestClass(TestClass),

    /// A test function.
    TestFunction(TestFunction),
}

pub trait EntityConversion {
    fn convert(entity_type: &mut EntityType) -> Option<&mut Self>;
}

implement_conversion!(Entity);
implement_conversion!(Enum);
implement_conversion!(Function);
implement_conversion!(TestClass);
implement_conversion!(TestFunction);

fn convert<T>(entity_type: &mut EntityType) -> Option<&mut T>
where
    T: EntityConversion,
{
    T::convert(entity_type)
}

/// The representation of an Entity as a possbile generic node on the
/// abstract syntax tree. An Entity has to be kind of a EntityType.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entities: Option<Vec<EntityType>>,
    pub description: Option<Description>,
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
    /// assert_eq!(class.name, "testClass");
    /// assert!(class.entities.is_none());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            entities: None,
            description: None,
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
    /// let entity_type = EntityType::Entity(Entity::new("inner_entity"));
    /// entity.add_entity::<Entity>(entity_type);
    ///
    /// assert!(entity.entities.is_some());
    /// ```
    pub fn add_entity<T>(&mut self, entity: EntityType) -> Option<&mut T>
    where
        T: EntityConversion,
    {
        if self.entities.is_none() {
            self.entities = Some(Vec::new());
        }

        if let Some(entities) = &mut self.entities {
            entities.push(entity);
            if let Some(entity) = entities.last_mut() {
                return convert(entity);
            }
        }

        None
    }

    /// Returns the functions of an entity.
    pub fn functions(&self) -> Option<Vec<&Function>> {
        if let Some(entities) = &self.entities {
            let mut entity_vec: Vec<&Function> = Vec::new();
            for entity in entities {
                if let EntityType::Function(fct) = entity {
                    entity_vec.push(&fct);
                }
            }
            return Some(entity_vec);
        }

        None
    }

    /// Sets the description for the Entity.
    pub fn set_description(&mut self, description: &str) {
        if self.description.is_none() {
            self.description = Some(Description::new());
        }

        if let Some(desc) = &mut self.description {
            desc.set_description(description);
        }
    }
}
