use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::{fs, thread};

fn handle_client(mut stream: UnixStream) {
    let mut buf = [0u8; 128];
    match stream.read(&mut buf) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buf[..size]);
            println!("Received request: {}", request);

            let response = "HTTP/1.1 200 OK\r\n\
                            Content-Type: text/html; charset=UTF-8\r\n\
                            Content-Length: 13\r\n\
                            \r\n\
                            Hello, world!";

            stream
                .write_all(response.as_bytes())
                .expect("Failed to wirte to the client");
        }
        Err(err) => eprintln!("Failed to read from client: {}", err),
    }
}

///  This is http server using unix domain open socket domain in `/tmp/hello.sock`
///  To test the server if it's working you can run `cargo run --bin unix-domain-http-client`
///  or run `curl -v -s -N --unix-socket /tmp/hello.sock http://localhost/` using curl.
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
