use std::borrow::Cow;
pub struct FastFormSerializer {
    buffer: String,
}

impl FastFormSerializer {
    pub fn new(pairs: &[(Cow<'_, str>, Cow<'_, str>)]) -> Self {
        let capacity = pairs
            .iter()
            .map(|(k, v)| k.len() * 3 + v.len() * 3 + 2)
            .sum();

        Self {
            buffer: String::with_capacity(capacity),
        }
    }

    pub fn extend_pairs<'a, I>(&mut self, pairs: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a (Cow<'a, str>, Cow<'a, str>)>,
    {
        for (key, value) in pairs {
            if !self.buffer.is_empty() {
                self.buffer.push('&');
            }

            for &b in key.as_bytes() {
                if should_encode(b) {
                    self.buffer.push('%');
                    self.buffer.push_str(&hex(b));
                } else {
                    self.buffer.push(b as char);
                }
            }

            self.buffer.push('=');

            for &b in value.as_bytes() {
                if should_encode(b) {
                    self.buffer.push('%');
                    self.buffer.push_str(&hex(b));
                } else {
                    self.buffer.push(b as char);
                }
            }
        }
        self
    }

    pub fn finish(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }
}

#[inline]
fn should_encode(byte: u8) -> bool {
    !(byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'.' || byte == b'_' || byte == b'~')
}

#[inline]
fn hex(byte: u8) -> String {
    format!("{:02X}", byte)
}
