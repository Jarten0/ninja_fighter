use bevy_ecs::system::Resource;
use ggez::graphics::Canvas;
use ggez::Context;

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

#[derive(Debug, Resource)]
pub struct GameInfo
where
    Self: 'static,
{
    pub current_canvas: Option<Canvas>,
    pub context_ptr: *mut Context,
    // game_root_ptr: *mut GameRoot,
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
