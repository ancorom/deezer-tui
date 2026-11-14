**tokio > net > udp**

# Module: net::udp

## Contents

**Structs**

- [`UdpSocket`](#udpsocket) - A UDP socket.

---

## tokio::net::udp::UdpSocket

*Struct*

A UDP socket.

UDP is "connectionless", unlike TCP. Meaning, regardless of what address you've bound to, a `UdpSocket`
is free to communicate with many different remotes. In tokio there are basically two main ways to use `UdpSocket`:

* one to many: [`bind`](`UdpSocket::bind`) and use [`send_to`](`UdpSocket::send_to`)
  and [`recv_from`](`UdpSocket::recv_from`) to communicate with many different addresses
* one to one: [`connect`](`UdpSocket::connect`) and associate with a single address, using [`send`](`UdpSocket::send`)
  and [`recv`](`UdpSocket::recv`) to communicate only with that remote address

This type does not provide a `split` method, because this functionality
can be achieved by instead wrapping the socket in an [`Arc`]. Note that
you do not need a `Mutex` to share the `UdpSocket` — an `Arc<UdpSocket>`
is enough. This is because all of the methods take `&self` instead of
`&mut self`. Once you have wrapped it in an `Arc`, you can call
`.clone()` on the `Arc<UdpSocket>` to get multiple shared handles to the
same socket. An example of such usage can be found further down.

[`Arc`]: std::sync::Arc

# Streams

If you need to listen over UDP and produce a [`Stream`], you can look
at [`UdpFramed`].

[`UdpFramed`]: https://docs.rs/tokio-util/latest/tokio_util/udp/struct.UdpFramed.html
[`Stream`]: https://docs.rs/futures/0.3/futures/stream/trait.Stream.html

# Example: one to many (bind)

Using `bind` we can create a simple echo server that sends and recv's with many different clients:
```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        let len = sock.send_to(&buf[..len], addr).await?;
        println!("{:?} bytes sent", len);
    }
}
```

# Example: one to one (connect)

Or using `connect` we can echo with a single remote address using `send` and `recv`:
```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;

    let remote_addr = "127.0.0.1:59611";
    sock.connect(remote_addr).await?;
    let mut buf = [0; 1024];
    loop {
        let len = sock.recv(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, remote_addr);

        let len = sock.send(&buf[..len]).await?;
        println!("{:?} bytes sent", len);
    }
}
```

# Example: Splitting with `Arc`

Because `send_to` and `recv_from` take `&self`. It's perfectly alright
to use an `Arc<UdpSocket>` and share the references to multiple tasks.
Here is a similar "echo" example that supports concurrent
sending/receiving:

```no_run
use tokio::{net::UdpSocket, sync::mpsc};
use std::{io, net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;
    let r = Arc::new(sock);
    let s = r.clone();
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            let len = s.send_to(&bytes, &addr).await.unwrap();
            println!("{:?} bytes sent", len);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        tx.send((buf[..len].to_vec(), addr)).await.unwrap();
    }
}
```


**Methods:**

- `fn bind<A>(addr: A) -> io::Result<UdpSocket>` - This function will create a new UDP socket and attempt to bind it to
- `fn from_std(socket: net::UdpSocket) -> io::Result<UdpSocket>` - Creates new `UdpSocket` from a previously bound `std::net::UdpSocket`.
- `fn into_std(self: Self) -> io::Result<std::net::UdpSocket>` - Turns a [`tokio::net::UdpSocket`] into a [`std::net::UdpSocket`].
- `fn local_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the local address that this socket is bound to.
- `fn peer_addr(self: &Self) -> io::Result<SocketAddr>` - Returns the socket address of the remote peer this socket was connected to.
- `fn connect<A>(self: &Self, addr: A) -> io::Result<()>` - Connects the UDP socket setting the default destination for send() and
- `fn ready(self: &Self, interest: Interest) -> io::Result<Ready>` - Waits for any of the requested ready states.
- `fn writable(self: &Self) -> io::Result<()>` - Waits for the socket to become writable.
- `fn poll_send_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for write/send readiness.
- `fn send(self: &Self, buf: &[u8]) -> io::Result<usize>` - Sends data on the socket to the remote address that the socket is
- `fn poll_send(self: &Self, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>` - Attempts to send data on the socket to the remote address to which it
- `fn try_send(self: &Self, buf: &[u8]) -> io::Result<usize>` - Tries to send data on the socket to the remote address to which it is
- `fn readable(self: &Self) -> io::Result<()>` - Waits for the socket to become readable.
- `fn poll_recv_ready(self: &Self, cx: & mut Context) -> Poll<io::Result<()>>` - Polls for read/receive readiness.
- `fn recv(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Receives a single datagram message on the socket from the remote address
- `fn poll_recv(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>` - Attempts to receive a single datagram message on the socket from the remote
- `fn try_recv(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Tries to receive a single datagram message on the socket from the remote
- `fn try_recv_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Tries to receive data from the stream into the provided buffer, advancing the
- `fn recv_buf<B>(self: &Self, buf: & mut B) -> io::Result<usize>` - Receives a single datagram message on the socket from the remote address
- `fn try_recv_buf_from<B>(self: &Self, buf: & mut B) -> io::Result<(usize, SocketAddr)>` - Tries to receive a single datagram message on the socket. On success,
- `fn recv_buf_from<B>(self: &Self, buf: & mut B) -> io::Result<(usize, SocketAddr)>` - Receives a single datagram message on the socket, advancing the
- `fn send_to<A>(self: &Self, buf: &[u8], addr: A) -> io::Result<usize>` - Sends data on the socket to the given address. On success, returns the
- `fn poll_send_to(self: &Self, cx: & mut Context, buf: &[u8], target: SocketAddr) -> Poll<io::Result<usize>>` - Attempts to send data on the socket to a given address.
- `fn try_send_to(self: &Self, buf: &[u8], target: SocketAddr) -> io::Result<usize>` - Tries to send data on the socket to the given address, but if the send is
- `fn recv_from(self: &Self, buf: & mut [u8]) -> io::Result<(usize, SocketAddr)>` - Receives a single datagram message on the socket. On success, returns
- `fn poll_recv_from(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<SocketAddr>>` - Attempts to receive a single datagram on the socket.
- `fn try_recv_from(self: &Self, buf: & mut [u8]) -> io::Result<(usize, SocketAddr)>` - Tries to receive a single datagram message on the socket. On success,
- `fn try_io<R, impl FnOnce() -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Tries to read or write from the socket using a user-provided IO operation.
- `fn async_io<R, impl FnMut() -> io::Result<R>>(self: &Self, interest: Interest, f: impl Trait) -> io::Result<R>` - Reads or writes from the socket using a user-provided IO operation.
- `fn peek(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Receives a single datagram from the connected address without removing it from the queue.
- `fn poll_peek(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>` - Receives data from the connected address, without removing it from the input queue.
- `fn try_peek(self: &Self, buf: & mut [u8]) -> io::Result<usize>` - Tries to receive data on the connected address without removing it from the input queue.
- `fn peek_from(self: &Self, buf: & mut [u8]) -> io::Result<(usize, SocketAddr)>` - Receives data from the socket, without removing it from the input queue.
- `fn poll_peek_from(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<SocketAddr>>` - Receives data from the socket, without removing it from the input queue.
- `fn try_peek_from(self: &Self, buf: & mut [u8]) -> io::Result<(usize, SocketAddr)>` - Tries to receive data on the socket without removing it from the input queue.
- `fn peek_sender(self: &Self) -> io::Result<SocketAddr>` - Retrieve the sender of the data at the head of the input queue, waiting if empty.
- `fn poll_peek_sender(self: &Self, cx: & mut Context) -> Poll<io::Result<SocketAddr>>` - Retrieve the sender of the data at the head of the input queue,
- `fn try_peek_sender(self: &Self) -> io::Result<SocketAddr>` - Try to retrieve the sender of the data at the head of the input queue.
- `fn broadcast(self: &Self) -> io::Result<bool>` - Gets the value of the `SO_BROADCAST` option for this socket.
- `fn set_broadcast(self: &Self, on: bool) -> io::Result<()>` - Sets the value of the `SO_BROADCAST` option for this socket.
- `fn multicast_loop_v4(self: &Self) -> io::Result<bool>` - Gets the value of the `IP_MULTICAST_LOOP` option for this socket.
- `fn set_multicast_loop_v4(self: &Self, on: bool) -> io::Result<()>` - Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
- `fn multicast_ttl_v4(self: &Self) -> io::Result<u32>` - Gets the value of the `IP_MULTICAST_TTL` option for this socket.
- `fn set_multicast_ttl_v4(self: &Self, ttl: u32) -> io::Result<()>` - Sets the value of the `IP_MULTICAST_TTL` option for this socket.
- `fn multicast_loop_v6(self: &Self) -> io::Result<bool>` - Gets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
- `fn set_multicast_loop_v6(self: &Self, on: bool) -> io::Result<()>` - Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
- `fn tclass_v6(self: &Self) -> io::Result<u32>` - Gets the value of the `IPV6_TCLASS` option for this socket.
- `fn set_tclass_v6(self: &Self, tclass: u32) -> io::Result<()>` - Sets the value for the `IPV6_TCLASS` option on this socket.
- `fn ttl(self: &Self) -> io::Result<u32>` - Gets the value of the `IP_TTL` option for this socket.
- `fn set_ttl(self: &Self, ttl: u32) -> io::Result<()>` - Sets the value for the `IP_TTL` option on this socket.
- `fn tos_v4(self: &Self) -> io::Result<u32>` - Gets the value of the `IP_TOS` option for this socket.
- `fn set_tos_v4(self: &Self, tos: u32) -> io::Result<()>` - Sets the value for the `IP_TOS` option on this socket.
- `fn device(self: &Self) -> io::Result<Option<Vec<u8>>>` - Gets the value for the `SO_BINDTODEVICE` option on this socket
- `fn bind_device(self: &Self, interface: Option<&[u8]>) -> io::Result<()>` - Sets the value for the `SO_BINDTODEVICE` option on this socket
- `fn join_multicast_v4(self: &Self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()>` - Executes an operation of the `IP_ADD_MEMBERSHIP` type.
- `fn join_multicast_v6(self: &Self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()>` - Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
- `fn leave_multicast_v4(self: &Self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()>` - Executes an operation of the `IP_DROP_MEMBERSHIP` type.
- `fn leave_multicast_v6(self: &Self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()>` - Executes an operation of the `IPV6_DROP_MEMBERSHIP` type.
- `fn take_error(self: &Self) -> io::Result<Option<io::Error>>` - Returns the value of the `SO_ERROR` option.

**Trait Implementations:**

- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`
- **TryFrom**
  - `fn try_from(stream: std::net::UdpSocket) -> Result<Self, <Self as >::Error>` - Consumes stream, returning the tokio I/O object.
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



