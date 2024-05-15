//! Contains all of the functionality related to getting [`bevy_ecs`] and [`ggez`] up and running.
//!
//! * [`GameRoot`] - Creates the main game state, initializes modules and libraries, and communicates between [`ggez`]'s engine and [`bevy_ecs`]'s world

// Hi! If your reading this, welcome to my fun little project. Some shenanigans are afoot!

use crate::input::KeycodeType;
use crate::logging;
use crate::scene::SceneError;
use crate::scene::SceneManager;
use crate::schedule::ScheduleTag;
use crate::space::Vector2;
use crate::Camera;
use crate::EngineConfig;
use crate::EngineConfigError;
use crate::GgezInterface;
use crate::Input;
use crate::SomeError;
use bevy_ecs::world::*;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use log::debug;
use log::*;
use std::path::PathBuf;
use std::time::Duration;

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
pub struct GameRoot
where
    Self: 'static,
{
    pub world: World,
    pub ticks_per_second: u32,
}

impl GameRoot {
    /// Simply changes the context value in the GameInfo resource to the input
    fn update_context(&mut self, ctx: &mut Context) {
        self.engine_mut().context_ptr = ctx;
    }

    fn engine(&mut self) -> &GgezInterface {
        self.world.resource::<GgezInterface>()
    }

    fn engine_mut(&mut self) -> Mut<'_, GgezInterface> {
        self.world.resource_mut::<GgezInterface>()
    }

    pub fn context(&self) -> &Context {
        self.world.resource::<GgezInterface>().get_context()
    }

    // no mut context shortcut since resource_mut returns a temporary value, instead of a borrowed one.
    // pub fn context_mut(&mut self) -> &mut Context {
    //     self.world.resource_mut::<GgezInterface>().get_context_mut()
    // }

    /// Loads and initialized essential data, and calls the [`ScheduleTag::Init`] systems
    ///
    /// To pass in a `config`, create a static [`EngineConfig`] and pass in a reference
    pub fn new(context: &mut Context, config: &'static EngineConfig) -> Result<Self, String> {
        let mut world = World::new();

        if let Err(err) = log::set_logger(&logging::LOGGER) {
            eprintln!("Failed to create logger! [{}]", err.to_string())
        }

        log::set_max_level(LevelFilter::Trace);

        info!("Begin log");

        // crate::schedule::add_schedules(&mut world, (config.schedule_builder_functions)());

        let game_info = GgezInterface::new(context, config.clone());
        World::insert_resource(&mut world, game_info);

        let input = Input::load();
        World::insert_resource(&mut world, input);

        let scene_manager = SceneManager::default();
        World::insert_resource(&mut world, scene_manager);

        let camera = Camera::default();
        World::insert_resource(&mut world, camera);

        trace!("Created main resources");

        crate::register_scene_types(&mut world);

        let mut root = GameRoot {
            world,
            ticks_per_second: config.ticks_per_second,
        };
        GameRoot::update_context(&mut root, context);

        (config.world_init)(&mut root.world);

        trace!("Initialized world and created game root");

        root.world
            .resource_scope(
                |world, mut res: Mut<SceneManager>| -> Result<(), SomeError> {
                    let path_str = config
                        .scene_paths
                        .get(0)
                        .ok_or(SomeError::EngineConfig(EngineConfigError::NoScenePaths))?;

                    let path: PathBuf = path_str.into();

                    if let Err(err) = res
                        .load_scene(world, path)
                        .map_err(|err| SomeError::Scene(err))
                    {
                        // a file not being found is okay, just cancel the load and print an error message.
                        #[cfg(feature = "editor_features")]
                        {
                            return Ok(());
                        }

                        #[cfg(not(feature = "editor_features"))]
                        {
                            return Err(err);
                        }
                    };

                    Ok(())
                },
            )
            .map_err(|err| err.to_string())?;

        trace!("Loaded default scene from EngineConfig");

        root.world.run_schedule(ScheduleTag::Init);

        if config.run_debug_schedules {
            root.world.run_schedule(ScheduleTag::DebugInit);
        }

        trace!("Ran init schedule!");

        Ok(root)
    }
}

impl EventHandler for GameRoot {
    /// Passes guard clauses depending on the TPS, checks for debug logic, updates resources then runs [`ScheduleTag::Tick`]
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        #[cfg(feature = "editor_features")]
        if self.engine().debug_mode {
            self.world.run_schedule(ScheduleTag::DebugGUI)
        }

        // FPS limiter: read `check_update_time` docs for more details
        if !ctx.time.check_update_time(self.ticks_per_second) {
            return GameResult::Ok(());
        }
        let remaining_update_time = ctx.time.remaining_update_time();
        if remaining_update_time > Duration::from_millis(80) {
            debug!("Lag spike of 5+ frames [{:?}]", remaining_update_time);

            if remaining_update_time > Duration::from_millis(10000) {
                log::debug!(
                    "Remaining update time is over 10,000 milliseconds! {}",
                    remaining_update_time.as_millis()
                );
            }
            while ctx.time.remaining_update_time() > Duration::from_millis(16) {
                ctx.time.check_update_time(self.ticks_per_second);
            }
        }

        self.update_context(ctx);

        self.world.resource_mut::<Input>().process_key_queue();

        #[cfg(feature = "editor_features")]
        if let Some(action) = self.world.resource::<Input>().get_action("enabledebugmode") {
            if action.status().is_just_pressed() {
                let mut engine = self.engine_mut();
                engine.debug_mode = !engine.debug_mode;
                debug!("Pressed debug button {}", engine.debug_mode);
            }
        }

        if self.engine().is_freeze_frame() {
            self.world.run_schedule(ScheduleTag::FreezeTick)
        } else {
            self.world.run_schedule(ScheduleTag::Tick)
        }

        #[cfg(feature = "editor_features")]
        if self.engine().debug_mode {
            self.world.run_schedule(ScheduleTag::DebugTick)
        }

        Ok(())
    }

    /// Creates a new [`Canvas`](graphics::Canvas) and calls the [`ScheduleTag::Frame`] schedule as often as possible
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.engine_mut().set_canvas(graphics::Canvas::from_frame(
            ctx,
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        ));

        self.update_context(ctx);

        self.world.run_schedule(ScheduleTag::Frame);

        #[cfg(feature = "editor_features")]
        if self.engine().debug_mode {
            // self.world.run_schedule(ScheduleTag::Gizmos);

            self.world.run_schedule(ScheduleTag::DebugFrame);
        }

        self.engine_mut()
        .take_canvas()
        .expect("game_info.current_canvas should never be moved during system running! If you took it, please undo that and make a clone or borrow instead of taking ownership over it.")
        .finish(&mut ctx.gfx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        // trace!("Key pressed");

        self.world
            .resource_mut::<Input>()
            .update_key_queue(KeycodeType::Mouse(button), true);

        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        self.world
            .resource_mut::<Input>()
            .update_key_queue(KeycodeType::Mouse(button), false);

        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        self.world
            .resource_mut::<Input>()
            .update_mouse_pos(Vector2::new(_x, _y));
        Ok(())
    }

    fn mouse_enter_or_leave(
        &mut self,
        _ctx: &mut Context,
        _entered: bool,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn mouse_wheel_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        self.world.resource_mut::<Input>().register_scroll(x, y);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if repeated {
            return Ok(());
        }

        if input.keycode == Some(ggez::winit::event::VirtualKeyCode::Escape) {
            ctx.request_quit();
        };

        let virtual_key_code = match input.keycode {
            Some(keycode) => keycode,
            None => {
                error!("Invalid keycode entered! Debug info: [{:#?}]", input);
                return Ok(());
            }
        };

        self.world
            .resource_mut::<Input>()
            .update_key_queue(KeycodeType::Keyboard(virtual_key_code), true);

        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        let virtual_key_code = match input.keycode {
            Some(keycode) => keycode,
            None => {
                error!("Invalid keycode entered! Debug info: [{:#?}]", input);
                return Ok(());
            }
        };

        self.world
            .resource_mut::<Input>()
            .update_key_queue(KeycodeType::Keyboard(virtual_key_code), false);

        Ok(())
    }

    fn text_input_event(
        &mut self,
        _ctx: &mut Context,
        character: char,
    ) -> Result<(), ggez::GameError> {
        self.world
            .resource_mut::<Input>()
            .register_text_input(character);

        Ok(())
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        debug!("quit_event() callback called, quitting...");
        self.world.resource_scope::<SceneManager, ()>(|world, res| {
            res.save_scene(world);
        });
        Ok(false)
    }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        _width: f32,
        _height: f32,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn on_error(
        &mut self,
        _ctx: &mut Context,
        _origin: event::ErrorOrigin,
        _e: ggez::GameError,
    ) -> bool {
        eprintln!(r#"Implement error handler :\ {}"#, _e);
        true
    }
}
