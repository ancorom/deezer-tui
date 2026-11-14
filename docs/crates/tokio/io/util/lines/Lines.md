**tokio > io > util > lines > Lines**

# Module: io::util::lines::Lines

## Contents

**Functions**

- [`next_line`](#next_line) - Returns the next line in the stream.

---

## tokio::io::util::lines::Lines::next_line

*Function*

Returns the next line in the stream.

# Cancel safety

This method is cancel safe.

# Examples

```
# use tokio::io::AsyncBufRead;
use tokio::io::AsyncBufReadExt;

# async fn dox(my_buf_read: impl AsyncBufRead + Unpin) -> std::io::Result<()> {
let mut lines = my_buf_read.lines();

while let Some(line) = lines.next_line().await? {
    println!("length = {}", line.len())
}
# Ok(())
# }
```

```rust
fn next_line(self: & mut Self) -> io::Result<Option<String>>
```



