// Dependecy structure
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub struct Dependency {
    pub name: String,
    pub update_command: String,
}

impl Dependency {
    pub fn new(name: String, update_command: String) -> Self {
        Self {
            name,
            update_command,
        }
    }
}
