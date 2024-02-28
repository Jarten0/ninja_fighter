use core::fmt::Display;

#[derive(Debug)]
pub enum SceneError {
    /// No scene was selected as the target when saving.
    NoTargetScene,
    /// Something went wrong while parsing a file.
    /// [`String`] is the IO error message, formatted as a string
    IOError(String),
    /// A scene that was loaded contained a component that has not been registered by the directory.
    /// [`String`] is the path of the missing component.
    MissingTypeRegistry(String),
    /// Failed to instantiate a scene, though not because of an IO error.
    /// Might be because of an ECS failure somewhere.
    LoadFailure(String),
}

impl Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneError::NoTargetScene => write!(f, "No target scene found"),
            SceneError::IOError(err) => write!(f, "IO error [{}]", err),
            SceneError::MissingTypeRegistry(err) => {
                write!(f, "Missing type registry [{}]", err)
            }
            SceneError::LoadFailure(err) => write!(f, "Load failure [{}]", err),
        }
    }
}

impl Into<String> for SceneError {
    fn into(self) -> String {
        self.to_string()
    }
}
