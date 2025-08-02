use base64::Engine;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::prelude::*;

fn deflate_with_encoder<W: Write + AsRef<[u8]>>(
    data: String,
    mut encoder: DeflateEncoder<W>,
) -> Option<String> {
    // Create a DeflateEncoder

    // Write the JSON string into the encoder
    if encoder.write_all(data.as_bytes()).is_err() {
        return None;
    }

    // Finish the encoding process
    let encoded_bytes = match encoder.finish() {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };

    // Convert the byte array to base64
    Some(base64::engine::general_purpose::STANDARD.encode(encoded_bytes))
}

pub fn deflate(data: String) -> Option<String> {
    deflate_with_encoder(
        data,
        DeflateEncoder::new(Vec::new(), Compression::default()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use flate2::read::DeflateDecoder;
    use std::io::{self, Read, Write};

    #[test]
    fn deflate_compresses_data() {
        let input = r#"{"key":"value"}"#.to_string();
        let encoded = deflate(input.clone()).expect("compression should succeed");

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .expect("base64 decode should succeed");

        let mut decoder = DeflateDecoder::new(&decoded[..]);
        let mut output = String::new();
        decoder
            .read_to_string(&mut output)
            .expect("decompression should succeed");

        assert_eq!(output, input);
    }

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl AsRef<[u8]> for FailingWriter {
        fn as_ref(&self) -> &[u8] {
            &[]
        }
    }

    #[test]
    fn deflate_returns_none_on_error() {
        let encoder = DeflateEncoder::new(FailingWriter, Compression::default());
        let result = deflate_with_encoder("data".into(), encoder);
        assert!(result.is_none());
    }
}
