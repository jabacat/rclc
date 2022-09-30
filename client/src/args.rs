use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Opts {
    Chat(ChatOpts),
}

#[derive(StructOpt)]
pub struct ChatOpts {
    /// whether or not to give up on connecting to the daemon after 10 ms
    #[structopt(long)]
    pub no_timeout: bool,
}
