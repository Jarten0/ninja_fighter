//! Creates an application for creating game data that will be serialized into the game binary itself
//! Can also store data outside of the binary as specified

use std::{thread::sleep, time::Duration};

fn main() {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter Editor", "Jarten0")
        .build()
        .expect("aieee, could not create ggez context!");

    let root = engine::GameRoot::new(
        &mut context,
        game::init_components_and_resources,
        editor::debuge::wrap_schedules_with_debug,
        60,
    );

    ggez::event::run(context, event_loop, root);
}
