use std::process::exit;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use bevy_ecs::entity::Entity;
use engine::input::key::keycode_converter::keycode_to_str;
use engine::scene::{
    add_entity_to_scene, ReflectTestSuperTrait, Scene, SceneData, SceneError, SceneManager,
    TestSuperTrait,
};
use engine::GameRoot;

use bevy_ecs::{
    reflect::ReflectComponent,
    world::{Mut, World},
};

use bevy_reflect::{DynamicStruct, StructInfo, TypeData};
use engine::scene::ToReflect;
use inquire::{validator::Validation, Confirm, CustomType, InquireError, Select, Text};

const HOME_HELP_TEXT: &str = "
Welcome to my humble abode

Little help is currently available, you'll have to pour through the code yourself

Sorry :\
";

const ACTION_EDITOR_HELP_TEXT: &str = "
This is where you can edit the keybinds of the current action.

* `add` - 
* `exit` - exits the debug CLI, returning to the main loop.
";

fn commands() -> &'static Vec<DebugCommand> {
    static COMMANDS: OnceLock<Vec<DebugCommand>> = OnceLock::new();

    COMMANDS.get_or_init(|| {
        vec![
            DebugCommand::new("help", help, "Shows this"),
            DebugCommand::new(
                "newscene",
                new_scene,
                "Creates a new, blank scene. Will prompt for a name.",
            ),
            DebugCommand::new(
                "savescene",
                save_scene,
                "Saves the target scene to the file specified. Will prompt for a file path if the target scene doesn't have one set.",
            ),
            DebugCommand::new("loadscene", load_scene, "Loads the scene from the given path. Will prompt for a file path."),
            DebugCommand::new("changescene", change_target_scene, "Sets the target scene to the one specified. Will prompt for a scene name."),
            DebugCommand::new("listscenes", list_scenes, "Lists the current scenes that are loaded."),
            DebugCommand::new("listentities", list_entities, "Lists the entites that are in the target scene."),
            DebugCommand::new("newentity", new_entity, "Creates a new, empty entity that you can add components to. Will prompt for a name //later"), //TODO: Give entity name
            DebugCommand::new("scoopentities", scoop_entities, "Pulls any isolated entities into the current target scene. "),
            DebugCommand::new("addcomponent", add_component, "Adds a new component to the entity. Advanced feature, many prompts and can be quite confusing. "),
            DebugCommand::new("crash", crash, "Exits the program instantly, without saving.")
        ]
    })
}

struct DebugCommand {
    command_name: &'static str,
    function: fn(&mut GameRoot) -> Result<(), String>,
    help_text: &'static str,
}

impl DebugCommand {
    pub fn new(
        name: &'static str,
        func: fn(&mut GameRoot) -> Result<(), String>,
        help_text: &'static str,
    ) -> Self {
        Self {
            command_name: name,
            function: func,
            help_text,
        }
    }
}

fn int_prompt_type() -> CustomType<'static, i32> {
    CustomType::new("Whaaa").with_default(0)
}

pub fn debug_cli(root: &mut GameRoot) {
    let char_limit = |input: &str| match input.chars().count() <= 100 {
        true => Ok(Validation::Valid),
        false => Ok(Validation::Invalid(
            "Max command length is 100 characters".into(),
        )),
    };

    let mut action_to_function: HashMap<&str, fn(&mut GameRoot) -> Result<(), String>> =
        HashMap::new();

    for debug_command in commands() {
        action_to_function.insert(debug_command.command_name, debug_command.function);
    }

    loop {
        let mut user_input = match Text::new("Debug >").with_validator(char_limit).prompt() {
            Ok(input) => input,
            Err(err) => {
                println!("Err? [{}]", err);
                continue;
            }
        };
        user_input = user_input.to_lowercase();

        if user_input == String::from("exit") {
            break;
        }
        if let Some(f) = action_to_function.get(user_input.as_str()) {
            if let Err(err) = f(root) {
                eprintln!("{}", err);
            };
        } else {
            println!("Invalid input.");
        }
    }
}

fn help(_: &mut GameRoot) -> Result<(), String> {
    println!("{}", HOME_HELP_TEXT);
    println!("Commands: ");
    for debug_command in commands() {
        println!(
            "   {}: {}",
            debug_command.command_name, debug_command.help_text
        )
    }
    Ok(())
}

fn save_scene(root: &mut GameRoot) -> Result<(), String> {
    match Confirm::new("Are you sure? This will overwrite any previous saved data >").prompt() {
        Ok(_response_yes) if _response_yes => root
            .world
            .resource_scope(|world, mut scene_manager: Mut<SceneManager>| {
                scene_manager.save_scene(world)
            })
            .map_err(|err| "SceneError: ".to_owned() + &err.to_string()),
        Ok(_response_no) => Err("Save aborted.".to_owned()),
        Err(err) => Err(format!(
            "Inquire error! Save aborted. [{}]",
            err.to_string()
        )),
    }
}

fn load_scene(root: &mut GameRoot) -> Result<(), String> {
    let path = match Text::new("Path of scene >").prompt() {
        Ok(ok) => PathBuf::from(ok),
        Err(err) => todo!(),
    };

    root.world.resource_scope(
        |world, mut scene_manager: Mut<SceneManager>| -> Result<(), String> {
            scene_manager.target_scene = Some(
                scene_manager
                    .load_scene(world, path)
                    .map_err(|err| -> String { err.to_string() })?,
            );

            Ok(())
        },
    )
}

fn unload_scene(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world, mut scene_manager: Mut<SceneManager>| -> Result<(), String> {
            scene_manager
                .unload_scene(world)
                .map_err(|err| -> String { err.to_string() })?;

            Ok(())
        },
    )
}

fn reload_scene(root: &mut GameRoot) -> Result<(), String> {
    let path = root
        .world
        .resource_scope(
            |world, mut scene_manager: Mut<SceneManager>| -> Option<PathBuf> {
                world
                    .get::<Scene>(scene_manager.target_scene?)?
                    .save_data_path()
                    .cloned()
            },
        )
        .ok_or(
            SceneError::NoTargetScene.to_string() + " [Error while getting path from target scene]",
        )?;

    unload_scene(root)?;

    root.world.resource_scope(
        |world, mut scene_manager: Mut<SceneManager>| -> Result<(), String> {
            scene_manager.target_scene = Some(
                scene_manager
                    .load_scene(world, path)
                    .map_err(|err| -> String {
                        err.to_string() + "[Error when reloading scene] "
                    })?,
            );

            Ok(())
        },
    )
}

fn new_scene(root: &mut GameRoot) -> Result<(), String> {
    let name = Text::new("What will be the name of the scene? >").prompt();
    let name = match name {
        Ok(ok) => ok,
        Err(err) => return Err(err.to_string()),
    };
    root.world
        .resource_scope(|world: &mut World, mut resource: Mut<SceneManager>| {
            resource.new_scene(world, name)
        });

    Ok(())
}

fn change_target_scene(root: &mut GameRoot) -> Result<(), String> {
    let op = |err: InquireError| -> String { err.to_string() };

    root.world.resource_scope(
        |world: &mut World, mut resource: Mut<SceneManager>| -> Result<(), String> {
            let target = Select::new("message", resource.current_scenes.keys().collect())
                .prompt()
                .map_err(op)?;

            resource.target_scene = resource.current_scenes.get(target).copied();

            Ok(())
        },
    )
}

fn list_scenes(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world: &mut World, mut res: Mut<SceneManager>| -> Result<(), String> {
            let scenestrings = res.current_scenes.keys().collect::<Vec<&String>>();
            if scenestrings.len() == 0 {
                println!("No scenes found!");
                return Ok(());
            }

            for scene in scenestrings {
                println!("\n{}", scene);
                dbg!(res.current_scenes.get(scene));
            }
            Ok(())
        },
    )
}

fn new_entity(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world: &mut World, mut res: Mut<SceneManager>| -> Result<(), String> {
            let scene = world
                .get::<Scene>(res.target_scene.ok_or("No target scene found!")?)
                .ok_or("No scene component found for the current target scene!")?;

            let entity = world.spawn_empty().id();

            engine::scene::add_entity_to_scene(world, res.target_scene.unwrap(), entity);

            Ok(())
        },
    )
}

fn add_component(root: &mut GameRoot) -> Result<(), String> {
    root.world
        .resource_scope(
            |world: &mut World, mut res: Mut<SceneManager>| -> Result<(), SceneError> {
                let scene = world
                    .get::<Scene>(res.target_scene.ok_or(SceneError::NoTargetScene)?)
                    .ok_or(SceneError::NoSceneComponent)?;

                // Display the available components that can be serialized
                for i in res.type_registry.iter() {
                    if i.data::<ReflectTestSuperTrait>().is_some() {
                        println!("{}", i.type_info().type_path())
                    }
                }

                // Create a new component based on user input
                let component_path = Text::new("Component path > ").prompt().unwrap();

                let component_registration: &bevy_reflect::TypeRegistration = res
                    .type_registry
                    .get_with_type_path(&component_path)
                    .ok_or(SceneError::MissingTypeRegistry(component_path.clone()))?;

                let (component_patch) = match component_registration.type_info() {
                    bevy_reflect::TypeInfo::Struct(info) => {
                        create_dynamic_struct(info, component_registration, component_path)?
                    }
                    bevy_reflect::TypeInfo::TupleStruct(info) => todo!(),
                    bevy_reflect::TypeInfo::Tuple(info) => todo!(),
                    bevy_reflect::TypeInfo::List(info) => todo!(),
                    bevy_reflect::TypeInfo::Array(info) => todo!(),
                    bevy_reflect::TypeInfo::Map(info) => todo!(),
                    bevy_reflect::TypeInfo::Enum(info) => todo!(),
                    bevy_reflect::TypeInfo::Value(info) => todo!(),
                };

                let reflect_component = component_registration.data::<ReflectComponent>().unwrap();

                // Figure out the entity to add the new component to.
                let entity = loop {
                    let name = Text::new("What entity do you want to add it to? > ")
                        .prompt()
                        .unwrap();

                    if name == "exit" {
                        return Err(SceneError::SerializeFailure(
                            "Aborted before entity could be instantiated".to_owned(),
                        ));
                    }

                    let entity = scene
                        .get_entity(world, name)
                        .ok_or("The entity does not exist!");

                    if let Ok(entity) = entity {
                        break entity;
                    }

                    println!("{}", entity.unwrap_err());
                };

                let mut entity = world.entity_mut(entity);

                reflect_component.apply_or_insert(&mut entity, dbg!(&component_patch));

                Ok(())
            },
        )
        .map_err(|err| err.to_string())
}

/// Creates a new component patch based on the given struct information, and constructs new field data from user input.
fn create_dynamic_struct(
    struct_info: &StructInfo,
    component_registration: &bevy_reflect::TypeRegistration,
    component_path: String,
) -> Result<(DynamicStruct), SceneError> {
    let mut component_patch = DynamicStruct::default();

    component_patch.set_represented_type(Some(component_registration.type_info()));

    let mut component_data: HashMap<&str, serde_json::Value> = HashMap::new();

    // Insert data for each field
    let field_names = for field_name in struct_info.field_names() {
        let field_type = struct_info.field(&field_name).unwrap().type_path();

        component_data.insert(
            &field_name,
            serde_json::from_str(
                &Text::new(&format!(
                    "Enter value for field [{}: {}] > ",
                    field_name, field_type
                ))
                .prompt()
                .unwrap(),
            )
            .map_err(|err| SceneError::SerializeFailure(err.to_string()))?,
        );
    };

    // Insert component data into patch
    for (name, field) in &component_data {
        // Find expected type to make disambiguation easier, ex. figure out if a json number should be an i32, f64, u8, etc.
        let expected_type = struct_info
            .field(name)
            .ok_or(SceneError::MissingTypeRegistry(component_path.clone()))?
            .type_path();

        let value = match field.to_reflect(Some(expected_type)) {
            Ok(ok) => ok,
            Err(ok) => ok,
        };

        component_patch.insert_boxed(&name, value);
    }

    Ok(component_patch)
}

fn list_entities(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world: &mut World, mut res: Mut<SceneManager>| -> Result<(), String> {
            let scene = world
                .get::<Scene>(res.target_scene.ok_or("No target scene found!")?)
                .ok_or("No scene component found for the current target scene!")?;

            if scene.get_entities().len() == 0 {
                println!("No entities found!");
                return Ok(());
            }

            for entity in scene.get_entities().clone() {
                let scene_data = match world.get::<SceneData>(entity) {
                    Some(ok) => ok,
                    None => continue,
                };

                println!("{}", scene_data.object_name);
            }

            Ok(())
        },
    )
}

fn scoop_entities(root: &mut GameRoot) -> Result<(), String> {
    root.world
        .resource_scope(
            |world: &mut World, res: Mut<SceneManager>| -> Result<(), SceneError> {
                let scene_id = world
                    .get::<Scene>(res.target_scene.ok_or(SceneError::NoTargetScene)?)
                    .ok_or(SceneError::NoSceneComponent)?
                    .scene_id;

                let mut entities_to_push: Vec<Entity> = Vec::new();

                for mut entity_mut in world.iter_entities_mut() {
                    if let Some(mut data) = entity_mut.get_mut::<SceneData>() {
                        if let None = data.scene_id {
                            data.scene_id = Some(scene_id);
                            entities_to_push.push(entity_mut.id())
                        }
                    }
                }

                for entity in entities_to_push {
                    add_entity_to_scene(world, res.target_scene.unwrap(), entity)?;
                }

                Ok(())
            },
        )
        .map_err(|err| err.to_string())
}

fn crash(root: &mut GameRoot) -> Result<(), String> {
    exit(1);
    Err("failed to crash??".to_owned()) // lie to the compiler >:)))
                                        //but if this returns err that means something seriously went wrong
}
