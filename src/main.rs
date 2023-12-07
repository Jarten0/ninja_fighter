pub mod components;
pub mod space;
mod schedule;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::system::Query;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{Context, ContextBuilder, GameResult};

// use bevy_ecs::*;
use bevy_ecs::world::*;
use space::Position;
use space::Velocity;

fn main() {
    // Make a Context.
    loop {
        let (mut context, event_loop) = ContextBuilder::new("ninja_fighter", "Jarten :)")
        .build()
        .expect("aieee, could not create ggez context!");

        // Create an instance of your event handler.
        // Usually, you should provide it with the Context object to
        // use when setting your game up.
        let my_game = GameRoot::new(&mut context);

        // Run!
        event::run(context, event_loop, my_game);
}
}

#[derive(Default)]
struct Indexer {
    pub init: Vec<Box<Entity>>,
    pub update: Vec<Box<Entity>>,
    pub draw: Vec<Box<dyn DrawBas>>,
    queue: Vec<Box<Entity>>,
}

#[allow(unused_variables)]
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
            entity.draw_bas(game_info, ctx, canvas);
        }
    }

    fn add_to_queue(&mut self, entity: Box<Entity>) {
        self.queue.push(entity);
    }
}

struct GameRoot where Self: 'static {
    pub game_info: GameInfo,
    pub entities: Indexer,
    schedule: Schedule,
}

impl GameRoot {
    fn update_context(&mut self, ctx: &mut Context) {
        self.game_info.context_ptr = ctx;
    }
}

pub struct GameInfo {
    pub world: World,
    context_ptr: *mut Context,
    game_root_ptr: *const GameRoot,
}

impl GameInfo {
    pub fn get_context(&self) -> &Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => panic!("ContextPtr is null! ContextPtr should NEVER be null"),
                false => return self.context_ptr.as_ref().unwrap()
            }
        }
    }

    pub fn get_mut_context(&mut self) -> &mut Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => panic!("context_Ptr is null! context_ptr should NEVER be null"),
                false => return self.context_ptr.as_mut().unwrap(),
            }
        }
    }

    pub fn new_entity(&self, entity: Box<Entity>) {
        unsafe {
            self.game_root_ptr.read().entities.add_to_queue(entity);
        };
    }
}

impl GameRoot {
    pub fn new(context: &mut Context) -> GameRoot {

        let schedule_default_thing = Schedule::default();

        let schedule = schedule::schedule_systems(schedule_default_thing);

        // Load/create resources such as images here.
        let mut world = World::new();

        let entity = world
            .spawn((Position::new(50.0, 50.0), Velocity::new(2.0, 2.0)))
            .id();

        let mut entity_ref = world.entity_mut(entity.clone());

        entity_ref.get_mut::<Velocity>().unwrap().y += 3.0;

        let entities = Indexer::default();

        let mut root = GameRoot {
            game_info: GameInfo {
                world,
                game_root_ptr: std::ptr::null::<GameRoot>(),
                context_ptr: context
            },
            entities,
            schedule,
        };
        root.game_info.game_root_ptr = &root;
        root
    }
}

impl EventHandler for GameRoot {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        self.update_context(ctx);
    
        self.schedule.run(&mut self.game_info.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.game_info.context_ptr = ctx;
        
        self.entities
            ._draw_calls(&mut self.game_info, ctx, &mut canvas);

        canvas.finish(ctx)
    }
}


pub trait Init<System>
where
    System: bevy_ecs::query::WorldQuery,
{
    fn init(query: Query<System>);
}

pub trait Update<System>
where
    System: bevy_ecs::query::WorldQuery,
{
    fn update(query: Query<System>);
}

pub trait Draw<System>
where
    System: bevy_ecs::query::WorldQuery
{
    fn draw(query: Query<System>);
}

pub trait DrawBas {
    fn draw_bas(&mut self, game_info: &mut GameInfo, ctx: &mut Context, canvas: &mut Canvas);
}

// pub trait EntityBundle
// where
//     Self: Init,
//     Self: Update,
//     Self: Draw,
// {
// }
