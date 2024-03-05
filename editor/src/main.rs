//! Creates an application for creating game data that will be serialized into the game binary itself
//! Can also store data outside of the binary as specified

fn main() {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter Editor", "Jarten0")
        .build()
        .expect("aieee, could not create ggez context!");

    let root = engine::GameRoot::new(
        &mut context,
        editor::debuge::init_editor_schedules,
        editor::debuge::wrap_schedules_with_debug,
    );

    ggez::event::run(context, event_loop, root);
}
