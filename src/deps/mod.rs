use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Deserialize)]
struct RawDependency {
    label: String,
    hint: String,
    update_command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub hint: String,
    pub update_command: String,
}

impl Dependency {
    pub fn new() -> Result<HashMap<String, Dependency>, serde_json::Error> {
        let raw_json_str = include_str!("deps.json");
        let raw_deps: Vec<RawDependency> = match serde_json::from_str(raw_json_str) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };
        Ok(raw_deps
            .into_iter()
            .map(|d| {
                (
                    d.label,
                    Dependency {
                        hint: d.hint,
                        update_command: d.update_command,
                    },
                )
            })
            .collect())
    }

    pub fn add(deps: &mut HashMap<String, Dependency>, key: String, dependency: Dependency) {
        deps.insert(key, dependency);
    }
}
