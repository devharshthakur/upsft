#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, tabled::Tabled)]
pub struct Dependency {
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Update command")]
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
