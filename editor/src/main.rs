//! Creates an application for creating game data that will be serialized into the game binary itself
//! Can also store data outside of the binary as specified

use ggez::conf::WindowMode;

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

    let root = engine::GameRoot::new(&mut context, &editor::EDITOR_ENGINE_CONFIG)
        .expect("could not build game root");

    ggez::event::run(context, event_loop, root);
}
