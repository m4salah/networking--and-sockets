use std::{
    error::Error,
    os::fd::{AsRawFd, OwnedFd},
    str::FromStr,
};

use nix::sys::socket::{
    connect, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, SockaddrIn,
};

// our result type to simplify the error handling.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

// create simple tcp server using network sockets.
// by making new socket file descriptor and bind it the given address,
// and listen into it.
fn connect_to_tcp(addr: &str) -> Result<OwnedFd> {
    let socket_fd = socket(
        AddressFamily::Inet, // the socket family (AF_INET, which is used for IPv4 addresses)
        SockType::Stream, // the socket type (SOCK_STREAM, which indicates a stream socket using the TCP protocol.)
        SockFlag::empty(), // don't use any flags
        None,             // use the default protocol
    )?;

    let sock_addr = SockaddrIn::from_str(addr).expect("Failed to create socket address");

    connect(socket_fd.as_raw_fd(), &sock_addr)?;

    Ok(socket_fd)
}

fn main() -> Result<()> {
    let socket_fd = connect_to_tcp("127.0.0.1:8080")?;

    // Read the file into a buffer
    let buffer = std::fs::read("./data.txt")?;

    // Send the file size to the server.
    let file_size_in_bytes = u32::to_be_bytes(buffer.len() as u32);
    send(
        socket_fd.as_raw_fd(),
        &file_size_in_bytes,
        MsgFlags::empty(),
    )?;
    println!("Send the size: {}", buffer.len());
    println!("Send bytes: {:?}", file_size_in_bytes);

    // Send the file to the server.
    send(socket_fd.as_raw_fd(), &buffer, MsgFlags::empty())?;
    println!("Send the data: {}", String::from_utf8_lossy(&buffer));

    Ok(())
}
