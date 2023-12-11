pub mod components;
pub mod readonly;
mod schedule;
pub mod space;

use bevy_ecs::entity::Entity;
use bevy_ecs::schedule::{Schedule, ScheduleLabel};
use bevy_ecs::system::Query;
use components::{Protag, ProtagBundle, Transform};
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

struct GameRoot
where
    Self: 'static,
{
    pub game_info: GameInfo,
    schedule: Schedule,
    draw_schedule: Schedule,
}

impl GameRoot {
    fn update_context(&mut self, ctx: &mut Context) {
        self.game_info.context_ptr = ctx;
    }
}

#[derive(Debug)]
pub struct GameInfo {
    pub world: World,
    context_ptr: *mut Context,
    game_root_ptr: *mut GameRoot,
    current_canvas: Option<Canvas>,
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
}

impl GameRoot {
    pub fn new(context: &mut Context) -> GameRoot {
        let (schedule, draw_schedule) =
            schedule::schedule_systems(Schedule::default(), Schedule::default());

        let mut world = World::new();

        let mut entity = world.spawn(ProtagBundle::default(&context.gfx));

        let mut root = GameRoot {
            game_info: GameInfo {
                world,
                game_root_ptr: std::ptr::null_mut::<GameRoot>(),
                context_ptr: context,
                current_canvas: Canvas::from_frame(&context.gfx, Color::WHITE),
            },
            schedule,
            draw_schedule,
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
        self.game_info.current_canvas = Some(graphics::Canvas::from_frame(ctx, Color::WHITE));

        self.update_context(ctx);

        self.draw_schedule.run(&mut self.game_info.world);

        let e = (self.game_info.current_canvas = None)

        // self.game_info.current_canvas.finish(ctx)
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
