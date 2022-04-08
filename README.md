> sample of base64 zip encode

```rust
use std::io::Bytes;
use std::io::prelude::{Read, Write};

use base64::{decode, encode};
use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::ZlibEncoder;

fn main() {
    let a = b"hello world";
    let b = "aGVsbG8gd29ybGQ=";

    assert_eq!(encode(a), b);
    assert_eq!(a, &decode(b).unwrap()[..]);


    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"foo");
    e.write_all(b"bar");
    let compressed_bytes = e.finish();
    let zip = compressed_bytes.unwrap();
    let out = &encode(&zip);
    dbg!(out);


    let mut d = ZlibDecoder::new(&zip[..]);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    dbg!(s);
}
```