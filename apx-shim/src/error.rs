use std::io;

use thiserror::Error;


#[derive(Error, Debug)]
pub enum ApxError
{
  #[error("Command error: {error}")]
  CommandError{ error: String},

  #[error("IO Error")]
  IoError(#[from] io::Error),
}

