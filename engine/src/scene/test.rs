#![allow(unused_imports)]
use crate::{
    scene::{
        component::{self, Scene},
        serialize::SerializedSceneData,
        traits::{SceneData, TestSuperTrait},
    },
    space::transform,
};
use bevy_ecs::{
    component::{Component, ComponentDescriptor},
    world::World,
};
use bevy_reflect::TypeRegistry;
use bevy_trait_query::TraitQuery;
use serde::Serialize;
use std::{fs::File, io::Write};

#[derive(Debug, Component, Serialize, bevy_reflect::Reflect)]
struct TestComponent {
    serialize_value_1: String,
    serialize_value_2: i32,
}

#[derive(Debug, Component, Serialize, bevy_reflect::Reflect)]
struct TestComponent2 {
    ineedafunnynameforthis: String,
    peepeedoodookaka: bool,
    the_real_test: char,
}

impl TraitQuery for TestComponent {}

#[test]
fn scene_test() {
    // Init world
    let mut world = World::new();
    super::register_scene_types(&mut world);

    world.init_component::<TestComponent>();
    world.init_component::<TestComponent2>();

    // Init component traits for querying
    use bevy_trait_query::RegisterExt;
    world.register_component_as::<dyn TestSuperTrait, TestComponent>();
    world.register_component_as::<dyn TestSuperTrait, TestComponent2>();

    let mut registery = TypeRegistry::empty();
    let mut registery = TypeRegistry::default();
    registery.register::<SceneData>();
    registery.register::<TestComponent>();
    registery.register::<TestComponent2>();

    // Init scene
    let mut scene_component: Scene = Scene::new("TestScene".to_string());
    let mut scene_entity_id = World::spawn(&mut world, scene_component).id();

    // At an unknown amount of time later, create an entity
    let test_entity = world.spawn(transform::Transform::default()).id().clone();

    let test_component = TestComponent {
        serialize_value_1: String::from("My name is :3"),
        serialize_value_2: 654101, // sixty ie fo te ti
    };

    world.entity_mut(test_entity).insert(test_component);

    // Another one for good measure
    let test_entity2 = world.spawn(transform::Transform::default()).id().clone();

    world.entity_mut(test_entity2).insert(TestComponent {
        serialize_value_1: String::from("My name is :3c"),
        serialize_value_2: 4226363,
    });

    world.entity_mut(test_entity).insert(TestComponent2 {
        ineedafunnynameforthis: String::new(),
        peepeedoodookaka: true,
        the_real_test: ' ',
    });

    // Add the test entitys to the scene
    let mut scene_entity = World::entity_mut(&mut world, scene_entity_id).id();

    if let Err(err) = component::add_entity_to_scene(&mut world, scene_entity, test_entity) {
        panic!("Adding entity failed! [{}]", err)
    }
    if let Err(err) = component::add_entity_to_scene(&mut world, scene_entity, test_entity2) {
        panic!("Adding entity failed! [{}]", err)
    }

    // Serialize the scene
    let mut entity_mut = World::entity_mut(&mut world, scene_entity).id();

    let serialized_scene =
        match component::to_serialized_scene(&mut world, entity_mut, &mut registery) {
            Ok(serialized_scene) => serialized_scene,
            Err(err) => panic!("Serializing scene failed! [{}]", err),
        };

    let msg = "jesoon should have serialized properly";
    let to_string: String = serde_json::to_string(&serialized_scene).expect(msg);

    println!(
        "The stringified jesoon, to be stored in file: {}",
        to_string
    );

    // Save txt for analysis
    let mut path_buf = std::env::current_dir().unwrap();
    path_buf.pop();
    path_buf.push("test_output");
    path_buf.push("scene_serialization.json");
    println!("{}", path_buf.to_str().unwrap());
    File::create(path_buf).unwrap().write(to_string.as_bytes());

    // Assume we make some sys calls to store and retrieve that data here
    let msg = "jesoon should have deserialized properly";
    let serialized_scene: SerializedSceneData =
        dbg!(serde_json::from_str(&to_string.clone()).expect(msg));

    // Deserialized the retrieved data
    let result_scene: Scene =
        SerializedSceneData::initialize(serialized_scene, &mut world).unwrap();

    // Double check that the scene serialized and deserialized properly
    let initial_scene: &Scene = world.get(scene_entity_id).unwrap();
    let var = Scene::entity_eq(initial_scene, &result_scene, &world);
    if !var {
        dbg!(initial_scene);
        dbg!(result_scene);
        dbg!(world);
    }
    assert!(var); // Fails if serde_json::from_str::<SerializedScene>(&to_string.clone()) returns incorrect scene data.
}
