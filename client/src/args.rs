use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Opts {
    Chat {
        /// path of the FIFO (named unix pipe file) through which the client and daemon will be
        /// communicating.
        // TODO: have a default value for this once the client-daemon protocol is more fleshed out.
        #[structopt(short, long)]
        fifo: PathBuf,
    },
}

impl Opts {
    pub fn get() -> Self {
        Self::from_args()
    }
}
