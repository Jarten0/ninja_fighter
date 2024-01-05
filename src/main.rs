//! The main directory.
//!
//! # Modules
//!
//! * [`components`] - where game logic occurs and where most end point functionality is written.
//!
//! * [`engine`] - modules designed to interface between the different libraries the core engine uses, including [`bevy_ecs`] and [`ggez`].
//! Also contains extra stuff to standardize things used between all projects that use this engine.

mod components;
mod engine;

/// The start of the program. The crux of the functionality however happens in [`engine::GameRoot`].
fn main() -> ! {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter", "Jarten0")
        .build()
        .expect("aieee, could not create ggez context!");

    let root = engine::GameRoot::new(&mut context);

    ggez::event::run(context, event_loop, root);
}
