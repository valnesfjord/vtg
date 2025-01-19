use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::borrow::Cow;
use vtg::structs::struct_to_vec::param;
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
    pub fn new_vec(pairs: &[(&str, &str)]) -> Self {
        let capacity = pairs
            .iter()
            .map(|(k, v)| k.len() * 3 + v.len() * 3 + 2)
            .sum();

        Self {
            buffer: String::with_capacity(capacity),
        }
    }
    pub fn extend_vec_pairs<'a, I>(&mut self, pairs: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a (&'a str, &'a str)>,
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

fn form_serialize_benchmark(c: &mut Criterion) {
    let test_data_vec = vec![("key1", "value1"), ("key2", "value2")];

    let test_data_param = vec![param("key1", "value1"), param("key2", "value2")];
    c.bench_function("vec_growing", |b| {
        b.iter(|| {
            let vec = vec![
                ("dynamic1", "value1"),
                ("dynamic2", "value2"),
                ("dynamic3", "value2"),
                ("dynamic4", "value2"),
            ];
            let mut serializer = FastFormSerializer::new_vec(&vec);
            serializer.extend_vec_pairs(&vec).finish()
        })
    });

    c.bench_function("param_growing", |b| {
        b.iter(|| {
            let vec = vec![
                param("dynamic1", "value1"),
                param("dynamic2", "value2"),
                param("dynamic3", "value2"),
                param("dynamic4", "value2"),
            ];
            let mut serializer = FastFormSerializer::new(&vec);
            serializer.extend_pairs(&vec).finish()
        })
    });
    c.bench_function("vec_direct", |b| {
        b.iter(|| {
            let mut serializer = FastFormSerializer::new_vec(black_box(&test_data_vec));
            serializer.extend_vec_pairs(&test_data_vec).finish()
        })
    });

    c.bench_function("vec_param", |b| {
        b.iter(|| {
            let mut serializer = FastFormSerializer::new(black_box(&test_data_param));
            serializer.extend_pairs(&test_data_param).finish()
        })
    });
}

criterion_group!(benches, form_serialize_benchmark);
criterion_main!(benches);
