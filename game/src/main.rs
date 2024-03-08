// // ! The main binary.
// // !
// // ! To create a game, first add the [`engine`] library as a dependency.
// // ! Then, add whichever component libraries you want to use.
// // !
// // ! After that, check out the [`editor`] for ways to develop a game.
// // !
// // ! * [`engine`] - modules designed to interface between the different libraries the core engine uses, including [`bevy_ecs`] and [`ggez`].
// // ! Also contains extra stuff to standardize things used between all projects that use this engine, for example, [`engine::space`] for a standard vector system.

// / The start of the program. The crux of the functionality however happens in [`engine::GameRoot`].
fn main() -> ! {
    let (mut context, event_loop) = ggez::ContextBuilder::new("Ninja Fighter", "Jarten0")
        .build()
        .expect("aieee, could not create ggez context!");

    let root = engine::GameRoot::new(
        &mut context,
        game::init_components_and_resources,
        game::schedule_builders,
        game::TICKS_PER_SECOND.clone(),
        None,
    );

    ggez::event::run(context, event_loop, root);
}
