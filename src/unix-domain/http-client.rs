use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

///  This is http client using unix domain connect to socket domain on `/tmp/hello.sock`
///  To test the clientt if it's working, run first the server by `cargo run --bin unix-domain-http-server`
fn main() {
    let socket_path = "/tmp/hello.sock";

    // Connect to the server
    match UnixStream::connect(socket_path) {
        Ok(mut stream) => {
            // Send an HTTP GET request to the server
            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();

            // Read the response from the server
            let mut buf = [0u8; 1024];
            match stream.read(&mut buf) {
                Ok(size) => {
                    let response = String::from_utf8_lossy(&buf[..size]);
                    println!("Received response: {}", response);
                }
                Err(err) => eprintln!("Failed to read from server: {}", err),
            }
        }
        Err(err) => eprintln!("Failed to connect to server: {}", err),
    }
}
