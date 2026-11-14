**tokio > net > udp > UdpSocket**

# Module: net::udp::UdpSocket

## Contents

**Functions**

- [`bind`](#bind) - This function will create a new UDP socket and attempt to bind it to
- [`connect`](#connect) - Connects the UDP socket setting the default destination for send() and
- [`from_std`](#from_std) - Creates new `UdpSocket` from a previously bound `std::net::UdpSocket`.
- [`join_multicast_v4`](#join_multicast_v4) - Executes an operation of the `IP_ADD_MEMBERSHIP` type.
- [`join_multicast_v6`](#join_multicast_v6) - Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.
- [`peek_from`](#peek_from) - Receives data from the socket, without removing it from the input queue.
- [`peek_sender`](#peek_sender) - Retrieve the sender of the data at the head of the input queue, waiting if empty.
- [`poll_peek_from`](#poll_peek_from) - Receives data from the socket, without removing it from the input queue.
- [`poll_peek_sender`](#poll_peek_sender) - Retrieve the sender of the data at the head of the input queue,
- [`readable`](#readable) - Waits for the socket to become readable.
- [`ready`](#ready) - Waits for any of the requested ready states.
- [`recv`](#recv) - Receives a single datagram message on the socket from the remote address
- [`recv_from`](#recv_from) - Receives a single datagram message on the socket. On success, returns
- [`send`](#send) - Sends data on the socket to the remote address that the socket is
- [`send_to`](#send_to) - Sends data on the socket to the given address. On success, returns the
- [`set_broadcast`](#set_broadcast) - Sets the value of the `SO_BROADCAST` option for this socket.
- [`set_multicast_loop_v4`](#set_multicast_loop_v4) - Sets the value of the `IP_MULTICAST_LOOP` option for this socket.
- [`set_multicast_loop_v6`](#set_multicast_loop_v6) - Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.
- [`set_multicast_ttl_v4`](#set_multicast_ttl_v4) - Sets the value of the `IP_MULTICAST_TTL` option for this socket.
- [`set_tclass_v6`](#set_tclass_v6) - Sets the value for the `IPV6_TCLASS` option on this socket.
- [`set_tos_v4`](#set_tos_v4) - Sets the value for the `IP_TOS` option on this socket.
- [`set_ttl`](#set_ttl) - Sets the value for the `IP_TTL` option on this socket.
- [`try_peek_sender`](#try_peek_sender) - Try to retrieve the sender of the data at the head of the input queue.
- [`writable`](#writable) - Waits for the socket to become writable.

---

## tokio::net::udp::UdpSocket::bind

*Function*

This function will create a new UDP socket and attempt to bind it to
the `addr` provided.

Binding with a port number of 0 will request that the OS assigns a port
to this listener. The port allocated can be queried via the `local_addr`
method.

# Example

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
#   if cfg!(miri) { return Ok(()); } // No UDP sockets in miri.
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    // use `sock`
#   let _ = sock;
    Ok(())
}
```

```rust
fn bind<A>(addr: A) -> io::Result<UdpSocket>
```



## tokio::net::udp::UdpSocket::connect

*Function*

Connects the UDP socket setting the default destination for send() and
limiting packets that are read via `recv` from the address specified in
`addr`.

# Example

```no_run
use tokio::net::UdpSocket;
# use std::{io, net::SocketAddr};

# #[tokio::main]
# async fn main() -> io::Result<()> {
let sock = UdpSocket::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).await?;

let remote_addr = "127.0.0.1:59600".parse::<SocketAddr>().unwrap();
sock.connect(remote_addr).await?;
let mut buf = [0u8; 32];
// recv from remote_addr
let len = sock.recv(&mut buf).await?;
// send to remote_addr
let _len = sock.send(&buf[..len]).await?;
# Ok(())
# }
```

```rust
fn connect<A>(self: &Self, addr: A) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::from_std

*Function*

Creates new `UdpSocket` from a previously bound `std::net::UdpSocket`.

This function is intended to be used to wrap a UDP socket from the
standard library in the Tokio equivalent.

This can be used in conjunction with `socket2`'s `Socket` interface to
configure a socket before it's handed off, such as setting options like
`reuse_address` or binding to multiple addresses.

# Notes

The caller is responsible for ensuring that the socket is in
non-blocking mode. Otherwise all I/O operations on the socket
will block the thread, which will cause unexpected behavior.
Non-blocking mode can be set using [`set_nonblocking`].

Passing a listener in blocking mode is always erroneous,
and the behavior in that case may change in the future.
For example, it could panic.

[`set_nonblocking`]: std::net::UdpSocket::set_nonblocking

# Panics

This function panics if thread-local runtime is not set.

The runtime is usually set implicitly when this function is called
from a future driven by a tokio runtime, otherwise runtime can be set
explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.

# Example

```no_run
use tokio::net::UdpSocket;
# use std::{io, net::SocketAddr};

# #[tokio::main]
# async fn main() -> io::Result<()> {
let addr = "0.0.0.0:8080".parse::<SocketAddr>().unwrap();
let std_sock = std::net::UdpSocket::bind(addr)?;
std_sock.set_nonblocking(true)?;
let sock = UdpSocket::from_std(std_sock)?;
// use `sock`
# Ok(())
# }
```

```rust
fn from_std(socket: net::UdpSocket) -> io::Result<UdpSocket>
```



## tokio::net::udp::UdpSocket::join_multicast_v4

*Function*

Executes an operation of the `IP_ADD_MEMBERSHIP` type.

This function specifies a new multicast group for this socket to join.
The address must be a valid multicast address, and `interface` is the
address of the local interface with which the system should join the
multicast group. If it's equal to `INADDR_ANY` then an appropriate
interface is chosen by the system.

```rust
fn join_multicast_v4(self: &Self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::join_multicast_v6

*Function*

Executes an operation of the `IPV6_ADD_MEMBERSHIP` type.

This function specifies a new multicast group for this socket to join.
The address must be a valid multicast address, and `interface` is the
index of the interface to join/leave (or 0 to indicate any interface).

```rust
fn join_multicast_v6(self: &Self, multiaddr: &Ipv6Addr, interface: u32) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::peek_from

*Function*

Receives data from the socket, without removing it from the input queue.
On success, returns the number of bytes read and the address from whence
the data came.

# Notes

On Windows, if the data is larger than the buffer specified, the buffer
is filled with the first part of the data, and `peek_from` returns the error
`WSAEMSGSIZE(10040)`. The excess data is lost.
Make sure to always use a sufficiently large buffer to hold the
maximum UDP packet size, which can be up to 65536 bytes in size.

MacOS will return an error if you pass a zero-sized buffer.

If you're merely interested in learning the sender of the data at the head of the queue,
try [`peek_sender`].

Note that the socket address **cannot** be implicitly trusted, because it is relatively
trivial to send a UDP datagram with a spoofed origin in a [packet injection attack].
Because UDP is stateless and does not validate the origin of a packet,
the attacker does not need to be able to intercept traffic in order to interfere.
It is important to be aware of this when designing your application-level protocol.

# Examples

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;

    let mut buf = vec![0u8; 32];
    let (len, addr) = socket.peek_from(&mut buf).await?;

    println!("peeked {:?} bytes from {:?}", len, addr);

    Ok(())
}
```

[`peek_sender`]: method@Self::peek_sender
[packet injection attack]: https://en.wikipedia.org/wiki/Packet_injection

```rust
fn peek_from(self: &Self, buf: & mut [u8]) -> io::Result<(usize, SocketAddr)>
```



## tokio::net::udp::UdpSocket::peek_sender

*Function*

Retrieve the sender of the data at the head of the input queue, waiting if empty.

This is equivalent to calling [`peek_from`] with a zero-sized buffer,
but suppresses the `WSAEMSGSIZE` error on Windows and the "invalid argument" error on macOS.

Note that the socket address **cannot** be implicitly trusted, because it is relatively
trivial to send a UDP datagram with a spoofed origin in a [packet injection attack].
Because UDP is stateless and does not validate the origin of a packet,
the attacker does not need to be able to intercept traffic in order to interfere.
It is important to be aware of this when designing your application-level protocol.

[`peek_from`]: method@Self::peek_from
[packet injection attack]: https://en.wikipedia.org/wiki/Packet_injection

```rust
fn peek_sender(self: &Self) -> io::Result<SocketAddr>
```



## tokio::net::udp::UdpSocket::poll_peek_from

*Function*

Receives data from the socket, without removing it from the input queue.
On success, returns the sending address of the datagram.

# Notes

Note that on multiple calls to a `poll_*` method in the `recv` direction, only the
`Waker` from the `Context` passed to the most recent call will be scheduled to
receive a wakeup

On Windows, if the data is larger than the buffer specified, the buffer
is filled with the first part of the data, and peek returns the error
`WSAEMSGSIZE(10040)`. The excess data is lost.
Make sure to always use a sufficiently large buffer to hold the
maximum UDP packet size, which can be up to 65536 bytes in size.

MacOS will return an error if you pass a zero-sized buffer.

If you're merely interested in learning the sender of the data at the head of the queue,
try [`poll_peek_sender`].

Note that the socket address **cannot** be implicitly trusted, because it is relatively
trivial to send a UDP datagram with a spoofed origin in a [packet injection attack].
Because UDP is stateless and does not validate the origin of a packet,
the attacker does not need to be able to intercept traffic in order to interfere.
It is important to be aware of this when designing your application-level protocol.

# Return value

The function returns:

* `Poll::Pending` if the socket is not ready to read
* `Poll::Ready(Ok(addr))` reads data from `addr` into `ReadBuf` if the socket is ready
* `Poll::Ready(Err(e))` if an error is encountered.

# Errors

This function may encounter any standard I/O error except `WouldBlock`.

[`poll_peek_sender`]: method@Self::poll_peek_sender
[packet injection attack]: https://en.wikipedia.org/wiki/Packet_injection

```rust
fn poll_peek_from(self: &Self, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<SocketAddr>>
```



## tokio::net::udp::UdpSocket::poll_peek_sender

*Function*

Retrieve the sender of the data at the head of the input queue,
scheduling a wakeup if empty.

This is equivalent to calling [`poll_peek_from`] with a zero-sized buffer,
but suppresses the `WSAEMSGSIZE` error on Windows and the "invalid argument" error on macOS.

# Notes

Note that on multiple calls to a `poll_*` method in the `recv` direction, only the
`Waker` from the `Context` passed to the most recent call will be scheduled to
receive a wakeup.

Note that the socket address **cannot** be implicitly trusted, because it is relatively
trivial to send a UDP datagram with a spoofed origin in a [packet injection attack].
Because UDP is stateless and does not validate the origin of a packet,
the attacker does not need to be able to intercept traffic in order to interfere.
It is important to be aware of this when designing your application-level protocol.

[`poll_peek_from`]: method@Self::poll_peek_from
[packet injection attack]: https://en.wikipedia.org/wiki/Packet_injection

```rust
fn poll_peek_sender(self: &Self, cx: & mut Context) -> Poll<io::Result<SocketAddr>>
```



## tokio::net::udp::UdpSocket::readable

*Function*

Waits for the socket to become readable.

This function is equivalent to `ready(Interest::READABLE)` and is usually
paired with `try_recv()`.

The function may complete without the socket being readable. This is a
false-positive and attempting a `try_recv()` will return with
`io::ErrorKind::WouldBlock`.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read that fails with `WouldBlock` or
`Poll::Pending`.

# Examples

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to a peer
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    socket.connect("127.0.0.1:8081").await?;

    loop {
        // Wait for the socket to be readable
        socket.readable().await?;

        // The buffer is **not** included in the async task and will
        // only exist on the stack.
        let mut buf = [0; 1024];

        // Try to recv data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match socket.try_recv(&mut buf) {
            Ok(n) => {
                println!("GOT {:?}", &buf[..n]);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
```

```rust
fn readable(self: &Self) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::ready

*Function*

Waits for any of the requested ready states.

This function is usually paired with `try_recv()` or `try_send()`. It
can be used to concurrently `recv` / `send` to the same socket on a single
task without splitting the socket.

The function may complete without the socket being ready. This is a
false-positive and attempting an operation will return with
`io::ErrorKind::WouldBlock`. The function can also return with an empty
[`Ready`] set, so you should always check the returned value and possibly
wait again if the requested states are not set.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to read or write that fails with `WouldBlock` or
`Poll::Pending`.

# Examples

Concurrently receive from and send to the socket on the same task
without splitting.

```no_run
use tokio::io::{self, Interest};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    socket.connect("127.0.0.1:8081").await?;

    loop {
        let ready = socket.ready(Interest::READABLE | Interest::WRITABLE).await?;

        if ready.is_readable() {
            // The buffer is **not** included in the async task and will only exist
            // on the stack.
            let mut data = [0; 1024];
            match socket.try_recv(&mut data[..]) {
                Ok(n) => {
                    println!("received {:?}", &data[..n]);
                }
                // False-positive, continue
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if ready.is_writable() {
            // Write some data
            match socket.try_send(b"hello world") {
                Ok(n) => {
                    println!("sent {} bytes", n);
                }
                // False-positive, continue
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
```

```rust
fn ready(self: &Self, interest: Interest) -> io::Result<Ready>
```



## tokio::net::udp::UdpSocket::recv

*Function*

Receives a single datagram message on the socket from the remote address
to which it is connected. On success, returns the number of bytes read.

The function must be called with valid byte array `buf` of sufficient
size to hold the message bytes. If a message is too long to fit in the
supplied buffer, excess bytes may be discarded.

The [`connect`] method will connect this socket to a remote address.
This method will fail if the socket is not connected.

# Cancel safety

This method is cancel safe. If `recv` is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, it is guaranteed that no messages were received on this
socket.

[`connect`]: method@Self::connect

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind socket
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    socket.connect("127.0.0.1:8081").await?;

    let mut buf = vec![0; 10];
    let n = socket.recv(&mut buf).await?;

    println!("received {} bytes {:?}", n, &buf[..n]);

    Ok(())
}
```

```rust
fn recv(self: &Self, buf: & mut [u8]) -> io::Result<usize>
```



## tokio::net::udp::UdpSocket::recv_from

*Function*

Receives a single datagram message on the socket. On success, returns
the number of bytes read and the origin.

The function must be called with valid byte array `buf` of sufficient
size to hold the message bytes. If a message is too long to fit in the
supplied buffer, excess bytes may be discarded.

# Cancel safety

This method is cancel safe. If `recv_from` is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, it is guaranteed that no messages were received on this
socket.

# Example

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;

    let mut buf = vec![0u8; 32];
    let (len, addr) = socket.recv_from(&mut buf).await?;

    println!("received {:?} bytes from {:?}", len, addr);

    Ok(())
}
```

# Notes
Note that the socket address **cannot** be implicitly trusted, because it is relatively
trivial to send a UDP datagram with a spoofed origin in a [packet injection attack].
Because UDP is stateless and does not validate the origin of a packet,
the attacker does not need to be able to intercept traffic in order to interfere.
It is important to be aware of this when designing your application-level protocol.

[packet injection attack]: https://en.wikipedia.org/wiki/Packet_injection

```rust
fn recv_from(self: &Self, buf: & mut [u8]) -> io::Result<(usize, SocketAddr)>
```



## tokio::net::udp::UdpSocket::send

*Function*

Sends data on the socket to the remote address that the socket is
connected to.

The [`connect`] method will connect this socket to a remote address.
This method will fail if the socket is not connected.

This method may fail with a [`ConnectionRefused`] error if the remote
address has replied with ICMP Unreachable to a previously sent packet.
However, this behavior depends on the OS.

[`connect`]: method@Self::connect
[`ConnectionRefused`]: std::io::ErrorKind::ConnectionRefused

# Return

On success, the number of bytes sent is returned, otherwise, the
encountered error is returned.

# Cancel safety

This method is cancel safe. If `send` is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that the message was not sent.

# Examples

```no_run
use tokio::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind socket
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    socket.connect("127.0.0.1:8081").await?;

    // Send a message
    socket.send(b"hello world").await?;

    Ok(())
}
```

```rust
fn send(self: &Self, buf: &[u8]) -> io::Result<usize>
```



## tokio::net::udp::UdpSocket::send_to

*Function*

Sends data on the socket to the given address. On success, returns the
number of bytes written.

Address type can be any implementer of [`ToSocketAddrs`] trait. See its
documentation for concrete examples.

It is possible for `addr` to yield multiple addresses, but `send_to`
will only send data to the first address yielded by `addr`.

This will return an error when the IP version of the local socket does
not match that returned from [`ToSocketAddrs`].

[`ToSocketAddrs`]: crate::net::ToSocketAddrs

# Cancel safety

This method is cancel safe. If `send_to` is used as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that the message was not sent.

# Example

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let len = socket.send_to(b"hello world", "127.0.0.1:8081").await?;

    println!("Sent {} bytes", len);

    Ok(())
}
```

```rust
fn send_to<A>(self: &Self, buf: &[u8], addr: A) -> io::Result<usize>
```



## tokio::net::udp::UdpSocket::set_broadcast

*Function*

Sets the value of the `SO_BROADCAST` option for this socket.

When enabled, this socket is allowed to send packets to a broadcast
address.

```rust
fn set_broadcast(self: &Self, on: bool) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::set_multicast_loop_v4

*Function*

Sets the value of the `IP_MULTICAST_LOOP` option for this socket.

If enabled, multicast packets will be looped back to the local socket.

# Note

This may not have any effect on IPv6 sockets.

```rust
fn set_multicast_loop_v4(self: &Self, on: bool) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::set_multicast_loop_v6

*Function*

Sets the value of the `IPV6_MULTICAST_LOOP` option for this socket.

Controls whether this socket sees the multicast packets it sends itself.

# Note

This may not have any effect on IPv4 sockets.

```rust
fn set_multicast_loop_v6(self: &Self, on: bool) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::set_multicast_ttl_v4

*Function*

Sets the value of the `IP_MULTICAST_TTL` option for this socket.

Indicates the time-to-live value of outgoing multicast packets for
this socket. The default value is 1 which means that multicast packets
don't leave the local network unless explicitly requested.

# Note

This may not have any effect on IPv6 sockets.

```rust
fn set_multicast_ttl_v4(self: &Self, ttl: u32) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::set_tclass_v6

*Function*

Sets the value for the `IPV6_TCLASS` option on this socket.

Specifies the traffic class field that is used in every packet
sent from this socket.

# Note

This may not have any effect on IPv4 sockets.

```rust
fn set_tclass_v6(self: &Self, tclass: u32) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::set_tos_v4

*Function*

Sets the value for the `IP_TOS` option on this socket.

This value sets the type-of-service field that is used in every packet
sent from this socket.

# Note

- This may not have any effect on IPv6 sockets.
- On Windows, `IP_TOS` is only supported on [Windows 8+ or
  Windows Server 2012+.](https://docs.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options)

```rust
fn set_tos_v4(self: &Self, tos: u32) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::set_ttl

*Function*

Sets the value for the `IP_TTL` option on this socket.

This value sets the time-to-live field that is used in every packet sent
from this socket.

# Examples

```no_run
use tokio::net::UdpSocket;
# use std::io;

# async fn dox() -> io::Result<()> {
let sock = UdpSocket::bind("127.0.0.1:8080").await?;
sock.set_ttl(60)?;

# Ok(())
# }
```

```rust
fn set_ttl(self: &Self, ttl: u32) -> io::Result<()>
```



## tokio::net::udp::UdpSocket::try_peek_sender

*Function*

Try to retrieve the sender of the data at the head of the input queue.

When there is no pending data, `Err(io::ErrorKind::WouldBlock)` is
returned. This function is usually paired with `readable()`.

Note that the socket address **cannot** be implicitly trusted, because it is relatively
trivial to send a UDP datagram with a spoofed origin in a [packet injection attack].
Because UDP is stateless and does not validate the origin of a packet,
the attacker does not need to be able to intercept traffic in order to interfere.
It is important to be aware of this when designing your application-level protocol.

[packet injection attack]: https://en.wikipedia.org/wiki/Packet_injection

```rust
fn try_peek_sender(self: &Self) -> io::Result<SocketAddr>
```



## tokio::net::udp::UdpSocket::writable

*Function*

Waits for the socket to become writable.

This function is equivalent to `ready(Interest::WRITABLE)` and is
usually paired with `try_send()` or `try_send_to()`.

The function may complete without the socket being writable. This is a
false-positive and attempting a `try_send()` will return with
`io::ErrorKind::WouldBlock`.

# Cancel safety

This method is cancel safe. Once a readiness event occurs, the method
will continue to return immediately until the readiness event is
consumed by an attempt to write that fails with `WouldBlock` or
`Poll::Pending`.

# Examples

```no_run
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Bind socket
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    socket.connect("127.0.0.1:8081").await?;

    loop {
        // Wait for the socket to be writable
        socket.writable().await?;

        // Try to send data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match socket.try_send(b"hello world") {
            Ok(n) => {
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}
```

```rust
fn writable(self: &Self) -> io::Result<()>
```



