#![allow(unused)]

//! Used for converting data into components
//!

use std::collections::HashMap;

use crate::scene::{SceneError, ToReflect};
use bevy_reflect::{
    DynamicStruct, DynamicTupleStruct, EnumInfo, ReflectOwned, StructInfo, TupleInfo,
    TupleStructInfo, TypeRegistry, UnnamedField, ValueInfo,
};
use inquire::Text;

/// Creates a new component patch based on the given struct information, and constructs new field data from user input.
pub fn new_dyn_struct(
    struct_info: &StructInfo,
    component_registration: &bevy_reflect::TypeRegistration,
    component_path: &str,
    type_registry: &TypeRegistry,
) -> Result<ReflectOwned, SceneError> {
    let mut component_patch = DynamicStruct::default();

    component_patch.set_represented_type(Some(component_registration.type_info()));

    let mut component_data: HashMap<&str, serde_json::Value> = HashMap::new();

    // Insert data for each field
    for field_name in struct_info.field_names() {
        let field_type = struct_info.field(&field_name).unwrap().type_path();

        component_data.insert(
            &field_name,
            serde_json::from_str(
                &Text::new(&format!(
                    "Enter value for field [{}: {}] >",
                    field_name, field_type
                ))
                .prompt()
                .unwrap(),
            )
            .map_err(|err| SceneError::SerializeFailure(err.to_string()))?,
        );
    }

    // Insert component data into patch
    for (name, field) in &component_data {
        // Find expected type to make disambiguation easier, ex. figure out if a json number should be an i32, f64, u8, etc.
        let expected_type = struct_info
            .field(name)
            .ok_or(SceneError::MissingTypeRegistry(component_path.to_owned()))?
            .type_path();

        let value = match field.to_reflect(Some(expected_type), type_registry) {
            Ok(ok) => ok,
            Err(ok) => ok,
        };

        component_patch.insert_boxed(&name, value);
    }

    Ok(ReflectOwned::Struct(Box::new(component_patch)))
}

///
pub fn new_dyn_tuple_struct(
    tuple_struct_info: &TupleStructInfo,
    component_registration: &bevy_reflect::TypeRegistration,
    component_path: &str,
    type_registry: &TypeRegistry,
    insert_value_for_field: impl Fn(UnnamedField) -> String,
) -> Result<ReflectOwned, SceneError> {
    let mut component_patch = DynamicTupleStruct::default();

    component_patch.set_represented_type(Some(component_registration.type_info()));

    let mut component_data: HashMap<usize, serde_json::Value> = HashMap::new();

    // Insert data for each field
    for unnamed_field in tuple_struct_info.iter() {
        let index = unnamed_field.index();

        let field: serde_json::Value =
            serde_json::from_str(&insert_value_for_field(unnamed_field.clone()))
                .map_err(|err| SceneError::SerializeFailure(err.to_string()))?;

        // Insert component data into patch

        // Find expected type to make disambiguation easier, ex. figure out if a json number should be an i32, f64, u8, etc.
        let expected_type = tuple_struct_info
            .field_at(index)
            .ok_or(SceneError::MissingTypeRegistry(component_path.to_owned()))?
            .type_path();

        let value = match field.to_reflect(Some(expected_type), type_registry) {
            Ok(ok) => ok,
            Err(ok) => ok,
        };

        component_patch.insert_boxed(value);
    }

    Ok(ReflectOwned::TupleStruct(Box::new(component_patch)))
}

pub fn new_dyn_tuple(
    struct_info: &TupleInfo,
    component_registration: &bevy_reflect::TypeRegistration,
    component_path: &str,
) -> Result<ReflectOwned, SceneError> {
    todo!()
}

pub fn new_dyn_enum(
    struct_info: &EnumInfo,
    component_registration: &bevy_reflect::TypeRegistration,
    component_path: &str,
) -> Result<ReflectOwned, SceneError> {
    todo!()
}

pub fn new_dyn_value(
    struct_info: &ValueInfo,
    component_registration: &bevy_reflect::TypeRegistration,
    component_path: &str,
) -> Result<ReflectOwned, SceneError> {
    todo!()
}
