use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub label: String,
    pub hint: String,
    pub update_command: String,
}

impl Dependency {
    pub fn new() -> HashMap<String, Dependency> {
        let mut deps = HashMap::new();
        deps.insert(
            "npm".to_string(),
            Dependency {
                label: "npm".to_string(),
                hint: "Node package manager".to_string(),
                update_command: "npm install --global".to_string(),
            },
        );
        deps.insert(
            "pnpm".to_string(),
            Dependency {
                label: "pnpm".to_string(),
                hint: "Another node package manager".to_string(),
                update_command: "pnpm update --latest --global".to_string(),
            },
        );
        deps.insert(
            "homebrew".to_string(),
            Dependency {
                label: "homebrew".to_string(),
                hint: "Package manager for macOS".to_string(),
                update_command: "brew update && brew upgrade && brew cleanup".to_string(),
            },
        );
        deps.insert(
            "vp".to_string(),
            Dependency {
                label: "vp".to_string(),
                hint: "Vite plus".to_string(),
                update_command: String::new(),
            },
        );
        deps
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
