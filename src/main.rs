use std::io::BufWriter;
use std::io::Write;

struct Rot13Writer<T>
where
    T: Write,
{
    inner: T,
}

impl<T> Rot13Writer<T>
where
    T: Write,
{
    pub fn new(inner: T) -> Self {
        Rot13Writer { inner: inner }
    }
}

impl<T> Write for Rot13Writer<T>
where
    T: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let rotated_buf: Vec<u8> = buf
            .iter()
            .map(|byte| {
                let offset = match *byte {
                    b'A'..=b'Z' => b'A',
                    b'a'..=b'z' => b'a',
                    _ => return *byte,
                };
                return offset + (*byte - offset + 13) % 26;
            })
            .collect();
        self.inner.write(&rotated_buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn main() {
    let mut content = Vec::<u8>::default();

    let mut buff = Rot13Writer::new(&mut content);
    buff.write(b"Lbh penpxrq zl fhcre qvssvphyg pbqvat punyyratr... pbqr vf ddommNst")
        .unwrap();

    println!(
        "result: {:?}",
        content.iter().map(|x| *x as char).collect::<String>()
    );
}

mod tests {
    #[test]
    fn test_rot13() {
        todo!()
    }
}
