use analysis::{Description, Enum, Function};
use synthesis::{TestClass, TestFunction};

////////////////////////////////////////////////////////////////////////////////

macro_rules! implement_conversion {
    ($t:ident) => {
        impl EntityConversion for $t {
            fn convert(entity_type: &EntityType) -> Option<&$t> {
                match entity_type {
                    EntityType::$t(entity) => Some(entity),
                    _ => None,
                }
            }

            fn convert_mut(entity_type: &mut EntityType) -> Option<&mut $t> {
                match entity_type {
                    EntityType::$t(entity) => Some(entity),
                    _ => None,
                }
            }
        }
    };
}

/// The different types an `Entitiy` can have.
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
    fn convert(entity_type: &EntityType) -> Option<&Self>;
    fn convert_mut(entity_type: &mut EntityType) -> Option<&mut Self>;
}

implement_conversion!(Entity);
implement_conversion!(Enum);
implement_conversion!(Function);
implement_conversion!(TestClass);
implement_conversion!(TestFunction);

////////////////////////////////////////////////////////////////////////////////

/// The representation of an `Entity` as a possbile generic node on the
/// abstract syntax tree. An `Entity` has to be kind of an `EntityType`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entities: Vec<EntityType>,
    pub description: Option<Description>,
}

impl Entity {
    /// Creates a new `Entity` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use thinlinelib::entity::Entity;
    ///
    /// let class = Entity::new("testClass");
    ///
    /// assert_eq!(class.name, "testClass");
    /// assert!(class.entities.is_empty());
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            entities: Vec::new(),
            description: None,
        }
    }

    /// Returns a reference to the Entity encapsulated within the EntityType.
    pub fn ref_from_entity_type<T>(entity_type: &EntityType) -> Option<&T>
    where
        T: EntityConversion,
    {
        T::convert(entity_type)
    }

    /// Returns a mutable reference to the Entity encapsulated within the EntityType.
    pub fn ref_mut_from_entity_type<T>(entity_type: &mut EntityType) -> Option<&mut T>
    where
        T: EntityConversion,
    {
        T::convert_mut(entity_type)
    }

    /// Adds an `Entity` to the `Entity` instance.
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
    /// assert_eq!(entity.entities.len(), 1);
    /// ```
    pub fn add_entity<T>(&mut self, entity: EntityType) -> Option<&mut T>
    where
        T: EntityConversion,
    {
        self.entities.push(entity);
        if let Some(entity) = self.entities.last_mut() {
            return Entity::ref_mut_from_entity_type(entity);
        }

        None
    }

    /// Returns the functions of an `Entity`.
    pub fn functions(&self) -> Vec<&Function> {
        let mut entity_vec: Vec<&Function> = Vec::new();
        for entity in &self.entities {
            if let EntityType::Function(fct) = entity {
                entity_vec.push(&fct);
            }
        }
        return entity_vec;
    }

    /// Sets the description for the `Entity`.
    pub fn set_description(&mut self, description: &str) {
        if self.description.is_none() {
            self.description = Some(Description::new());
        }

        if let Some(desc) = &mut self.description {
            desc.set(description);
        }
    }
}
