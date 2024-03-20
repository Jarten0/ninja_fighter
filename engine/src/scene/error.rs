use core::fmt::Display;

#[derive(Debug)]
pub enum SceneError {
    /// No scene was selected as the target when saving.
    ///
    /// Scene was already unloaded/ No scene was selected as the target when unloading.
    NoTargetScene,
    /// Something went wrong while parsing a file.
    /// [`String`] is the IO error message, formatted as a string
    IOError(std::io::Error),
    /// Something went wrong while trying to gather user input.
    InputError(String),
    /// A scene that was loaded contained a component that has not been registered by the directory.
    /// [`String`] is the path of the missing component.
    MissingTypeRegistry(String),
    /// The type that's trying to be serialized does not have reflection type data inserted into the registry.
    /// To fix, add #[reflect(Component)] to your type
    NoReflectData(String),
    /// The type that's trying to be serialized does not have the serialize trait.
    /// String is the type that is missing the trait.
    NoSerializationImplementation(String),
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
    /// There were no entities available to operate upon.
    NoEntitiesAvailable,
}

impl Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneError::NoTargetScene => write!(f, "No target scene found"),
            SceneError::IOError(err) => write!(f, "Scene IO error [{}]", err),
            SceneError::MissingTypeRegistry(err) => {
                write!(f, "Missing type registry [{}]", err)
            }
            SceneError::LoadFailure(err) => write!(f, "Scene Load failure [{}]", err),
            SceneError::NoSceneComponent => write!(
                f,
                "Component failure: No Scene component found on scene entity"
            ),
            SceneError::NoSceneDataComponent => write!(
                f,
                "Component failure: No SceneData component found on the object"
            ),
            SceneError::InputError(err) => write!(f, "Scene User input error [{}]", err),
            SceneError::SerializeFailure(err) => {
                write!(f, "Scene Serialize failure [{}]", err.to_string())
            }
            SceneError::NoReflectData(err) => write!(f, "No Reflection data [{}]", err),
            SceneError::NoEntitiesAvailable => write!(f, "No available entities"),
            SceneError::NoSerializationImplementation(missing_type) => write!(
                f,
                "Missing Serialization Implementation for {}",
                missing_type
            ),
        }
    }
}

impl Into<String> for SceneError {
    fn into(self) -> String {
        self.to_string()
    }
}
