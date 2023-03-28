use anyhow::{bail, Context, Result};
use args::Opts;
use client_daemon::{
    client_daemon_client::ClientDaemonClient,
    event::Event::{ChatRequest, Message, MessageDelete},
    SendMessageRequest,
};
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
}

impl<'a> State<'a> {
    async fn new() -> Result<State<'a>> {
        let mut stdout = io::stdout().lock();
        if !stdout.is_tty() {
            bail!("stdout is not a tty");
        }

        terminal::enable_raw_mode().context("couldn't enable raw mode")?;

        print!("waiting for daemon connection...");
        // FIXME: wait for address to actually become available before connecting
        let daemon = ClientDaemonClient::connect("http://0.0.0.0:5768").await?;
        stdout.flush()?;
        stdout
            .queue(terminal::Clear(ClearType::CurrentLine))?
            .execute(cursor::MoveToColumn(0))?;

        Ok(Self {
            stdout,
            input: String::new(),
            daemon,
            event_stream: EventStream::new(),
        })
    }

    /// start the event loop
    async fn start(&mut self) -> Result<()> {
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
            bunt::println!(
                "{$green}[+] OK{/$} {$yellow}Got initial UI state from daemon:{/$} {:?}",
                ui_state
            );
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
                                Err(term_e) => panic!("terminal error: {}", term_e),
                            }
                        }
                        None => eprintln!("Blank terminal event")
                    }
                }
                opt = drx.recv() => match opt {
                    Some(msg) => match msg.event {
                        Some(evt) => match evt {
                            Message(mevt) => {
                                self.print_upwards(|| {
                                    bunt::print!("{$green}[+] OK {/$}{$yellow}Message event:{/$} {:?}\n\r", mevt);
                                    Ok(())
                                })?;
                            },
                            MessageDelete(mdevt) => {
                                self.print_upwards(|| {
                                    bunt::print!("{$green}[+] OK {/$}{$yellow}Message delete event:{/$} {:?}\n\r", mdevt);
                                    Ok(())
                                })?;
                            },
                            ChatRequest(crevt) => {
                                self.print_upwards(|| {
                                    bunt::print!("{$green}[+] OK {/$}{$yellow}Chat request event:{/$} {:?}\n\r", crevt);
                                    Ok(())
                                })?;
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

    async fn handle_term_evt(&mut self, event: Event) -> Result<ControlFlow<()>> {
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

                self.daemon
                    .send_message(SendMessageRequest {
                        recipient: 0,
                        content: self.input.clone(),
                    })
                    .await?;
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
