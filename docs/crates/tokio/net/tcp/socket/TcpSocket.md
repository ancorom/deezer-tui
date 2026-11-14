**tokio > net > tcp > socket > TcpSocket**

# Module: net::tcp::socket::TcpSocket

## Contents

**Functions**

- [`set_linger`](#set_linger) - Sets the linger duration of this socket by setting the `SO_LINGER` option.
- [`set_nodelay`](#set_nodelay) - Sets the value of the `TCP_NODELAY` option on this socket.
- [`set_tclass_v6`](#set_tclass_v6) - Sets the value for the `IPV6_TCLASS` option on this socket.
- [`set_tos_v4`](#set_tos_v4) - Sets the value for the `IP_TOS` option on this socket.
- [`set_zero_linger`](#set_zero_linger) - Sets a linger duration of zero on this socket by setting the `SO_LINGER` option.

---

## tokio::net::tcp::socket::TcpSocket::set_linger

*Function*

Sets the linger duration of this socket by setting the `SO_LINGER` option.

This option controls the action taken when a stream has unsent messages and the stream is
closed. If `SO_LINGER` is set, the system shall block the process until it can transmit the
data or until the time expires.

If `SO_LINGER` is not specified, and the socket is closed, the system handles the call in a
way that allows the process to continue as quickly as possible.

This option is deprecated because setting `SO_LINGER` on a socket used with Tokio is always
incorrect as it leads to blocking the thread when the socket is closed. For more details,
please see:

> Volumes of communications have been devoted to the intricacies of `SO_LINGER` versus
> non-blocking (`O_NONBLOCK`) sockets. From what I can tell, the final word is: don't do
> it. Rely on the `shutdown()`-followed-by-`read()`-eof technique instead.
>
> From [The ultimate `SO_LINGER` page, or: why is my tcp not reliable](https://blog.netherlabs.nl/articles/2009/01/18/the-ultimate-so_linger-page-or-why-is-my-tcp-not-reliable)

Although this method is deprecated, it will not be removed from Tokio.

Note that the special case of setting `SO_LINGER` to zero does not lead to blocking. Tokio
provides [`set_zero_linger`](Self::set_zero_linger) for this purpose.

```rust
fn set_linger(self: &Self, dur: Option<Duration>) -> io::Result<()>
```



## tokio::net::tcp::socket::TcpSocket::set_nodelay

*Function*

Sets the value of the `TCP_NODELAY` option on this socket.

If set, this option disables the Nagle algorithm. This means that segments are always
sent as soon as possible, even if there is only a small amount of data. When not set,
data is buffered until there is a sufficient amount to send out, thereby avoiding
the frequent sending of small packets.

# Examples

```no_run
use tokio::net::TcpSocket;

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let socket = TcpSocket::new_v4()?;

socket.set_nodelay(true)?;
# Ok(())
# }
```

```rust
fn set_nodelay(self: &Self, nodelay: bool) -> io::Result<()>
```



## tokio::net::tcp::socket::TcpSocket::set_tclass_v6

*Function*

Sets the value for the `IPV6_TCLASS` option on this socket.

Specifies the traffic class field that is used in every packet
sent from this socket.

# Note

This may not have any effect on IPv4 sockets.

```rust
fn set_tclass_v6(self: &Self, tclass: u32) -> io::Result<()>
```



## tokio::net::tcp::socket::TcpSocket::set_tos_v4

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



## tokio::net::tcp::socket::TcpSocket::set_zero_linger

*Function*

Sets a linger duration of zero on this socket by setting the `SO_LINGER` option.

This causes the connection to be forcefully aborted ("abortive close") when the socket is
dropped or closed. Instead of the normal TCP shutdown handshake (`FIN`/`ACK`), a TCP `RST`
(reset) segment is sent to the peer, and the socket immediately discards any unsent data
residing in the socket send buffer. This prevents the socket from entering the `TIME_WAIT`
state after closing it.

This is a destructive action. Any data currently buffered by the OS but not yet transmitted
will be lost. The peer will likely receive a "Connection Reset" error rather than a clean
end-of-stream.

See the documentation for [`set_linger`](Self::set_linger) for additional details on how
`SO_LINGER` works.

```rust
fn set_zero_linger(self: &Self) -> io::Result<()>
```



