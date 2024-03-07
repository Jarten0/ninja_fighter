use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use engine::input::key::keycode_converter::keycode_to_str;
use engine::scene::{Scene, SceneData, SceneError, SceneManager};
use engine::GameRoot;

use bevy_ecs::{
    reflect::ReflectComponent,
    world::{Mut, World},
};

use bevy_reflect::DynamicStruct;
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
            DebugCommand::new("addcomponent", add_component, "Adds a new component to the entity. Advanced feature, many prompts and can be quite confusing. ")
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
    match Confirm::new("Are you sure? This will overwrite any previous saved data").prompt() {
        Ok(_response_yes) if _response_yes => root
            .world
            .resource_scope(|world, mut scene_manager: Mut<SceneManager>| {
                scene_manager.save_scene(world)
            })
            .map_err(|err| err.into()),
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
    root.world.resource_scope(
        |world: &mut World, mut res: Mut<SceneManager>| -> Result<(), String> {
            let scene = world
                .get::<Scene>(res.target_scene.ok_or("No target scene found!")?)
                .ok_or("No scene component found for the current target scene!")?;

            for i in res.type_registry.iter() {
                println!("{}", i.type_info().type_path())
            }

            let component_path = Text::new("Component path > ").prompt().unwrap();

            let component_registration: &bevy_reflect::TypeRegistration = res
                .type_registry
                .get_with_type_path(&component_path)
                .ok_or(SceneError::MissingTypeRegistry(component_path.clone()).to_string())?;

            let field_names = match component_registration.type_info() {
                bevy_reflect::TypeInfo::Struct(struct_info) => struct_info.field_names(),
                bevy_reflect::TypeInfo::TupleStruct(_) => todo!(), // These `todo!()` 's shouldn't be hit, but if they are, implement something here.
                bevy_reflect::TypeInfo::Tuple(_) => todo!(),
                bevy_reflect::TypeInfo::List(_) => todo!(),
                bevy_reflect::TypeInfo::Array(_) => todo!(),
                bevy_reflect::TypeInfo::Map(_) => todo!(),
                bevy_reflect::TypeInfo::Enum(_) => todo!(),
                bevy_reflect::TypeInfo::Value(value) => {
                    return Err(
                        "expected a component type, found a non-component value type".to_owned(),
                    )
                }
            };

            let mut component_patch = DynamicStruct::default();

            component_patch.set_represented_type(Some(component_registration.type_info()));

            let mut component_data: HashMap<&str, serde_json::Value> = HashMap::new();

            for field_name in field_names {
                component_data.insert(
                    &field_name,
                    serde_json::from_str(
                        &Text::new(&format!("Enter value for field [{}] > ", field_name))
                            .prompt()
                            .unwrap(),
                    )
                    .map_err(|err| err.to_string())?,
                );
            }

            for (name, field) in &component_data {
                println!("!");
                let f = || {
                    panic!(
                        "No expected type found! tried to find the field named [{:?}] on {:?}. ",
                        name, component_path
                    )
                };

                let type_info = match component_registration.type_info() {
                    bevy_reflect::TypeInfo::Struct(struct_info) => struct_info,
                    bevy_reflect::TypeInfo::TupleStruct(_) => todo!(),
                    bevy_reflect::TypeInfo::Tuple(_) => todo!(),
                    bevy_reflect::TypeInfo::List(_) => todo!(),
                    bevy_reflect::TypeInfo::Array(_) => todo!(),
                    bevy_reflect::TypeInfo::Map(_) => todo!(),
                    bevy_reflect::TypeInfo::Enum(_) => todo!(),
                    bevy_reflect::TypeInfo::Value(_) => todo!(),
                };

                let expected_type = type_info
                    .field(name)
                    .ok_or(SceneError::MissingTypeRegistry(component_path.clone()).to_string())?
                    .type_path();

                let value = match field.to_reflect(Some(expected_type)) {
                    Ok(ok) => ok,
                    Err(ok) => ok,
                };

                component_patch.insert_boxed(&name, value);
            }
            let reflect_component = component_registration.data::<ReflectComponent>().unwrap();

            let entity = loop {
                let name = Text::new("What entity do you want to add it to? > ")
                    .prompt()
                    .unwrap();

                if name == "exit" {
                    return Err("Aborted before entity could be instantiated".to_owned());
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

            reflect_component.apply_or_insert(&mut entity, &component_patch);

            Ok(())
        },
    )
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
