use std::{io, ops::ControlFlow};

use crate::{State, PROMPT};
use anyhow::{Context, Result};
use common::client_daemon::ClientToDaemonMsg;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, ClearType},
    tty::IsTty,
    ExecutableCommand, QueueableCommand,
};

impl<'a> State<'a> {
    /// clear the prompt line, perform a function (function is assumed to print something
    /// followed by `"\n\r"`), then restore the prompt line. stdout is not flushed.
    pub(crate) fn print_upwards<F>(&mut self, f: F) -> Result<()>
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

    pub(crate) fn handle_term_evt(&mut self) -> Result<ControlFlow<()>> {
        let event = event::read().context("couldn't read next terminal event")?;
        match event {
            Event::Key(kev) => self.handle_kev(kev),
            _ => Ok(ControlFlow::Continue(())),
        }
    }

    pub(crate) fn handle_kev(&mut self, kev: KeyEvent) -> Result<ControlFlow<()>> {
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

#[allow(unused_must_use)]
/// Try our best to clean up the terminal state; if too many errors happen, just print some newlines and call it good.
pub fn cleanup() {
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
