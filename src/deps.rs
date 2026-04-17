use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dependency {
    Npm,
    Pnpm,
    Homebrew,
    Vp,
    Fnm,
}

impl Dependency {
    pub const ALL: [Dependency; 5] = [
        Dependency::Npm,
        Dependency::Pnpm,
        Dependency::Homebrew,
        Dependency::Vp,
        Dependency::Fnm,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Dependency::Npm => "npm",
            Dependency::Pnpm => "pnpm",
            Dependency::Homebrew => "homebrew",
            Dependency::Vp => "vp",
            Dependency::Fnm => "fnm",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Dependency::Npm => "Node package manager",
            Dependency::Pnpm => "Another node package manager",
            Dependency::Homebrew => "Package manager for macOS",
            Dependency::Vp => "Vite plus",
            Dependency::Fnm => "Fast Node version manager",
        }
    }
}

impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}
