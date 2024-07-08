use nix::sys::socket::{
    accept, bind, listen, recv, send, socket, AddressFamily, Backlog, MsgFlags, SockFlag, SockType,
    SockaddrIn,
};
use std::{os::fd::AsRawFd, str::FromStr};

/// This is the server
fn main() {
    // 1. create the socket file descriptor
    // https://www.man7.org/linux/man-pages/man2/socket.2.html
    let socket_fd = socket(
        AddressFamily::Inet, // the socket family (AF_INET, which is used for IPv4 addresses)
        SockType::Stream, // the socket type (SOCK_STREAM, which indicates a stream socket using the TCP protocol.)
        SockFlag::empty(), // don't use any flags
        None,             // use the default protocol
    )
    .expect("Failed to create socket");

    // 2. Create a socket address
    // we are using this ip with port in this format because we are using INET family
    // this allow socket to handle different protocol with different formats.
    // now we are using INET family which uses IPv4
    let sock_addr =
        SockaddrIn::from_str("127.0.0.1:6797").expect("Failed to create socket address");

    // 3. Bind the socket to the address
    // https://www.man7.org/linux/man-pages/man2/bind.2.html
    bind(socket_fd.as_raw_fd(), &sock_addr).expect("Failed to bind socket");

    // 4. Listen for incoming connections
    // The backlog parameter specifies the maximum length of the queue of pending connections
    let backlog = Backlog::new(1).expect("Failed to create backlog");
    // https://man7.org/linux/man-pages/man2/listen.2.html
    listen(&socket_fd, backlog).expect("Failed to listen for connections");

    // 5. Accept incoming connections
    // https://man7.org/linux/man-pages/man2/accept.2.html
    let conn_fd = accept(socket_fd.as_raw_fd()).expect("Failed to accept connection");

    // 6. Read data
    let mut buf = [0u8; 1024];
    // https://man7.org/linux/man-pages/man2/recv.2.html
    // we can use read or recv
    // recv without any flags is just read https://man7.org/linux/man-pages/man2/read.2.html
    // The only difference between recv() and read(2) is the presence of
    // flags.  With a zero flags argument, recv() is generally
    // equivalent to read(2) (but see NOTES).  Also, the following call
    let bytes_read =
        recv(conn_fd, &mut buf, MsgFlags::empty()).expect("Failed to read from connection");
    // let bytes_read = read(conn_fd, &mut buf).expect("Failed to read from connection");
    let received_data =
        std::str::from_utf8(&buf[..bytes_read]).expect("Failed to convert received data to string");
    println!(
        "Received from client {} bytes: {:?} repr: {}",
        bytes_read,
        &buf[..bytes_read],
        received_data
    );

    // 7. echo back to the client
    // https://man7.org/linux/man-pages/man2/sendto.2.html
    // we can use write or send
    // send without any flags is just write https://man7.org/linux/man-pages/man2/write.2.html
    //The only difference between send() and write(2) is the presence of
    // flags.  With a zero flags argument, send() is equivalent to write.

    let bytes_written = send(conn_fd, &buf[..bytes_read], MsgFlags::empty())
        .expect("Failted to echo back to the client");
    // let bytes_written =
    //     write(conn_fd, &buf[..bytes_read]).expect("Failted to echo back to the client");

    println!("Written {bytes_written} bytes to the client",);
}
