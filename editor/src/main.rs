//! Creates an application for creating game data that will be serialized into the game binary itself
//! Can also store data outside of the binary as specified

use bevy_ecs::world::{Mut, World};
use editor::input_debugger::InputDebugger;
use editor::inspector::EditorGUI;
use engine::EngineConfig;
use ggez::conf::WindowMode;

pub static EDITOR_ENGINE_CONFIG: EngineConfig = EngineConfig {
    external_scene_paths: &[game::INITIAL_SCENE],
    scenes_folder: Some(game::SCENE_FOLDER),
    world_init: init_editor_schedules,
    ticks_per_second: game::ENGINE_CONFIG.ticks_per_second,
    freeze_on_unfocus: false,
    freeze_on_minimize: false,
    run_debug_schedules: true,
};

// Add new resources here!
pub fn init_editor_schedules(world: &mut World) {
    // The editor essentially acts as a wrapper for the game itself, injecting it's own code into the game.
    // Thus, we run all of the usual functions for the game, then we run our own stuff on top of it.
    game::init_components_and_resources(world);

    world.insert_resource(InputDebugger::default());

    let editor_interface = world.resource_scope(
        |world: &mut World, mut engine: Mut<engine::GgezInterface>| {
            EditorGUI::new(engine.get_context_mut(), world)
        },
    );
    world.insert_resource(editor_interface);

    world.add_schedule(editor::debug_tick_schedule());
    world.add_schedule(editor::debug_frame_schedule());
    world.add_schedule(editor::debug_init_schedule());
    world.add_schedule(editor::debug_gui_schedule());

    log::trace!("Created editor resources and initialized editor schedules");
}

fn main() {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter Editor", "Jarten0")
        .window_mode(WindowMode {
            width: 800.0,
            height: 600.0,
            maximized: true,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            transparent: false,
            min_width: 1.0,
            min_height: 1.0,
            max_width: 1920.0,
            max_height: 1080.0,
            resizable: true,
            visible: true,
            resize_on_scale_factor_change: false,
            logical_size: None,
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let mut root = engine::GameRoot::new(&mut context, &EDITOR_ENGINE_CONFIG)
        .expect("could not build game root");

    editor::console::hook_emergency_panic_handler(&mut root);

    ggez::event::run(context, event_loop, root);
}
