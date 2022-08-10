use std::{
    fs::{self},
    os::unix::prelude::FileTypeExt,
};

use anyhow::{bail, Context, Result};
use args::Opts;

pub mod args;

fn main() -> Result<()> {
    match Opts::get() {
        Opts::Chat { fifo } => {
            let metadata = fs::metadata(&fifo).context("couldn't get metadata of fifo")?;
            if !metadata.file_type().is_fifo() {
                bail!("{fifo:?} is not a fifo");
            }

            loop {
                let bytes = fs::read(&fifo).context("couldn't read fifo")?;
                println!("got message: {:02x?}", bytes);
            }
        }
    }
}
