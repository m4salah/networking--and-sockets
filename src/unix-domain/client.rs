use std::io::prelude::*;
use std::os::unix::net::UnixStream;

/// This is unix domain client connect to socket domain on `/tmp/hello.sock`
/// make sure to run the server first by `cargo run --bin unix-domain-server`
fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/hello.sock")?;
    stream.write_all(b"hello world")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}
