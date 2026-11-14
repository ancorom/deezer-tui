**image > io > image_reader_type > ImageReader**

# Module: io::image_reader_type::ImageReader

## Contents

**Functions**

- [`open`](#open) - Open a file to read, format will be guessed from path.

---

## image::io::image_reader_type::ImageReader::open

*Function*

Open a file to read, format will be guessed from path.

This will not attempt any io operation on the opened file.

If you want to inspect the content for a better guess on the format, which does not depend
on file extensions, follow this call with a call to [`with_guessed_format`].

[`with_guessed_format`]: #method.with_guessed_format

```rust
fn open<P>(path: P) -> io::Result<Self>
```



