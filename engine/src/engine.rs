//! Contains resources useful for interoperability between [`ggez`] and [`bevy_ecs`]

use bevy_ecs::system::Resource;
use ggez::graphics::Canvas;

/// A basic resource designed for holding information and sharing access to [`ggez`] through [`bevy_ecs`]'s resource system.
///
/// # Fields
///
/// * `context_ptr` - private raw pointer pointing to the current [`Context`] for the given schedule.
///
/// * `current_canvas` - private optional holding the current [`Canvas`].
/// Holds [`None`] if operating during an `Update` frame, or holds `Some(Canvas)` if operating during a `Draw` frame.

#[derive(Debug, Resource)]
pub struct GgezInterface
where
    Self: 'static,
{
    current_canvas: Option<Canvas>,

    pub(crate) context_ptr: *mut ggez::Context,

    /// Whether debug functionality should be enabled or not.
    pub debug_mode: bool,
}

unsafe impl Send for GgezInterface {}
unsafe impl Sync for GgezInterface {}

#[allow(dead_code)]
impl GgezInterface {
    pub(crate) fn new(context_ptr: &mut ggez::Context) -> Self {
        Self {
            current_canvas: None,
            context_ptr,
            debug_mode: false,
        }
    }
    /// Returns a reference to the current canvas [`ggez`] will operate on.
    pub fn get_canvas(&self) -> Option<&Canvas> {
        self.current_canvas.as_ref()
    }
    pub fn get_canvas_mut(&mut self) -> Option<&mut Canvas> {
        self.current_canvas.as_mut()
    }
    pub(crate) fn set_canvas(&mut self, canvas: Canvas) {
        self.current_canvas = Some(canvas);
    }
    pub(crate) fn take_canvas(&mut self) -> Option<Canvas> {
        self.current_canvas.take()
    }

    pub fn is_debug_draw(&self) -> bool {
        true
    }

    pub fn is_debug_mode(&self) -> bool {
        false
    }

    /// Returns a reference to the value that `self.context_ptr` points to.
    /// Panics if `self.context_ptr` is null or invalid, which should never be the case in normal scenarios. If it is, investigate immediately.
    pub fn get_context(&self) -> &ggez::Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => {
                    panic!("`MainCanvas.context_ptr` is null! `context_ptr` should never be null!")
                }
                false => return self.context_ptr.as_ref().expect(
                    "`MainCanvas.context_ptr` is invalid! Something fundamental has gone wrong!",
                ),
            }
        }
    }

    /// Returns a mutable reference to the value that `self.context_ptr` points to.
    /// Panics if `self.context_ptr` is null or invalid, which should never be the case in normal scenarios. If it is, investigate immediately.
    pub fn get_context_mut(&mut self) -> &mut ggez::Context {
        unsafe {
            match self.context_ptr.is_null() {
                true => {
                    panic!("`MainCanvas.context_ptr` is null! `context_ptr` should never be null!")
                }
                false => return self.context_ptr.as_mut().expect(
                    "`MainCanvas.context_ptr` is invalid! Something fundamental has gone wrong!",
                ),
            }
        }
    }
}
