use anyhow::{bail, Context, Result};
use args::{ChatOpts, Opts};
use crossterm::{event, terminal, tty::IsTty};
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use std::{
    fs::File,
    io::{self, StdoutLock, Write},
    ops::ControlFlow,
    os::unix::prelude::AsRawFd,
    time::Duration,
};
use structopt::StructOpt;

mod args;
mod fifo;
mod tui;

const PROMPT: &str = "> ";

struct State<'a> {
    stdout: StdoutLock<'a>,

    /// the current text in the input field
    input: String,

    /// a handle to the `~/.rclc/dtocbuf` fifo in read-only mode
    dtocbuf: File,

    /// a handle to the `~/.rclc/ctodbuf` fifo in write-only mode
    ctodbuf: File,
}

impl<'a> State<'a> {
    fn new(opts: ChatOpts) -> Result<Self> {
        let stdout = io::stdout().lock();
        if !stdout.is_tty() {
            bail!("stdout is not a tty");
        }

        terminal::enable_raw_mode().context("couldn't enable raw mode")?;

        let (ctodbuf, dtocbuf) = Self::try_get_fifos(&opts)?;

        Ok(Self {
            stdout,
            input: String::new(),
            dtocbuf,
            ctodbuf,
        })
    }

    /// start the event loop
    fn start(&mut self) -> Result<()> {
        // a mio poll lets you monitor for readiness events from multiple sources.
        let mut poll = Poll::new().context("failed to start mio poll")?;
        let mut events = Events::with_capacity(1024);

        // register the dtoc fifo to notify the poll whenever it is readable (whenever a new
        // message from the daemon is available to read).
        poll.registry()
            .register(
                &mut SourceFd(&self.dtocbuf.as_raw_fd()),
                Token(0),
                Interest::READABLE,
            )
            .context("could not register daemon->client fifo with mio poll")?;

        // register stdin to notify the poll whenever it is readalbe (whenever the user presses
        // a key).
        poll.registry()
            .register(
                &mut SourceFd(&io::stdin().as_raw_fd()),
                Token(1),
                Interest::READABLE,
            )
            .context("could not register stdin with mio poll")?;

        print!("{}", PROMPT);
        self.stdout.flush()?;

        'evt_loop: loop {
            // block until the next poll event happens
            poll.poll(&mut events, None)
                .context("failed to poll mio poll")?;

            for event in &events {
                match event.token() {
                    Token(0) => self.handle_fifo_msg()?,
                    Token(1) => match self.handle_term_evt()? {
                        ControlFlow::Continue(()) => (),
                        ControlFlow::Break(()) => break 'evt_loop,
                    },
                    _ => unreachable!(),
                }
            }

            // temporary fix for initial buffering problem
            while event::poll(Duration::ZERO)? {
                self.handle_term_evt()?;
            }

            self.stdout.flush()?;
        }

        Ok(())
    }
}

fn chat(opts: ChatOpts) -> Result<()> {
    let mut state = State::new(opts)?;
    state.start()?;

    Ok(())
}

fn go() -> Result<()> {
    match Opts::from_args() {
        Opts::Chat(chatopts) => chat(chatopts),
    }
}

fn main() {
    if let Err(e) = go() {
        tui::cleanup();
        println!("rclc client error: {e:?}");
    }
}
