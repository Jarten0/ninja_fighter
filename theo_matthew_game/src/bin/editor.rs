static ENGINE_CONFIG: engine::EngineConfig = engine::EngineConfig {
    external_scene_paths: &[""],
    scenes_folder: Some("theo_matthew_game/assets/scenes"),
    world_init: theo_matthew_game::init_world,
    ticks_per_second: 60,
    freeze_on_unfocus: false,
    freeze_on_minimize: true,
    run_debug_schedules: true,
};

fn main() {
    let (mut context, event_loop) =
        ggez::ContextBuilder::new("theo_matthew_game", "Jarten0 + Thermulus")
            .window_setup(ggez::conf::WindowSetup {
                title: String::from("Theo Matthew Game"),
                samples: ggez::conf::NumSamples::One,
                vsync: false,
                icon: String::new(),
                srgb: true,
            })
            .window_mode(ggez::conf::WindowMode::default().maximized(true))
            .build()
            .expect("could not build context?!");

    let mut game_root = engine::GameRoot::new(&mut context, &ENGINE_CONFIG)
        .expect("expected no errors on game root initialization");

    editor::console::hook_emergency_panic_handler(&mut game_root);

    ggez::event::run(context, event_loop, game_root);
}
