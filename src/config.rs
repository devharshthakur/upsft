use crate::deps::Dependency;
use cliclack::multiselect;
use std::io;

#[derive(Debug, Clone)]
pub struct Config {
    pub dependencies: Vec<Dependency>,
}

impl Config {
    pub fn new(dependencies: Vec<Dependency>) -> Self {
        Self { dependencies }
    }
}

pub fn prompt_config() -> io::Result<Config> {
    let mut prompt = multiselect("Select dependencies this CLI should manage");

    for dependency in Dependency::ALL {
        prompt = prompt.item(dependency, dependency.label(), dependency.hint());
    }

    let dependencies = prompt.interact()?;

    Ok(Config::new(dependencies))
}
