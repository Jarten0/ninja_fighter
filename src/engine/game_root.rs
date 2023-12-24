use crate::engine::Input;
use bevy_ecs::schedule::Schedule;

use crate::components::ProtagBundle;
use crate::engine::MainCanvas;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

use bevy_ecs::world::*;
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
pub(crate) struct GameRoot
where
    Self: 'static,
{
    // pub game_info: GameInfo,
    schedule: Schedule,
    draw_schedule: Schedule,
    world: World,
}

impl GameRoot {
    /// Simply changes the context value in the GameInfo resource to the input
    fn update_context(&mut self, ctx: &mut Context) {
        self.get_game_info().context_ptr = ctx;
    }

    fn get_game_info(&mut self) -> Mut<'_, MainCanvas> {
        self.world.resource_mut::<MainCanvas>()
    }

    /// Loads and initialized essential data for [`bevy_ecs`] operations, specifically the [`GameRoot`] and [`MainCanvas`] structs
    pub(crate) fn new(context: &mut Context) -> Self {
        let (schedule, draw_schedule) =
            crate::engine::schedule::schedule_systems(Schedule::default(), Schedule::default());

        let mut world = World::new();

        let game_info = MainCanvas::new(context);
        World::insert_resource(&mut world, game_info);

        let input = Input::load();
        World::insert_resource(&mut world, input);

        let mut root = GameRoot {
            schedule,
            draw_schedule,
            world,
        };
        GameRoot::update_context(&mut root, context);

        let bundle = ProtagBundle::default(&mut root.get_game_info());
        World::spawn(&mut root.world, bundle);

        root
    }
}

impl EventHandler for GameRoot {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.update_context(ctx);

        self.schedule.run(&mut self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.get_game_info()
            .set_canvas(graphics::Canvas::from_frame(ctx, Color::WHITE));

        self.update_context(ctx);

        self.draw_schedule.run(&mut self.world);

        self.get_game_info()
        .take_canvas()
        .expect("game_info.current_canvas should never be moved during system running! If you took it, please undo that and make a clone or borrow instead of taking ownership over it.")
        .finish(&mut ctx.gfx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        todo!();

        // Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        todo!();

        // Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
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
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        todo!();

        // Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if input.keycode == Some(ggez::winit::event::VirtualKeyCode::Escape) {
            ctx.request_quit();
        }

        self.world.resource_mut::<Input>();

        todo!();

        // Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        _input: ggez::input::keyboard::KeyInput,
    ) -> Result<(), ggez::GameError> {
        todo!();

        // Ok(())
    }

    fn text_input_event(
        &mut self,
        _ctx: &mut Context,
        _character: char,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn gamepad_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _btn: ggez::input::gamepad::gilrs::Button,
        _id: event::GamepadId,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn gamepad_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _btn: ggez::input::gamepad::gilrs::Button,
        _id: event::GamepadId,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut Context,
        _axis: ggez::input::gamepad::gilrs::Axis,
        _value: f32,
        _id: event::GamepadId,
    ) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        // debug!("quit_event() callback called, quitting...");
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
        true
    }
}
