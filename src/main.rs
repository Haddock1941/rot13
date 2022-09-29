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
        // collect bytes from buf with alphabet ascii characters rotated by 13 places
        let rotated_buf: Vec<u8> = buf
            .iter()
            .map(|byte| {
                // get beginning of alphabet, either upper or lowercase, or return early
                let offset = match *byte {
                    b'A'..=b'Z' => b'A',
                    b'a'..=b'z' => b'a',
                    // not in alphabet, return as is
                    _ => return *byte,
                };
                // rotate byte by 13 places
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

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    // Shamelessly reusing the exercise here
    fn test_rot13() {
        let mut content = Vec::<u8>::default();

        let mut buff = Rot13Writer::new(&mut content);
        buff.write(b"Lbh penpxrq zl fhcre qvssvphyg pbqvat punyyratr... pbqr vf ddommNst")
            .unwrap();

        assert_eq!(
            content.iter().map(|x| *x as char).collect::<String>(),
            "You cracked my super difficult coding challenge... code is qqbzzAfg".to_string()
        );
    }
}
