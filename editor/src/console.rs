#![allow(unused)]
use bevy_ecs::entity::Entity;
use bevy_ecs::system::{Res, ResMut};
use bevy_ecs::{
    reflect::ReflectComponent,
    world::{Mut, World},
};
use bevy_reflect::{DynamicStruct, DynamicTupleStruct, ReflectOwned, TypeInfo};
use engine::{
    scene::{self, *},
    LogData,
};
use engine::{GameRoot, Input};
use inquire::{validator::Validation, Confirm, InquireError, Select, Text};
use log::error;
use std::any::Any;
use std::panic::PanicInfo;
use std::process::exit;
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

fn commands() -> &'static Vec<DebugCommand> {
    static COMMANDS: OnceLock<Vec<DebugCommand>> = OnceLock::new();

    COMMANDS.get_or_init(|| {
        vec![
            DebugCommand::new("help", help, "Shows this"),
            DebugCommand::new("log", log, "Logs as much info as possible into the standard output"),
            DebugCommand::new(
                "newscene",
                new_scene,
                "Creates a new, blank scene. Will prompt for a name.",
            ),
            DebugCommand::new("listscenes", list_scenes, "Lists the current scenes that are loaded."),
            DebugCommand::new(
                "savescene",
                save_scene,
                "Saves the target scene to the file specified. Will prompt for a file path if the target scene doesn't have one set.",
            ),
            DebugCommand::new("loadscene", load_scene, "Loads the scene from the given path. Will prompt for a file path."),
            DebugCommand::new("unloadscene", unload_scene, "Unloads the current target scene. Does not save, any unsaved data will be lost."),
            DebugCommand::new("reloadscene", reload_scene, "Reloads the current target scene, pulling in data from the save file. Does not save, any unsaved data will be lost."),
            DebugCommand::new("changescene", change_target_scene, "Sets the target scene to the one specified. Will prompt for a scene name."),
            DebugCommand::new("newentity", new_entity, "Creates a new, empty entity that you can add components to. Will prompt for a name //later"), //TODO: Give entity name
            DebugCommand::new("listentities", list_entities, "Lists the entites that are in the target scene."),
            DebugCommand::new("scoopentities", scoop_entities, "Pulls any isolated entities into the current target scene. "),
            DebugCommand::new("addcomponent", add_component, "Adds a new component to the entity. Advanced feature, many prompts and can be quite confusing. "),
            DebugCommand::new("listcomponents", list_components, "Displays every currently instantiated component, as well as the entity it belongs to."),
            DebugCommand::new("crash", crash, "Exits the program instantly, without saving.")
            ]
    })
}

pub fn check_for_debug(input: Res<Input>, mut engine: ResMut<engine::GgezInterface>) {
    if let Some(action) = input.get_action("debugconsole") {
        if action.status().is_just_pressed() {
            debug_cli(unsafe { &mut *SECRET_ROOT_PTR })
        }
    }
}

/// This is to be used purely by the panic handler and debug console, as an escape mechanism to save your data before it is lost.
/// Do not use this under any other circumstances, doing so will result in behaviour that I don't care to define.
static mut SECRET_ROOT_PTR: *mut GameRoot = std::ptr::null_mut();
pub fn hook_emergency_panic_handler(root: &mut GameRoot) {
    unsafe { SECRET_ROOT_PTR = root };

    std::panic::set_hook(Box::new(|panic_info: &std::panic::PanicInfo| {
        println!(
            "\n{}\nEntering debug console to rescue data.",
            // panic_info.location().unwrap(),
            panic_info.to_string()
        );
        panic_debug_cli(unsafe { &mut *SECRET_ROOT_PTR }, panic_info);
        // panic_info.log();
    }));
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

pub fn panic_debug_cli(root: &mut GameRoot, panic_info: &PanicInfo) {
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
        let mut user_input = match Text::new("Panic debug >")
            .with_validator(char_limit)
            .prompt()
        {
            Ok(input) => input,
            Err(err) => {
                let _ = match err {
                    InquireError::OperationCanceled => break,
                    InquireError::OperationInterrupted => crash(root),
                    _ => {
                        println!("Err? [{}]", err);
                        Ok(())
                    }
                };
                continue;
            }
        };
        user_input = user_input.to_lowercase();

        if (&user_input == "exit") || (&user_input == "") {
            break;
        }
        if &user_input == "logbacktrace" {
            let var = std::env::var("RUST_BACKTRACE");
            if var == Ok("1".to_owned()) {
                log::trace!("{}", std::backtrace::Backtrace::capture());
            } else {
                std::env::set_var("RUST_BACKTRACE", "1");
                log::trace!("{}", std::backtrace::Backtrace::capture());
                if var.is_ok() {
                    std::env::set_var("RUST_BACKTRACE", "0");
                } else {
                    std::env::set_var("RUST_BACKTRACE", "");
                }
                continue;
            }
        }
        if let Some(f) = action_to_function.get(user_input.as_str()) {
            if let Err(err) = f(root) {
                println!("Operation error");
                error!("{}", err);
            };
        } else {
            println!("Invalid input.");
        }
    }

    println!("Exiting debug console");
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
                let _ = match err {
                    InquireError::OperationCanceled => break,
                    InquireError::OperationInterrupted => crash(root),
                    _ => {
                        println!("Err? [{}]", err);
                        Ok(())
                    }
                };
                continue;
            }
        };
        user_input = user_input.to_lowercase();

        if (&user_input == "exit") || (&user_input == "") {
            break;
        }
        if let Some(f) = action_to_function.get(user_input.as_str()) {
            if let Err(err) = f(root) {
                println!("Operation error");
                error!("{}", err);
            };
        } else {
            println!("Invalid input.");
        }
    }

    println!("Exiting debug console");
}

// Misc commands

fn help(_: &mut GameRoot) -> Result<(), String> {
    // println!("{}", HOME_HELP_TEXT);
    println!("Commands: ");
    for debug_command in commands() {
        println!(
            "   {}: {}",
            debug_command.command_name, debug_command.help_text
        )
    }
    Ok(())
}

fn crash(_root: &mut GameRoot) -> Result<(), String> {
    exit(1);
    #[allow(unreachable_code)]
    Err("failed to crash??".to_owned()) // lie to the compiler >:)))
                                        //but if this returns err that means something seriously went wrong
}

// Scene

fn save_scene(root: &mut GameRoot) -> Result<(), String> {
    match Confirm::new("Are you sure? This will overwrite any previous saved data >").prompt() {
        Ok(_response_yes) if _response_yes => root
            .world
            .resource_scope(|world, scene_manager: Mut<SceneManager>| {
                scene_manager.save_scene(world)
            })
            .map_err(|err| "SceneError: ".to_owned() + &err.to_string()),
        Ok(_response_no) => Err("Aborted saving scene".to_owned()),
        Err(err) => Err(format!("Inquire error! Save aborted [{}]", err.to_string())),
    }
}

fn load_scene(root: &mut GameRoot) -> Result<(), String> {
    let path = match Text::new("Path of scene >").prompt() {
        Ok(ok) => PathBuf::from(ok),
        Err(err) => return Err(format!("Aborted loading scene [{}]", err.to_string())),
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
            |world, scene_manager: Mut<SceneManager>| -> Option<PathBuf> {
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
        })
        .map_err(|err| err.to_string())
}

fn change_target_scene(root: &mut GameRoot) -> Result<(), String> {
    let op = |err: InquireError| -> String { err.to_string() };

    root.world.resource_scope(
        |_world: &mut World, mut resource: Mut<SceneManager>| -> Result<(), String> {
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
        |_world: &mut World, res: Mut<SceneManager>| -> Result<(), String> {
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

// Entities

fn new_entity(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world: &mut World, res: Mut<SceneManager>| -> Result<(), String> {
            let name = inquire::Text::new("Name of the entity >")
                .prompt()
                .map_err(|err| err.to_string())?;

            let name = {
                if name == "" {
                    None
                } else {
                    Some(name)
                }
            };

            world
                .get::<Scene>(res.target_scene.ok_or("No target scene found!")?)
                .ok_or("No scene component found for the current target scene!")?;

            let entity = world.spawn_empty().id();

            engine::scene::add_entity_to_scene(world, res.target_scene.unwrap(), entity, name)
                .map_err(|err| err.to_string())
        },
    )
}

fn list_entities(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world: &mut World, res: Mut<SceneManager>| -> Result<(), String> {
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
                    None => {
                        println!("Unidentified entity");
                        continue;
                    }
                };

                println!("{}", scene_data.entity_name);
            }

            Ok(())
        },
    )
}

fn scoop_entities(root: &mut GameRoot) -> Result<(), String> {
    let mut entities_to_scoop: Vec<Entity> = Vec::new();
    let mut target_scene: Option<Entity> = None;

    root.world
        .resource_scope(
            |world: &mut World, res: Mut<SceneManager>| -> Result<(), SceneError> {
                let scene_id = world
                    .get::<Scene>(res.target_scene.ok_or(SceneError::NoTargetScene)?)
                    .ok_or(SceneError::NoSceneComponent)?
                    .scene_id;

                target_scene = res.target_scene;

                for mut entity_mut in world.iter_entities_mut() {
                    if let Some(mut data) = entity_mut.get_mut::<SceneData>() {
                        if let None = data.scene_id {
                            data.scene_id = Some(scene_id);
                            entities_to_scoop.push(entity_mut.id())
                        }
                    } else {
                        entities_to_scoop.push(entity_mut.id())
                    }
                }

                Ok(())
            },
        )
        .map_err(|err| err.to_string())?;

    for entity in entities_to_scoop {
        add_entity_to_scene(&mut root.world, target_scene.unwrap(), entity, None)
            .map_err(|err| format!("Failed to scoop entity [{}]", err.to_string()))?;
    }

    Ok(())
}

// Components

fn add_component(root: &mut GameRoot) -> Result<(), String> {
    root.world
        .resource_scope(
            |world: &mut World, res: Mut<SceneManager>| -> Result<(), SceneError> {
                let scene = world
                    .get::<Scene>(res.target_scene.ok_or(SceneError::NoTargetScene)?)
                    .ok_or(SceneError::NoSceneComponent)?;

                let entity_names = world
                    .get::<Scene>(res.target_scene.ok_or(SceneError::NoTargetScene)?)
                    .ok_or(SceneError::NoSceneComponent)?
                    .get_entities()
                    .iter()
                    .map(|entity| world.get::<SceneData>(*entity).unwrap().entity_name.clone())
                    .collect::<Vec<String>>();

                if entity_names.len() == 0 {
                    return Err(SceneError::NoEntitiesAvailable);
                }

                // Display the available components that can be serialized
                let types = res
                    .type_registry
                    .iter()
                    .filter(|i| i.data::<ReflectTestSuperTrait>().is_some());

                let type_names: Vec<&str> = types
                    .cloned()
                    .map(|reg| reg.type_info().type_path())
                    .collect();

                // Create a new component based on user input
                let component_path = Select::new("Component path >", type_names)
                    .prompt()
                    .unwrap();

                let component_registration: &bevy_reflect::TypeRegistration = res
                    .type_registry
                    .get_with_type_path(component_path)
                    .ok_or(SceneError::MissingTypeRegistry(component_path.to_owned()))?;

                let reflect_component = component_registration.data::<ReflectComponent>().ok_or(
                    SceneError::NoReflectData(format!(
                        "{} is missing a ReflectComponent type data (Add #[reflect(Component)])",
                        component_path
                    )),
                )?;

                // Figure out the entity to add the new component to.
                let entity = loop {
                    let name = inquire::Select::new(
                        "What entity do you want to add it to? >",
                        entity_names.clone(),
                    )
                    .prompt()
                    .unwrap();

                    let entity = scene
                        .get_entity(world, name)
                        .ok_or("The entity does not exist!");

                    if let Ok(entity) = entity {
                        break entity;
                    }
                };

                let mut entity = world.entity_mut(entity);

                let represented_type = Some(component_registration.type_info());

                let component_patch: ReflectOwned = match component_registration.type_info() {
                    TypeInfo::Struct(info) => {
                        let mut dynamic_struct = DynamicStruct::default();
                        dynamic_struct.set_represented_type(represented_type);
                        ReflectOwned::Struct(Box::new(dynamic_struct))
                    }
                    TypeInfo::TupleStruct(info) => {
                        let mut dynamic_struct = DynamicTupleStruct::default();
                        dynamic_struct.set_represented_type(represented_type);
                        ReflectOwned::TupleStruct(Box::new(dynamic_struct))
                    }
                    TypeInfo::Tuple(info) => {
                        new_dyn_tuple(info, component_registration, component_path)?
                    }
                    TypeInfo::List(info) => todo!(),
                    TypeInfo::Array(info) => todo!(),
                    TypeInfo::Map(info) => todo!(),
                    TypeInfo::Enum(info) => {
                        new_dyn_enum(info, component_registration, component_path)?
                    }
                    TypeInfo::Value(info) => {
                        new_dyn_value(info, component_registration, component_path)?
                    }
                };

                reflect_component.apply_or_insert(
                    &mut entity,
                    match &component_patch {
                        ReflectOwned::Struct(e) => e.as_reflect(),
                        ReflectOwned::TupleStruct(e) => e.as_reflect(),
                        ReflectOwned::Tuple(e) => e.as_reflect(),
                        ReflectOwned::List(e) => e.as_reflect(),
                        ReflectOwned::Array(e) => e.as_reflect(),
                        ReflectOwned::Map(e) => e.as_reflect(),
                        ReflectOwned::Enum(e) => e.as_reflect(),
                        ReflectOwned::Value(e) => e.as_reflect(),
                    },
                    &res.type_registry,
                );

                Ok(())
            },
        )
        .map_err(|err| err.to_string())
}

fn list_components(root: &mut GameRoot) -> Result<(), String> {
    root.world
        .resource_scope(|world, res: Mut<SceneManager>| -> Result<(), SceneError> {
            let scene = world
                .get::<Scene>(res.target_scene.ok_or(SceneError::NoTargetScene)?)
                .ok_or(SceneError::NoSceneComponent)?;
            let object_id = scene.scene_id;

            for (entity, components) in world.query::<(Entity, &dyn TestSuperTrait)>().iter(world) {
                match world.get::<SceneData>(entity) {
                    Some(scene_data) => {
                        if scene_data.scene_id.is_none() {
                            continue;
                        }
                        if !(scene_data.scene_id.unwrap() == object_id) {
                            continue;
                        }
                        println!("{}", scene_data.entity_name)
                    }
                    None => continue,
                }
                for component in &components {
                    let path = res
                        .type_registry
                        .get(component.as_reflect().type_id())
                        .unwrap()
                        .type_info()
                        .type_path();
                    println!(" - {}", path);
                }
            }

            Ok(())
        })
        .map_err(|err| err.to_string())
}

fn log(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource::<engine::Input>().log();
    root.world.resource::<scene::SceneManager>().log();
    root.world.resource::<engine::GgezInterface>().log();
    Ok(())
}
