use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Opts {
    Chat,
}

impl Opts {
    pub fn get() -> Self {
        Self::from_args()
    }
}
