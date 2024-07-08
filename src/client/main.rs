use nix::{
    sys::socket::{
        connect, recv, send, socket, AddressFamily, MsgFlags, SockFlag, SockType, SockaddrIn,
    },
    unistd::close,
};
use std::{os::fd::AsRawFd, str::FromStr};

/// This is the client
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

    // 3. Connect to the server
    // https://man7.org/linux/man-pages/man2/connect.2.html
    connect(socket_fd.as_raw_fd(), &sock_addr).expect("Failed to connect to the server");

    // 4. Send message to the server
    // https://man7.org/linux/man-pages/man2/sendto.2.html
    // we can use write or send
    // send without any flags is just write https://man7.org/linux/man-pages/man2/write.2.html
    //The only difference between send() and write(2) is the presence of
    // flags.  With a zero flags argument, send() is equivalent to write.
    let bytes_written = send(
        socket_fd.as_raw_fd(),
        "hello from rust client".as_bytes(),
        MsgFlags::empty(),
    )
    .expect("Failted to send msg to the server");
    // let bytes_written =
    //     write(conn_fd, &buf[..bytes_read]).expect("Failted to echo back to the client");
    println!("Written {bytes_written} bytes to the server",);

    // 5. Read data sent from the server
    let mut buf = [0u8; 1024];
    // https://man7.org/linux/man-pages/man2/recv.2.html
    // we can use read or recv
    // recv without any flags is just read https://man7.org/linux/man-pages/man2/read.2.html
    // The only difference between recv() and read(2) is the presence of
    // flags.  With a zero flags argument, recv() is generally
    // equivalent to read(2) (but see NOTES).  Also, the following call
    let bytes_read = recv(socket_fd.as_raw_fd(), &mut buf, MsgFlags::empty())
        .expect("Failed to read from connection");
    // let bytes_read = read(conn_fd, &mut buf).expect("Failed to read from connection");
    let received_data =
        std::str::from_utf8(&buf[..bytes_read]).expect("Failed to convert received data to string");
    println!(
        "Received from server {} bytes: {:?} repr: {}",
        bytes_read,
        &buf[..bytes_read],
        received_data
    );

    // we don't actully need to close the those file descriptors manually
    // rust will do this for us autoamtically
    let _ = close(socket_fd.as_raw_fd()).expect("Failed to close the socket");
}
