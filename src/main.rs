pub mod components;
pub mod readonly;
mod schedule;
pub mod space;

use std::thread::park_timeout;
use std::time::Duration;

use bevy_ecs::schedule::Schedule;
use components::ProtagBundle;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{Context, ContextBuilder, GameResult};

use bevy_ecs::world::*;

/// A basic container-struct designed for holding information and sharing access through [`bevy_ecs`]'s component system.
/// Use the [`components::context::WorldInfo`] component in a query to access.
///
/// # Fields
///
/// * `world` - owned public means of accessing the [`World`] [`bevy_ecs`] provides.
///
/// * `context_ptr` - private raw pointer pointing to the current [`Context`] for the given schedule.
///
/// * `game_root_ptr` - private raw pointer pointing to the [`GameRoot`] which owns this struct as well as the system [`Schedule`]'s
///
/// * `current_canvas` - private optional holding the current [`Canvas`].
/// Holds [`None`] if operating during an `Update` frame, or holds `Some(Canvas)` if operating during a `Draw` frame.

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
    /// Returns a reference to the value that `self.context_ptr` points to.
    /// Panics if `self.context_ptr` is null or invalid, which should never be the case in normal scenarios. If it is, investigate immediately.
    pub fn get_context(&self) -> &Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => {
                    panic!("`game_info.context_ptr` is null! `context_ptr` should never be null!")
                }
                false => {
                    return self.context_ptr.as_ref().expect(
                        "`game_info.context_ptr` is invalid! Something fundamental has gone wrong!",
                    )
                }
            }
        }
    }

    /// Returns a mutable reference to the value that `self.context_ptr` points to.
    /// Panics if `self.context_ptr` is null or invalid, which should never be the case in normal scenarios. If it is, investigate immediately.
    pub fn get_mut_context(&mut self) -> &mut Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => {
                    panic!("`game_info.context_ptr` is null! `context_ptr` should never be null!")
                }
                false => {
                    return self.context_ptr.as_mut().expect(
                        "`game_info.context_ptr` is invalid! Something fundamental has gone wrong!",
                    )
                }
            }
        }
    }
}

/// A basic container-struct that handles [`ggez`]'s events and interfaces with [`bevy_ecs`]'s ECS to provide full engine functionality.
/// Use the [`components::context::WorldInfo`] component in a query, then use `WorldInfo.game_info.` to access.
///
/// # Fields
///
/// * `game_info` - public owned singleton (unenforced) struct containing global information seperated from interaction with engine level code
///
/// * `schedule` - private owned [`Schedule`] used in operating [`bevy_ecs`]. Is run during `Update` frames.
///
/// * `draw_schedule` - private owned [`Schedule`] used in operating [`bevy_ecs`]. Is run during `Draw` frames.
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

impl GameRoot {
    /// Loads and initialized essential data for [`bevy_ecs`] operations, specifically the [`GameRoot`] and [`GameInfo`] structs
    fn new(context: &mut Context) -> GameRoot {
        let (schedule, draw_schedule) =
            schedule::schedule_systems(Schedule::default(), Schedule::default());

        // park_timeout(Duration::from_secs(15));

        let world = World::new();

        let game_info = GameInfo {
            world,
            game_root_ptr: std::ptr::null_mut::<GameRoot>(),
            context_ptr: context,
            current_canvas: None,
        };

        let mut root = GameRoot {
            game_info,
            schedule,
            draw_schedule,
        };

        root.update_context(context);
        root.game_info.game_root_ptr = &mut root;

        let bundle = ProtagBundle::default(&mut root.game_info);
        let _protag = root.game_info.world.spawn(bundle);

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

        self.game_info
            .current_canvas
            .take()
            .expect("game_info.current_canvas should never be moved during system running! If you took it, please undo that and make a clone or borrow instead of take ownership over.")
            .finish(&mut ctx.gfx)
    }
}

fn main() {
    loop {
        let (mut context, event_loop) = ContextBuilder::new("Ninja Fighter", "Jarten0")
            .build()
            .expect("aieee, could not create ggez context!");

        let my_game = GameRoot::new(&mut context);

        event::run(context, event_loop, my_game);
    }
}
