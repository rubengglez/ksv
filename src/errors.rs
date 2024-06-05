use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("impossible open file")]
    ImpossibleOpenFile(#[from] io::Error),
    #[error("no command")]
    NoCommand,
    #[error("serializing failed")]
    ImpossibleRemoveKeyDoesNotExist,
    #[error("parsing error")]
    ParsingError(#[from] serde_json::Error),
}