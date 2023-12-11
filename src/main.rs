pub mod components;
pub mod readonly;
mod schedule;
pub mod space;

use bevy_ecs::entity::Entity;
use bevy_ecs::schedule::Schedule;
use bevy_ecs::system::Query;
use components::Protag;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{Context, ContextBuilder, GameResult};

// use bevy_ecs::*;
use bevy_ecs::world::*;
use space::Velocity;

fn main() {
    // Make a Context.
    loop {
        let (mut context, event_loop) = ContextBuilder::new("Ninja Fighter", "Jarten0")
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

#[allow(dead_code)]
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

struct GameRoot
where
    Self: 'static,
{
    pub game_info: GameInfo,
    entities: Indexer,
    schedule: Schedule,
}

impl GameRoot {
    fn update_context(&mut self, ctx: &mut Context) {
        self.game_info.context_ptr = ctx;
    }
}

#[derive()]
pub struct GameInfo {
    pub world: World,
    context_ptr: *mut Context,
    game_root_ptr: *mut GameRoot,
}

unsafe impl Send for GameInfo {}
unsafe impl Sync for GameInfo {}

impl GameInfo {
    pub fn get_context(&self) -> &Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => panic!("ContextPtr is null! ContextPtr should NEVER be null"),
                false => return self.context_ptr.as_ref().unwrap(),
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

        let mut entity = world.spawn(Protag::default());

        entity.get_mut::<Velocity>().unwrap().y += 3.0;

        let mut entities = Indexer::default();

        entities.add_to_queue(Box::new(entity.id()));

        let mut root = GameRoot {
            game_info: GameInfo {
                world,
                game_root_ptr: std::ptr::null_mut::<GameRoot>(),
                context_ptr: context,
            },
            entities,
            schedule,
        };
        root.game_info.game_root_ptr = &mut root;
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
        self.update_context(ctx);

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
    System: bevy_ecs::query::WorldQuery,
{
    fn draw(query: Query<System>);
}

pub trait DrawBas {
    fn draw_bas(&mut self, game_info: &mut GameInfo, ctx: &mut Context, canvas: &mut Canvas);
}
