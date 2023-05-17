use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    os::unix::{
        net::{UnixListener, UnixStream},
        prelude::PermissionsExt,
    },
    thread,
};

use common::{
    client_daemon::{
        parse_message, serialize_message_daemon_to_client, ClientToDaemonMsg, DaemonToClientMsg,
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
            let message = match parse_message(actual_line.into_bytes()) {
                Ok(p) => p,
                Err(e) => {
                    notif("Stream Error", &format!("Received malformed packet: {}", e));
                    ClientToDaemonMsg::Unknown
                }
            };

            println!("Message: {:?}", message);

            let to_send = match serialize_message_daemon_to_client(DaemonToClientMsg::Test(
                "The quick brown fox jumped over the lazy dogs".to_string(),
            )) {
                Ok(b) => b,
                Err(e) => {
                    notif(
                        "Stream Error",
                        &format!("Could not serialize packet: {:?}", e),
                    );
                    return;
                }
            };
            println!("Returning: {:?}", to_send);

            let mut writer = BufWriter::new(&stream);
            writer.write_all(&to_send).unwrap();
            writer.flush().unwrap();
        }
    });
}
