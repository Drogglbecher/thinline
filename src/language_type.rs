use analysis::{Analysis, Argument, Enum, Function};
use clang;
use entity::{Entity, EntityType};
use failure::{err_msg, Fallible};
use python_parser::ast::{CompoundStatement, Expression, Statement};
use python_parser::{file_input, make_strspan};
use std::fs::File;
use std::io::Read;

////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref CLANG: Option<clang::Clang> = {
        match clang::Clang::new() {
            Ok(clang) => Some(clang),
            Err(_) => None,
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

pub trait LanguageType: Default {
    fn file_types() -> &'static [&'static str];
    fn extract_entities<T: LanguageType>(analysis: &Analysis<T>) -> Fallible<()>;
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Clone, Debug)]
struct CFamily;

impl CFamily {
    fn format_arguments(arguments: &[clang::Entity]) -> Fallible<Vec<Argument>> {
        let mut args = Vec::new();

        for argument in arguments {
            args.push(Argument::new(
                argument.get_display_name().unwrap_or(String::new()),
                Some(
                    argument
                        .get_type()
                        .ok_or_else(|| err_msg("Argument type can not be parsed from signature."))?
                        .get_display_name(),
                ),
            ));
        }

        Ok(args)
    }

    fn analyse_clang_function_entity(entity: &clang::Entity) -> Fallible<Option<EntityType>> {
        if let Some(entity_name) = entity.get_name() {
            let mut function = Function::new(entity_name);

            // Set return type.
            if let Some(return_type) = entity.get_type() {
                function.set_return_type(return_type.get_display_name().as_str())?;
            }

            // Set arguments vector.
            if let Some(arguments) = entity.get_arguments() {
                function.set_arguments(&Self::format_arguments(&arguments)?);
            }

            // Set description.
            if let Some(description) = entity.get_comment() {
                function.set_description(description.as_str());
            }

            return Ok(Some(EntityType::Function(function)));
        }

        Ok(None)
    }

    fn analyse_clang_enum_entity(entity: &clang::Entity) -> Fallible<Option<EntityType>> {
        if let Some(entity_name) = entity.get_name() {
            let enumeration = Enum::new(entity_name);

            return Ok(Some(EntityType::Enum(enumeration)));
        }

        Ok(None)
    }

    fn analyse_clang_generic_entity(entity: &clang::Entity) -> Fallible<Option<EntityType>> {
        if let Some(entity_name) = entity.get_name() {
            let entity = Entity::new(entity_name);

            return Ok(Some(EntityType::Entity(entity)));
        }

        Ok(None)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The file extensions which should be checked for C project analysis.
static C_FILE_EXTENSIONS: &[&str] = &["c", "h"];

#[derive(Default, Clone, Debug)]
pub struct C;

impl C {
    fn analyse_clang_entity(entity: &clang::Entity) -> Fallible<Option<EntityType>> {
        let entity_kind = entity.get_kind();

        // Search for functions outside the system headers
        if !entity.is_in_system_header() {
            match &entity_kind {
                clang::EntityKind::FunctionDecl => {
                    return CFamily::analyse_clang_function_entity(entity);
                }
                clang::EntityKind::EnumDecl => {
                    return CFamily::analyse_clang_enum_entity(entity);
                }
                _ => {}
            }

            return Ok(None);
        }

        Ok(None)
    }
}

impl LanguageType for C {
    fn file_types() -> &'static [&'static str] {
        C_FILE_EXTENSIONS
    }

    fn extract_entities<C: LanguageType>(analysis: &Analysis<C>) -> Fallible<()> {
        if let Some(ref clang) = *CLANG {
            let clang_index = clang::Index::new(&clang, false, false);
            for project_file in analysis.project_files().iter() {
                if let EntityType::Entity(mut index) = EntityType::Entity(Entity::new("")) {
                    let parsed_path = &clang_index.parser(&project_file.path).parse()?;
                    let clang_entity = parsed_path.get_entity();

                    // Iterate through the child entities of the current entity
                    for child in clang_entity.get_children() {
                        if let Ok(Some(entity)) = Self::analyse_clang_entity(&child) {
                            index.add_entity::<Entity>(entity);
                        }
                    }

                    println!("{:#?}", index);
                    project_file.add_entity(index);
                }
            }
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The file extensions which should be checked for C++ project analysis.
static CPP_FILE_EXTENSIONS: &[&str] = &["cpp", "hpp"];

#[derive(Default, Clone, Debug)]
pub struct Cpp;

impl Cpp {
    fn analyse_clang_entity(entity: &clang::Entity) -> Fallible<Option<EntityType>> {
        let entity_kind = entity.get_kind();

        // Search for functions outside the system headers
        if !entity.is_in_system_header() {
            match &entity_kind {
                clang::EntityKind::Constructor
                | clang::EntityKind::Destructor
                | clang::EntityKind::Method
                | clang::EntityKind::FunctionDecl => {
                    return CFamily::analyse_clang_function_entity(entity);
                }
                clang::EntityKind::EnumDecl => {
                    return CFamily::analyse_clang_enum_entity(entity);
                }
                clang::EntityKind::ClassDecl | clang::EntityKind::Namespace => {
                    return CFamily::analyse_clang_generic_entity(entity);
                }
                _ => {}
            }

            return Ok(None);
        }

        Ok(None)
    }

    fn analyse_clang_entity_tree(parent: &mut Entity, clang_entity: &clang::Entity) -> Fallible<()> {
        // Iterate through the child entities of the current entity
        for child in clang_entity.get_children() {
            if let Ok(Some(entity)) = Self::analyse_clang_entity(&child) {
                if let Some(added_entity) = parent.add_entity(entity) {
                    Self::analyse_clang_entity_tree(added_entity, &child)?;
                }
            }
        }

        Ok(())
    }
}

impl LanguageType for Cpp {
    fn file_types() -> &'static [&'static str] {
        CPP_FILE_EXTENSIONS
    }

    fn extract_entities<Cpp: LanguageType>(analysis: &Analysis<Cpp>) -> Fallible<()> {
        if let Some(ref clang) = *CLANG {
            let clang_index = clang::Index::new(&clang, false, false);
            for project_file in analysis.project_files().iter() {
                if let EntityType::Entity(mut index) = EntityType::Entity(Entity::new("")) {
                    let parsed_path = &clang_index.parser(&project_file.path).parse()?;
                    let clang_entity = parsed_path.get_entity();

                    Self::analyse_clang_entity_tree(&mut index, &clang_entity)?;

                    println!("{:#?}", index);
                    project_file.add_entity(index);
                }
            }
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The file extensions which should be checked for Python project analysis.
static PYTHON_FILE_EXTENSIONS: &[&str] = &["py"];

#[derive(Default, Clone, Debug)]
pub struct Python;

impl Python {
    fn extract_function_doc(function: &mut Function, statement: &Statement) {
        if let Statement::Assignment(ent_v, _) = statement {
            for ent in ent_v.iter() {
                if let Expression::String(expr_v) = ent {
                    for expr in expr_v.iter() {
                        function.set_description(&expr.content.to_string_lossy());
                    }
                }
            }
        }
    }

    fn analyse_statement(entity: &mut Entity, statement: &Statement) -> Fallible<()> {
        if let Statement::Compound(ent_box) = statement {
            match Box::leak((*ent_box).clone()) {
                // Statement is a statement definition
                CompoundStatement::Funcdef(expr) => {
                    let mut function: Function = Function::new(expr.name.as_str());

                    // Split arguments and add them to the function
                    let mut arguments: Vec<Argument> = Vec::new();
                    for arg in &expr.parameters.positional_args {
                        arguments.push(Argument::new(arg.0.as_str(), None));
                    }
                    function.set_arguments(&arguments);

                    if let Some(mut function_inst) =
                        entity.add_entity(EntityType::Function(function))
                    {
                        Self::extract_function_doc(&mut function_inst, &expr.code[0]);
                    }
                }

                // Statement is a class definition
                CompoundStatement::Classdef(expr) => {
                    if let Some(ref mut class_entity) =
                        entity.add_entity(EntityType::Entity(Entity::new(expr.name.as_str())))
                    {
                        for code in &expr.code {
                            Self::analyse_statement(class_entity, &code)?;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl LanguageType for Python {
    fn file_types() -> &'static [&'static str] {
        PYTHON_FILE_EXTENSIONS
    }

    fn extract_entities<Python: LanguageType>(analysis: &Analysis<Python>) -> Fallible<()> {
        for project_file in analysis.project_files().iter() {
            // Parse file to string
            let mut file = File::open(&project_file.path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            if let EntityType::Entity(mut index) = EntityType::Entity(Entity::new("")) {
                match file_input(make_strspan(content.as_str())) {
                    Ok(ast) => {
                        for entity in ast.1.iter() {
                            Self::analyse_statement(&mut index, entity)?;
                        }
                    }
                    Err(_) => bail!("Unable to create python AST."),
                }
                println!("{:#?}", index);
                project_file.add_entity(index);
            }
        }

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod c {
    use super::{Analysis, LanguageType, C, C_FILE_EXTENSIONS};

    #[test]
    fn new() {
        let analysis: Analysis<C> = Analysis::new();

        assert_eq!(analysis.file_types, C_FILE_EXTENSIONS);
        assert_eq!(analysis.project_files().len(), 0);
    }

    #[test]
    fn file_types() {
        assert_eq!(C::file_types(), C_FILE_EXTENSIONS);
    }
}

#[cfg(test)]
mod cpp {
    use super::{Analysis, Cpp, LanguageType, CPP_FILE_EXTENSIONS};

    #[test]
    fn new() {
        let analysis: Analysis<Cpp> = Analysis::new();

        assert_eq!(analysis.file_types, CPP_FILE_EXTENSIONS);
        assert_eq!(analysis.project_files().len(), 0);
    }

    #[test]
    fn file_types() {
        assert_eq!(Cpp::file_types(), CPP_FILE_EXTENSIONS);
    }
}

#[cfg(test)]
mod python {
    use super::{Analysis, LanguageType, Python, PYTHON_FILE_EXTENSIONS};

    #[test]
    fn new() {
        let analysis: Analysis<Python> = Analysis::new();

        assert_eq!(analysis.file_types, PYTHON_FILE_EXTENSIONS);
        assert_eq!(analysis.project_files().len(), 0);
    }

    #[test]
    fn file_types() {
        assert_eq!(Python::file_types(), PYTHON_FILE_EXTENSIONS);
    }
}
