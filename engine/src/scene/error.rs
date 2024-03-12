use core::fmt::Display;

#[derive(Debug, Clone)]
pub enum SceneError {
    /// No scene was selected as the target when saving.
    ///
    /// Scene was already unloaded/ No scene was selected as the target when unloading.
    NoTargetScene,
    /// Something went wrong while parsing a file.
    /// [`String`] is the IO error message, formatted as a string
    IOError(String),
    /// Something went wrong while trying to gather user input.
    InputError(String),
    /// A scene that was loaded contained a component that has not been registered by the directory.
    /// [`String`] is the path of the missing component.
    MissingTypeRegistry(String),
    /// Failed to instantiate a scene, though not because of an IO error.
    /// Might be because of an ECS failure somewhere.
    ///
    /// [`String`] is the IO error returned by whatever call to the OS failed.
    LoadFailure(String),
    /// Failed to serialize scene data
    SerializeFailure(String),
    /// The scene entity you were trying to operate on did not have a [`Scene`](super::Scene) component.
    NoSceneComponent,
    /// The scene object entity you were working on did not have a [`SceneData`](super::SceneData) component
    NoSceneDataComponent,
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
            SceneError::NoSceneComponent => write!(
                f,
                "Component failure: No Scene component found on scene entity"
            ),
            SceneError::NoSceneDataComponent => write!(
                f,
                "Component failure: No SceneData component found on the object"
            ),
            SceneError::InputError(err) => write!(f, "User input error [{}]", err),
            SceneError::SerializeFailure(err) => write!(f, "Serialize failure [{}]"),
        }
    }
}

impl Into<String> for SceneError {
    fn into(self) -> String {
        self.to_string()
    }
}
