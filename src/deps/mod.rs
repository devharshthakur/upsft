use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Dependency {
    pub label: String,
    pub hint: String,
    pub update_command: String,
}

impl Dependency {
    pub fn new() -> Result<HashMap<String, Dependency>, serde_json::Error> {
        let raw_json_str = include_str!("deps.json");
        let deps: Vec<Dependency> = match serde_json::from_str(raw_json_str) {
            Ok(d) => d,
            Err(e) => {
                eprint!("Failed to load default deps");
                return Err(e);
            }
        };
        Ok(deps.into_iter().map(|d| (d.label.clone(), d)).collect())
    }

    pub fn add(deps: &mut HashMap<String, Dependency>, key: String, dependency: Dependency) {
        deps.insert(key, dependency);
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}
