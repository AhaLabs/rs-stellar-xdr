mod list;
mod schema;

use clap::{Args, Subcommand};

use crate::cli::Channel;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SchemaError(#[from] schema::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[command(subcommand)]
    sub: Sub,
}

#[derive(Subcommand, Clone, Debug)]
pub enum Sub {
    List(list::Cmd),
    Schema(schema::Cmd),
}

impl Cmd {
    pub fn run(&self, channel: &Channel) -> Result<(), Error> {
        match &self.sub {
            Sub::List(c) => c.run(channel),
            Sub::Schema(c) => c.run(channel)?,
        }
        Ok(())
    }
}
