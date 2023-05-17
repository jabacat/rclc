use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    os::unix::{
        net::{UnixListener, UnixStream},
        prelude::PermissionsExt,
    },
    thread,
};

use common::{
    client_daemon::{
        parse_message_client_to_daemon, serialize_message_daemon_to_client, ClientToDaemonMsg,
        DaemonToClientMsg,
    },
    notif::notif,
};

pub fn listen(path: &'static str) {
    thread::spawn(move || {
        // Get rid of the old sock
        std::fs::remove_file(path).ok();

        // Try to handle sock connections then
        let listener = match UnixListener::bind(path) {
            Ok(listener) => listener,
            Err(e) => {
                notif(
                    "Daemon Error",
                    &format!("Failed to bind to {}: {}", path, e),
                );
                return;
            }
        };

        // Set the permissions on the sock
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o777)).ok();

        // Spawn a new thread to listen for commands
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        match handle_stream(stream) {
                            Ok(_) => {}
                            Err(err) => {
                                notif(
                                    "Daemon Error",
                                    &format!("Failed to accept connection: {}", err),
                                );
                                break;
                            }
                        };
                    }
                    Err(err) => {
                        notif(
                            "Daemon Error",
                            &format!("Failed to accept connection: {}", err),
                        );
                        break;
                    }
                }
            }
        });
    });
}

pub fn handle_stream(stream: UnixStream) -> io::Result<()> {
    thread::spawn(move || -> io::Result<()> {
        let reader = BufReader::new(&stream);
        for line in reader.lines() {
            let actual_line = match line {
                Ok(line) => line,
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => {
                        return Err(e);
                    }
                    _ => {
                        notif("Stream Error", &format!("Failed to read line: {}", e));
                        return Err(e);
                    }
                },
            };
            let message = match parse_message_client_to_daemon(actual_line.into_bytes()) {
                Ok(p) => p,
                Err(e) => {
                    notif("Stream Error", &format!("Received malformed packet: {}", e));
                    ClientToDaemonMsg::Unknown
                }
            };

            println!("Message: {:?}", message);

            // TODO: This would be an excellent use for a macro
            let to_send = match serialize_message_daemon_to_client(DaemonToClientMsg::Test(
                "The quick brown fox jumped over the lazy dogs".to_string(),
            )) {
                Ok(b) => b,
                Err(e) => {
                    notif(
                        "Stream Error",
                        &format!("Could not serialize packet: {:?}", e),
                    );
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to parse message",
                    ));
                }
            };
            println!("Returning: {:?}", to_send);

            let mut writer = BufWriter::new(&stream);
            writer.write_all(&to_send)?;
            writer.flush()?;
        }
        Ok(())
    });
    Ok(())
}
