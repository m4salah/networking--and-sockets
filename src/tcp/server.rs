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
    let bytes_read = recv(conn_fd, &mut size_buf, MsgFlags::empty())?;
    let file_size = u32::from_ne_bytes(size_buf);
    println!("File size: {}", file_size);
    println!("Bytes Read: {}", bytes_read);

    // I encounter some weird issue this recv sometimes doesn't receive the full sended data
    // sometimes it does sometimes doesn't this was so annoying to me so i asked chatGPT
    // ChatGPT: The issue you're encountering is due to the nature of TCP socket communication.
    // TCP is a stream-oriented protocol, which means that data is sent as a continuous stream of bytes.
    // When you call recv, you may not receive all the data in one call.
    // Instead, you might receive a partial message,
    // or your call to recv might return before all data has arrived.
    //
    // Here's an explanation of what might be happening in your code:
    // 1. Partial Reads: Your server calls recv twice, once to receive the file size and once to receive the file data.
    // However, there's no guarantee that the second recv call will read all 446 bytes in one go.
    // This is why sometimes you only see a partial read of 4 bytes.
    // 2. Data Arrival Timing: Depending on how the OS schedules your server and client processes,
    // and how the TCP stack handles the data,
    // the data might not be fully available when the second recv call is made.
    //
    // Receive the file data
    let mut file_buf = vec![0; file_size as usize];
    let mut total_read = 0;

    while total_read < file_size as usize {
        let bytes_read = recv(conn_fd, &mut file_buf[total_read..], MsgFlags::empty())?;
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read;
    }

    println!("File data bytes read: {}", total_read);
    println!("File data: {}", String::from_utf8_lossy(&file_buf));
    Ok(())
}
