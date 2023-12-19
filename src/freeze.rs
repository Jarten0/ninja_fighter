#[allow(dead_code)]
pub(crate) enum FreezeType {
    UNFOCUSED,
    MINIMIZED,
    PAUSED,
    LOADING,
    IMPACT(u32),
}
