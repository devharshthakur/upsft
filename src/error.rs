use std::io;

use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}
