use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::thread;

use common::client_daemon::{serialize_message_client_to_daemon, ClientToDaemonMsg};
use common::notif::notif;

pub fn hook(path: &'static str) {
    thread::spawn(move || {
        let mut stream = match UnixStream::connect(path) {
            Ok(stream) => stream,
            Err(e) => {
                notif(
                    "Client Error",
                    &format!("Could not serialize packet: {:?}", e),
                );
                return;
            }
        };
        let mut to_send = match serialize_message_client_to_daemon(ClientToDaemonMsg::Test(
            "The quick brown fox jumped over the lazy dogs".to_string(),
        )) {
            Ok(b) => b,
            Err(e) => {
                notif(
                    "Client Error",
                    &format!("Could not serialize packet: {:?}", e),
                );
                return;
            }
        };

        to_send.push(10); // Add newline character
        stream.write_all(&to_send).unwrap();
        stream.flush().unwrap();
        // Read the response
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
    });
}
