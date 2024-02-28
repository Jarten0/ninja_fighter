use std::{collections::HashMap, path::PathBuf};

use crate::{
    input::key::keycode_converter::keycode_to_str,
    scene::{Scene, SceneData, SceneManager},
    GameRoot,
};

use bevy_ecs::world::{Mut, World};
use clap::builder::Str;
use inquire::{validator::Validation, Confirm, CustomType, InquireError, Select, Text};

const HOME_HELP_TEXT: &str = "
Welcome to my humble abode

No help is currently available, you'll have to pour through the code yourself

Sorry :\
";

const ACTION_EDITOR_HELP_TEXT: &str = "
This is where you can edit the keybinds of the current action.

* `add` - 
* `exit` - exits the debug CLI, returning to the main loop.
";

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

    let commands: Vec<(&str, fn(&mut GameRoot) -> Result<(), String>)> = vec![
        ("help", help),
        ("newscene", new_scene),
        ("savescene", save_scene),
        ("loadscene", load_scene),
        ("changescene", change_target_scene),
        ("listscenes", list_scenes),
        ("listentities", list),
    ];

    for (input, func) in commands {
        action_to_function.insert(input, func);
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

/// Lists the current entities in the target scene
fn list(root: &mut GameRoot) -> Result<(), String> {
    root.world.resource_scope(
        |world: &mut World, mut res: Mut<SceneManager>| -> Result<(), String> {
            let scene = world
                .get::<Scene>(res.target_scene.ok_or("No target scene found!")?)
                .ok_or("No scene component found for the current target scene!")?;

            if scene.entities.len() == 0 {
                println!("No entities found!");
                return Ok(());
            }

            for entity in scene.entities.clone() {
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
