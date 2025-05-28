use std::{fmt, io::Write};



struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

// 实现Debug trait 
impl fmt::Debug for BufBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"hello world").unwrap();
    println!("{:?}", buf);
}