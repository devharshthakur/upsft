use crate::{deps, util::execute_command};
use std::{fmt, process::Output};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dependency {
    Npm,
    Pnpm,
    Homebrew,
    Vp,
}

impl Dependency {
    pub const ALL: [Dependency; 4] = [
        Dependency::Npm,
        Dependency::Pnpm,
        Dependency::Homebrew,
        Dependency::Vp,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Dependency::Npm => "npm",
            Dependency::Pnpm => "pnpm",
            Dependency::Homebrew => "homebrew",
            Dependency::Vp => "vp",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Dependency::Npm => "Node package manager",
            Dependency::Pnpm => "Another node package manager",
            Dependency::Homebrew => "Package manager for macOS",
            Dependency::Vp => "Vite plus",
        }
    }

    pub fn update_dep(self, dep: deps::Dependency) -> Result<Output, std::io::Error> {
        match dep {
            Dependency::Npm => execute_command("npm install --global"),
            Dependency::Homebrew => execute_command("brew update && brew upgrade && brew cleanup"),
            Dependency::Pnpm => execute_command("pnpm update --latest --global"),
            Dependency::Vp => unimplemented!("Vp update not implemented"),
        }
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}
