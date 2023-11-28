pub mod transform;

use bevy_ecs::entity::Entity;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
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
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

pub struct MyGame {
    pub world: World,
    pub trackEntity: Entity,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut world = World::new();

        let entity = world
            .spawn((Position { x: 50.0, y: 50.0 }, Velocity { x: 2.0, y: 2.0 }))
            .id();

        let mut entity_ref = world.entity_mut(entity.clone());

        entity_ref.get_mut::<Velocity>().unwrap().y += 3.0;

        MyGame {
            world,
            trackEntity: entity,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...

        let mut ent = self.world.entity_mut(self.trackEntity.clone());

        println!("{}", ent.get_mut::<Position>().unwrap().y);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Draw code here...
        canvas.finish(ctx)
    }
}
