use std::{
    error::Error,
    os::fd::{AsRawFd, OwnedFd},
    str::FromStr,
};

use nix::sys::socket::{
    accept, bind, listen, recv, socket, AddressFamily, Backlog, MsgFlags, SockFlag, SockType,
    SockaddrIn,
};

// our result type to simplify the error handling.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

// create simple tcp server using network sockets.
// by making new socket file descriptor and bind it the given address,
// and listen into it.
fn create_tcp_server(addr: &str) -> Result<OwnedFd> {
    let socket_fd = socket(
        AddressFamily::Inet, // the socket family (AF_INET, which is used for IPv4 addresses)
        SockType::Stream, // the socket type (SOCK_STREAM, which indicates a stream socket using the TCP protocol.)
        SockFlag::empty(), // don't use any flags
        None,             // use the default protocol
    )?;

    let sock_addr = SockaddrIn::from_str(addr)?;

    bind(socket_fd.as_raw_fd(), &sock_addr)?;
    // Listen for incoming connections
    let backlog = Backlog::new(1)?;
    listen(&socket_fd, backlog)?;
    Ok(socket_fd)
}

fn main() -> Result<()> {
    let socket_fd = create_tcp_server("127.0.0.1:8080")?;

    let conn_fd = accept(socket_fd.as_raw_fd())?;

    // Receive the size of the file
    let mut size_buf = [0; 4];
    recv(conn_fd, &mut size_buf, MsgFlags::empty())?;
    println!("Size Buf: {size_buf:?}");
    let file_size = u32::from_be_bytes(size_buf);
    println!("File size: {}", file_size);

    // Receive the file data
    let mut file_buf = vec![0; file_size as usize];

    recv(conn_fd, &mut file_buf, MsgFlags::empty())?;
    println!("File data: {}", String::from_utf8_lossy(&file_buf));
    Ok(())
}
