use anyhow::{bail, Context, Result};
use args::{ChatOpts, Opts};
use common::client_daemon::{ClientToDaemonMsg, DaemonToClientMsg};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    tty::IsTty,
    ExecutableCommand, QueueableCommand,
};
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use std::{
    fs::File,
    io::{self, Read, StdoutLock, Write},
    ops::ControlFlow,
    os::unix::prelude::AsRawFd,
    panic, thread,
    time::Duration,
};
use structopt::StructOpt;

mod args;

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

    fn try_get_fifos(opts: &ChatOpts) -> Result<(File, File)> {
        let thread = thread::spawn(|| {
            let home_dir = dirs::home_dir().context("couldn't get home directory")?;

            let ctodbuf = File::options()
                .write(true)
                .open(home_dir.join(".rclc/ctodbuf"))
                .context("couldn't open client->daemon fifo")?;

            let dtocbuf = File::open(home_dir.join(".rclc/dtocbuf"))
                .context("couldn't open daemon->client fifo")?;

            Ok((ctodbuf, dtocbuf))
        });

        if !opts.no_timeout {
            thread::sleep(Duration::from_millis(10));
        }

        if opts.no_timeout || thread.is_finished() {
            match thread.join() {
                Ok(o) => o,
                Err(e) => panic::resume_unwind(e),
            }
        } else {
            bail!("daemon is not running (10 ms timeout reached)");
        }
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

    /// clear the prompt line, perform a function (function is assumed to print something
    /// followed by `"\n\r"`), then restore the prompt line. stdout is not flushed.
    fn print_upwards<F>(&mut self, f: F) -> Result<()>
    where
        F: Fn() -> Result<()>,
    {
        self.stdout
            .queue(terminal::Clear(ClearType::CurrentLine))?
            .queue(cursor::MoveToColumn(0))?;
        f()?;
        print!("{}{}", PROMPT, self.input);
        Ok(())
    }

    fn send_fifo_msg(&mut self, msg: &ClientToDaemonMsg) -> Result<()> {
        rmp_serde::encode::write(&mut self.ctodbuf, &msg)
            .with_context(|| format!("couldn't write message {msg:?} to fifo"))
    }

    fn handle_fifo_msg(&mut self) -> Result<()> {
        let mut data = Vec::new();
        self.dtocbuf
            .read_to_end(&mut data)
            .context("couldn't read from fifo")?;
        match rmp_serde::from_slice::<DaemonToClientMsg>(&data) {
            Ok(msg) => print!("got fifo msg: {msg:?}\n\r"),
            Err(e) => self.print_upwards(|| {
                bunt::print!("{$red}invalid message from daemon: {}{/$}\n\r", e);
                Ok(())
            })?,
        }

        Ok(())
    }

    fn handle_term_evt(&mut self) -> Result<ControlFlow<()>> {
        let event = event::read().context("couldn't read next terminal event")?;
        match event {
            Event::Key(kev) => self.handle_kev(kev),
            _ => Ok(ControlFlow::Continue(())),
        }
    }

    fn handle_kev(&mut self, kev: KeyEvent) -> Result<ControlFlow<()>> {
        match kev.code {
            KeyCode::Char('c') if kev.modifiers == KeyModifiers::CONTROL => {
                // notify the event loop to break
                return Ok(ControlFlow::Break(()));
            }
            KeyCode::Backspace => {
                if !self.input.is_empty() {
                    self.input.pop();
                    print!("\x08 \x08");
                }
            }
            KeyCode::Enter => {
                if self.input.is_empty() {
                    return Ok(ControlFlow::Continue(()));
                }

                self.send_fifo_msg(&ClientToDaemonMsg::Send(self.input.clone()))?;
                self.input.clear();
                self.stdout
                    .queue(terminal::Clear(ClearType::CurrentLine))?
                    .queue(cursor::MoveToColumn(0))?;
                print!("{}", PROMPT);
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                print!("{c}");
            }
            _ => (),
        }

        Ok(ControlFlow::Continue(()))
    }
}

fn chat(opts: ChatOpts) -> Result<()> {
    let mut state = State::new(opts)?;
    state.start()?;

    Ok(())
}

#[allow(unused_must_use)]
/// Try our best to clean up the terminal state; if too many errors happen, just print some newlines and call it good.
fn cleanup() {
    let mut stdout = io::stdout().lock();
    if stdout.is_tty() {
        stdout.execute(cursor::Show);
        if terminal::disable_raw_mode().is_ok() {
            print!("\n\n");
        } else {
            print!("\n\r\n\r");
        }
        stdout.execute(terminal::Clear(ClearType::CurrentLine));
    }
}

fn go() -> Result<()> {
    match Opts::from_args() {
        Opts::Chat(chatopts) => chat(chatopts),
    }
}

fn main() {
    if let Err(e) = go() {
        cleanup();
        println!("rclc client error: {e:?}");
    }
}
