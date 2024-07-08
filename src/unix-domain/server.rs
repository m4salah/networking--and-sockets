use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::{fs, thread};

fn handle_client(mut stream: UnixStream) {
    let mut buf = [0u8; 128];
    match stream.read(&mut buf) {
        Ok(size) => {
            let received_message = String::from_utf8_lossy(&buf[..size]);
            println!("Received message: {}", received_message);

            stream
                .write_all(&buf[..size])
                .expect("Failed to write to echo client");
        }
        Err(err) => eprintln!("Failed to read from client: {}", err),
    }
}

/// This is simple echo unix domain server open socket domain on `/tmp/hello.sock`
/// and accept the connetion on this file from anywhere in the local network.
/// to test if the server is working run the unix domain client by `cargo run --bin
/// unix-domain-client` to send hello world to the server
/// or use something like nc by running this command `echo "Hello, World" | nc -U /tmp/hello.sock`
fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/hello.sock";

    // Clean up any existing socket file
    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path).unwrap();
    }

    let listener = UnixListener::bind("/tmp/hello.sock")?;

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                eprintln!("Error while listing to incoming connection: {err}");
                break;
            }
        }
    }

    // Clean up
    fs::remove_file(socket_path).unwrap();
    Ok(())
}
