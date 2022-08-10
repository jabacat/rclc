use std::{
    fs::{self},
    os::unix::prelude::FileTypeExt, path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use args::Opts;
use common::client_daemon::DaemonToClientMsg;

pub mod args;

fn chat(fifo: PathBuf) -> Result<()> {
    let metadata = fs::metadata(&fifo).context("couldn't get metadata of fifo")?;
    if !metadata.file_type().is_fifo() {
        bail!("{fifo:?} is not a fifo");
    }

    loop {
        let bytes = fs::read(&fifo).context("couldn't read fifo")?;
        match rmp_serde::from_slice::<DaemonToClientMsg>(&bytes) {
            Ok(msg) => println!("recieved message: {msg:?}"),
            Err(e) => println!("recieved invalid message: {e}"),
        }
    }
}

fn main() -> Result<()> {
    match Opts::get() {
        Opts::Chat { fifo } => chat(fifo),
    }
}
