extern crate thinlinelib;

pub static MULTILINE_COMMENT: &str = "
**this
is
a

multiline
**
//comment";

#[cfg(test)]
mod entity {
    use thinlinelib::entity::{Entity, EntityType};
    use thinlinelib::analysis::{Enum, Function};
    use MULTILINE_COMMENT;

    #[test]
    fn new() {
        let entity = Entity::new("ent");

        assert_eq!(entity.name, String::from("ent"));
        assert!(entity.entities.is_none());
    }

    #[test]
    fn add_entity() {
        {
            let mut entity = Entity::new("ent");
            assert!(entity.entities.is_none());

            let entity_type = EntityType::Entity(Entity::new("inner_entity"));
            assert!(entity.add_entity::<Entity>(entity_type).is_some());

            let fct = EntityType::Function(Function::new("fct"));
            assert!(entity.add_entity::<Function>(fct).is_some());

            assert!(entity.entities.is_some());

            let inner_entities = entity.entities.unwrap();
            assert_eq!(inner_entities.len(), 2);
        }

        {
            let mut entity = Entity::new("ent");
            assert!(entity.entities.is_none());

            let enumeration = EntityType::Enum(Enum::new("enumeration"));
            let ret = entity.add_entity::<Enum>(enumeration);

            assert!(ret.is_some());
            assert_eq!(ret.unwrap().name, "enumeration");
        }
    }

    #[test]
    fn functions() {
        let mut entity = Entity::new("ent");

        {
            assert!(entity.functions().is_none());
        }

        {
            let fct1 = EntityType::Function(Function::new("fct1"));
            assert!(entity.add_entity::<Function>(fct1).is_some());

            let fct2 = EntityType::Function(Function::new("fct2"));
            assert!(entity.add_entity::<Function>(fct2).is_some());

            let functions_option = entity.functions();

            assert!(functions_option.is_some());
            let functions = functions_option.unwrap();

            assert_eq!(functions.len(), 2);
            assert_eq!(functions[0].name, "fct1");
            assert_eq!(functions[1].name, "fct2");
        }
    }

    #[test]
    fn set_description() {
        let mut entity = Entity::new("ent");

        {
            assert!(entity.description.is_none());
        }

        {
            entity.set_description(MULTILINE_COMMENT);

            assert!(entity.description.is_some());
            let entity_desc = entity.description.unwrap().description;

            assert_eq!(entity_desc.len(), 5);
            assert_eq!(entity_desc[0], "this");
            assert_eq!(entity_desc[1], "is");
            assert_eq!(entity_desc[2], "a");
            assert_eq!(entity_desc[3], "multiline");
            assert_eq!(entity_desc[4], "comment");
        }
    }
}
