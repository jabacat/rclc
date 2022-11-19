use anyhow::{bail, Context, Result};
use args::Opts;
use client_daemon::{
    client_daemon_client::ClientDaemonClient,
    event::Event::{ChatRequest, Message, MessageDelete},
    SendMessageRequest,
};
//use common::client_daemon::{ClientToDaemonMsg, DaemonToClientMsg};
use crossterm::event::EventStream;
use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    tty::IsTty,
    ExecutableCommand, QueueableCommand,
};
use std::{
    io::{self, StdoutLock, Write},
    ops::ControlFlow,
};
use tokio_stream::StreamExt;
use tonic::transport::Channel;

pub mod client_daemon {
    tonic::include_proto!("clientdaemon");
}

mod args;

const PROMPT: &str = "> ";

struct State<'a> {
    stdout: StdoutLock<'a>,

    /// the current text in the input field
    input: String,

    daemon: ClientDaemonClient<Channel>,

    event_stream: EventStream,
    /*/// a handle to the `~/.rclc/dtocbuf` fifo in read-only mode
    dtocbuf: File,

    /// a handle to the `~/.rclc/ctodbuf` fifo in write-only mode
    ctodbuf: File,*/
}

impl<'a> State<'a> {
    async fn new() -> Result<State<'a>> {
        let mut stdout = io::stdout().lock();
        if !stdout.is_tty() {
            bail!("stdout is not a tty");
        }

        terminal::enable_raw_mode().context("couldn't enable raw mode")?;

        //let home_dir = dirs::home_dir().context("couldn't get home directory")?;

        print!("waiting for daemon connection...");
        stdout.flush()?;
        /*// aquire a write handle on the ctod fifo
        // this blocks the client until the daemon opens the file to read
        let ctodbuf = File::options()
            .write(true)
            .open(home_dir.join(".rclc/ctodbuf"))
            .context("couldn't open client->daemon fifo")?;

        // clear waiting message
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))?
            .execute(cursor::MoveToColumn(0))?;

        print!("waiting for daemon response...");
        stdout.flush()?;
        // aquire a read handle on the dtoc file
        // this blocks until the daemon opens the file to write
        let dtocbuf = File::open(home_dir.join(".rclc/dtocbuf"))
            .context("couldn't open daemon->client fifo")?;*/

        // clear waiting message
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))?
            .execute(cursor::MoveToColumn(0))?;

        let client = ClientDaemonClient::connect("http://0.0.0.0:5768").await?;

        Ok(Self {
            stdout,
            input: String::new(),
            daemon: client,
            event_stream: EventStream::new(),
        })
    }

    /// start the event loop
    async fn start(&mut self) -> Result<()> {
        // a mio poll lets you monitor for readiness events from multiple sources.
        /*let poll = Poll::new().context("failed to start mio poll")?;
        let events = Events::with_capacity(1024);*/

        /*// register the dtoc fifo to notify the poll whenever it is readable (whenever a new
        // message from the daemon is available to read).
        poll.registry()
            .register(
                &mut SourceFd(&self.dtocbuf.as_raw_fd()),
                Token(0),
                Interest::READABLE,
            )
            .context("could not register daemon->client fifo with mio poll")?;*/

        // register stdin to notify the poll whenever it is readalbe (whenever the user presses
        // a key).
        /*poll.registry()
            .register(
                &mut SourceFd(&io::stdin().as_raw_fd()),
                Token(1),
                Interest::READABLE,
            )
            .context("could not register stdin with mio poll")?;*/

        let mut stream = self.daemon.subscribe_to_events(()).await?.into_inner();
        let (dtx, mut drx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move {
            while let Some(maybe_event) = stream.next().await {
                match maybe_event {
                    Ok(event) => dtx.send(event).await.unwrap(),
                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        });

        print!("{}", PROMPT);
        self.stdout.flush()?;

        let ui_state = self.daemon.get_ui_state(()).await?.into_inner();
        self.print_upwards(|| {
            bunt::println!("{$green}[+] OK{/$} {$yellow}Got initial UI state from daemon:{/$} {:?}", ui_state);
            Ok(())
        })?;

        'evt_loop: loop {
            tokio::select! {
                event = self.event_stream.next() => {
                    match event {
                        Some(rst) => {
                            match rst {
                                Ok(evt) => match self.handle_term_evt(evt).await? {
                                    ControlFlow::Continue(()) => (),
                                    ControlFlow::Break(()) => break 'evt_loop,
                                },
                                Err(poo) => panic!("poo occurred: {poo}"),
                            }
                        }
                        None => panic!("I can't program")
                    }
                }
                opt = drx.recv() => match opt {
                    Some(msg) => match msg.event {
                        Some(evt) => match evt {
                            Message(mevt) => {
                                bunt::print!("{$green}[+] OK {/$}{$yellow}Message event:{/$} {:?}\n\raw", mevt);
                                //print!("[+ok] MESSAGE event: {:?}", mevt);
                            },
                            MessageDelete(mdevt) => {
                                bunt::print!("{$green}[+] OK {/$}{$yellow}Message delete event:{/$} {:?}\n\r", mdevt);
                                //println!("DELETED event: {:?}", mdevt);
                            },
                            ChatRequest(crevt) => {
                                bunt::print!("{$green}[+] OK {/$}{$yellow}Scammer Likely:{/$} {:?}\n\r", crevt);
                                //println!("Scammer Likely: {:?}", crevt);
                            }
                        }
                        None => self.print_upwards(|| {
                            bunt::eprintln!("{$red}[-] XX Blank message from daemon{/$}");
                            Ok(())
                        })?
                    }
                    None => self.print_upwards(|| {
                        bunt::eprintln!("{$red}[-] XX Connection to daemon closed{/$}");
                        Ok(())
                    })?
                }
            }
            /*// block until the next poll event happens
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
            }*/

            // XXX: hopefully this code change fixes this issue anyways
            /*// temporary fix for initial buffering problem
            while event::poll(Duration::ZERO)? {
                self.handle_term_evt()?;
            }*/

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

    /*fn send_fifo_msg(&mut self, msg: &ClientToDaemonMsg) -> Result<()> {
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
    }*/

    async fn handle_term_evt(&mut self, event: Event) -> Result<ControlFlow<()>> {
        //let event = event::read().context("couldn't read next terminal event")?;
        match event {
            Event::Key(kev) => self.handle_kev(kev).await,
            _ => Ok(ControlFlow::Continue(())),
        }
    }

    async fn handle_kev(&mut self, kev: KeyEvent) -> Result<ControlFlow<()>> {
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

                self.daemon.send_message(SendMessageRequest {
                    recipient: 0,
                    content: self.input.clone(),
                }).await?;
                //self.send_fifo_msg(&ClientToDaemonMsg::Send(self.input.clone()))?;
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

async fn chat() -> Result<()> {
    let mut state = State::new().await?;
    state.start().await?;

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

async fn go() -> Result<()> {
    match Opts::get() {
        Opts::Chat => chat().await,
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = go().await {
        cleanup();
        println!("rclc client error: {e:?}");
    };
}
