// // ! The main binary.
// // !
// // ! To create a game, first add the [`engine`] library as a dependency.
// // ! Then, add whichever component libraries you want to use.
// // !
// // ! After that, check out the [`editor`] for ways to develop a game.
// // !
// // ! * [`engine`] - modules designed to interface between the different libraries the core engine uses, including [`bevy_ecs`] and [`ggez`].
// // ! Also contains extra stuff to standardize things used between all projects that use this engine, for example, [`engine::space`] for a standard vector system.

use ggez::conf::WindowMode;

// / The start of the program. The crux of the functionality however happens in [`engine::GameRoot`].
fn main() -> ! {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter", "Jarten0")
        .window_mode(WindowMode {
            width: 800.0,
            height: 600.0,
            maximized: true,
            fullscreen_type: ggez::conf::FullscreenType::Desktop,
            borderless: false,
            transparent: false,
            min_width: 800.0,
            min_height: 600.0,
            max_width: 1920.0,
            max_height: 1080.0,
            resizable: true,
            visible: true,
            resize_on_scale_factor_change: false,
            logical_size: None,
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let root = engine::GameRoot::new(&mut context, &game::ENGINE_CONFIG);

    ggez::event::run(context, event_loop, root);
}
