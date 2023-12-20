mod components;
mod engine;
mod freeze;
mod game_root;
mod schedule;

pub mod space;

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

    let root = game_root::GameRoot::new(&mut context);

    ggez::event::run(context, event_loop, root);
}
