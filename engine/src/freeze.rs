use std::default;

#[allow(dead_code)]
/// Declares different kinds of states where the game should stop updating
///
/// Game logic should be disabled if
#[derive(Debug, Default, PartialEq, Clone)]
pub(crate) enum FreezeType {
    /// The game is in focus, and there isn't anything that should be preventing game logic and rendering from updating.
    #[default]
    NONE,
    /// The game window is no longer in focus, depending on settings dim the screen and stop updating game logic, but keep rendering.
    UNFOCUSED,
    /// The game window is minimized and unseeable, disable rendering and potentially game logic
    MINIMIZED,
    /// The game is focused, but in a paused state due to player input. Stop most game logic, but keep rendering
    PAUSED,
    /// The game is currently loading assets. Stop most rendering and typical game logic
    LOADING,
    /// The game is paused for some amount of frames ([`u32`]). Keep rendering, stop most game logic temporarily
    IMPACT(u32),
    /// The game is paused due to debug state. Keep rendering, but stop ALL game logic.
    ///
    /// Contains the previous state to return to when exiting debug state.
    DEBUG(Box<FreezeType>),
}

impl FreezeType {
    /// Compares self to other, and checks if they're of the same variant, regardless of variant parameters.
    pub fn variant_eq(&self, other: &Self) -> bool {
        match self {
            FreezeType::NONE => self == other,
            FreezeType::UNFOCUSED => self == other,
            FreezeType::MINIMIZED => self == other,
            FreezeType::PAUSED => self == other,
            FreezeType::LOADING => self == other,
            FreezeType::IMPACT(_) => todo!(),
            FreezeType::DEBUG(_) => todo!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct FreezeManager {
    current_freeze_state: FreezeType,
}

impl FreezeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(
        &mut self,
        unfocused: bool,
        minimized: bool,
        paused: bool,
        loading: bool,
        impact: bool,
        debug: bool,
    ) {
        if debug {
            return;
        } else if let FreezeType::DEBUG(previous_state) = &self.current_freeze_state {
            self.current_freeze_state = *previous_state.to_owned();
        }

        if unfocused {}
    }
}
