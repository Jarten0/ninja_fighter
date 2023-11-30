pub mod protag;
pub mod render;
pub mod transform;

use bevy_ecs::entity::Entity;
use bevy_ecs::schedule::{IntoSystemConfigs, Schedule};
use bevy_ecs::system::Query;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{Context, ContextBuilder, GameResult};

// use bevy_ecs::*;
use bevy_ecs::world::*;
use transform::{Position, Transform, Velocity};

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
    pub init: Vec<Box<Entity>>,
    pub update: Vec<Box<Entity>>,
    pub draw: Vec<Box<Entity>>,
    queue: Vec<Box<Entity>>,
}

impl Indexer {
    fn _init_calls(&mut self, game_info: &mut GameInfo, ctx: &mut Context) {
        for index in 0..self.queue.len() {
            match self.queue.pop() {
                None => (),
                Some(entity) => self.init.push(entity),
            };
        }

        for entity in &mut self.init {
            // entity.init(game_info, ctx);
        }
        self.init.clear();
    }

    fn _update_calls(&mut self, game_info: &mut GameInfo, ctx: &mut Context) {
        for entity in &mut self.update {
            // entity.update(game_info, ctx);
        }
    }

    fn _draw_calls(&mut self, game_info: &mut GameInfo, ctx: &mut Context, canvas: &mut Canvas) {
        for entity in &mut self.draw {
            // entity.draw(game_info, ctx, canvas);
        }
    }

    fn add_to_queue(&mut self, entity: Box<Entity>) {
        self.queue.push(entity);
    }
}

struct GameRoot {
    pub game_info: GameInfo,
    pub entities: Indexer,
}

pub struct GameInfo {
    pub world: World,
    game_root: *const GameRoot,
}

impl GameInfo {
    pub fn new_entity(&self, entity: Box<Entity>) {
        unsafe {
            self.game_root.read().entities.add_to_queue(entity);
        };
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
                game_root: std::ptr::null::<GameRoot>(),
            },
            entities,
        };
        root.game_info.game_root = &root;
        root
    }
}

impl EventHandler for GameRoot {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mut schedule = Schedule::default();

        schedule.add_systems(Position::update);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.entities
            ._draw_calls(&mut self.game_info, ctx, &mut canvas);

        canvas.finish(ctx)
    }
}

pub trait SystemsManager
where
    Self: Sized,
{
    fn schedule_systems<Marker>() -> dyn IntoSystemConfigs<Marker>;
}

pub trait Init<System>
where
    System: bevy_ecs::query::WorldQuery,
{
    fn init(mut query: Query<System>) {}
}

pub trait Update<System>
where
    System: bevy_ecs::query::WorldQuery,
{
    fn update(mut query: Query<System>) {}
}

pub trait Draw {
    fn draw(&mut self, root: &mut GameInfo, ctx: &mut Context, canvas: &mut graphics::Canvas);
}

// pub trait EntityBundle
// where
//     Self: Init,
//     Self: Update,
//     Self: Draw,
// {
// }
