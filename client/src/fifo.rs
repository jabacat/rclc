use crate::{args::ChatOpts, State};
use anyhow::{bail, Context, Result};
use common::client_daemon::{ClientToDaemonMsg, DaemonToClientMsg};
use std::{fs::File, io::Read, panic, thread, time::Duration};

impl<'a> State<'a> {
    /// try to open the two fifo files in their respective modes. timeout after 10 ms, unless the
    /// `--no-timeout` option was provided.
    pub(crate) fn try_get_fifos(opts: &ChatOpts) -> Result<(File, File)> {
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

    pub(crate) fn send_fifo_msg(&mut self, msg: &ClientToDaemonMsg) -> Result<()> {
        rmp_serde::encode::write(&mut self.ctodbuf, &msg)
            .with_context(|| format!("couldn't write message {msg:?} to fifo"))
    }

    pub(crate) fn handle_fifo_msg(&mut self) -> Result<()> {
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
}
