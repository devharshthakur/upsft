/// A managed dependency with a name and its update command.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub struct Dependency {
    /// Unique identifier for this dependency.
    pub name: String,
    /// Shell command to update this dependency.
    pub update_command: String,
}

impl Dependency {
    /// Create a new [`Dependency`] from a name and update command.
    pub fn new(name: String, update_command: String) -> Self {
        Self {
            name,
            update_command,
        }
    }
}
