pub mod protag;
pub mod render;
pub mod transform;

use bevy_ecs::entity::Entity;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{Context, ContextBuilder, GameResult};

// use bevy_ecs::*;
use bevy_ecs::world::*;
use transform::{Position, Velocity};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("ninja_fighter", "Jarten :)")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GameRoot::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

#[derive(Default)]
struct Indexer {
    pub init: Vec<Box<dyn Init>>,
    pub update: Vec<Box<dyn Update>>,
    pub draw: Vec<Box<dyn Draw>>,
}

impl Indexer {
    fn _init_calls(&mut self, game_info: &mut GameInfo, ctx: &mut Context) {
        for entity in &mut self.init {
            entity.init(game_info, ctx);
        }
        self.init.clear();
    }

    fn _update_calls(&mut self, game_info: &mut GameInfo, ctx: &mut Context) {
        for entity in &mut self.update {
            entity.update(game_info, ctx);
        }
    }

    fn _draw_calls(&mut self, game_info: &mut GameInfo, ctx: &mut Context, canvas: &mut Canvas) {
        for entity in &mut self.draw {
            entity.draw(game_info, ctx, canvas);
        }
    }
}

struct GameRoot {
    pub game_info: GameInfo,
    entities: Indexer,
}

pub struct GameInfo {
    pub world: World,
    game_root: Option<&'static mut GameRoot>,
}

impl GameInfo {
    fn new_entity(&mut self) {
        match self.game_root {
            None => todo!(),
            Some(..) => todo!(),
        }
    }
}

impl GameRoot {
    pub fn new(_ctx: &mut Context) -> GameRoot {
        // Load/create resources such as images here.
        let mut world = World::new();

        let entity = world
            .spawn((Position { x: 50.0, y: 50.0 }, Velocity { x: 2.0, y: 2.0 }))
            .id();

        let mut entity_ref = world.entity_mut(entity.clone());

        entity_ref.get_mut::<Velocity>().unwrap().y += 3.0;

        let entities = Indexer::default();

        let mut root = GameRoot {
            game_info: GameInfo {
                world,
                game_root: None,
            },
            entities,
        };
        // root.game_info.game_root = Some(&mut root); // !TODO
        root
    }
}

impl EventHandler for GameRoot {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.entities._update_calls(&mut self.game_info, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.entities
            ._draw_calls(&mut self.game_info, ctx, &mut canvas);

        canvas.finish(ctx)
    }
}

pub trait Init {
    fn init(&mut self, root: &mut GameInfo, ctx: &mut Context);
}

pub trait Update {
    fn update(&mut self, root: &mut GameInfo, ctx: &mut Context);
}

pub trait Draw {
    fn draw(&mut self, root: &mut GameInfo, ctx: &mut Context, canvas: &mut graphics::Canvas);
}
