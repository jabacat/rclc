use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::os::unix::prelude::PermissionsExt;
use std::thread;

use common::net::parse_packet;
use common::net::Packet;

pub fn listen(path: &'static str) {
    thread::spawn(move || {
        // Get rid of the old sock
        std::fs::remove_file(path).ok();

        // Try to handle sock connections then
        let listener = match UnixListener::bind(path) {
            Ok(listener) => listener,
            Err(e) => {
                println!("Failed to bind to {}: {}", path, e);
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
                        println!("Failed to accept connection: {}", err);
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
                        println!("Failed to read line: {}", e);
                        return;
                    }
                },
            };
            let packet = match parse_packet(&actual_line) {
                Ok(p) => p,
                Err(e) => {
                    println!("Received malformed packet: {}", e);
                    Packet::Unknown
                }
            };
            match packet {
                Packet::Hello(hi) => {
                    let hello_packet = Packet::HelloResponse(hi.clone(), 0);
                    println!("Received hello packet: {}", hi);
                    let mut writer = BufWriter::new(&stream);
                    writer
                        .write_all(format!("{}", hello_packet).as_bytes())
                        .unwrap();
                    writer.flush().unwrap();
                }
                Packet::HelloResponse(_, _) => {}
                Packet::Unknown => {}
            };
        }
    });
}
