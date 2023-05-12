use std::{
    io::{BufRead, BufReader},
    os::unix::{
        net::{UnixListener, UnixStream},
        prelude::PermissionsExt,
    },
    thread,
};

use common::{
    client_daemon::{parse_client_to_daemon_message, ClientToDaemonMsg},
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
                        handle_stream(stream);
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

pub fn handle_stream(stream: UnixStream) {
    thread::spawn(move || {
        let reader = BufReader::new(&stream);
        for line in reader.lines() {
            let actual_line = match line {
                Ok(line) => line,
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => {
                        return;
                    }
                    _ => {
                        notif("Stream Error", &format!("Failed to read line: {}", e));
                        return;
                    }
                },
            };
            let message = match parse_client_to_daemon_message(actual_line.into_bytes()) {
                Ok(p) => p,
                Err(e) => {
                    notif("Stream Error", &format!("Received malformed packet: {}", e));
                    ClientToDaemonMsg::Unknown
                }
            };

            println!("Message: {:?}", message);
        }
    });
}
