/// Dependencies loaded from config
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct RawDependency {
    pub label: String,
    pub(crate) hint: String,
    pub(crate) update_command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub hint: String,
    pub update_command: String,
}
