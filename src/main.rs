//! The main directory.
//!
//! # Modules
//!
//! * [`components`] - where game logic occurs and where most end point functionality is written.
//!
//! * [`engine`] - modules designed to interface between the different libraries the core engine uses, including [`bevy_ecs`] and [`ggez`].
//!
//! * [`space`] -

mod components;
mod engine;

/// The start of the program. The crux of the functionality however happens in [`engine::GameRoot`].
fn main() -> ! {
    let args: Vec<String> = std::env::args().collect();

    for arg in args {
        if arg == String::from("input") {
            engine::input_cli_editor();
        }
    }

    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter", "Jarten0")
        .build()
        .expect("aieee, could not create ggez context!");

    let root = engine::GameRoot::new(&mut context);

    ggez::event::run(context, event_loop, root);
}
