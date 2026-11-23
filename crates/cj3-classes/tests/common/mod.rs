//! Reader for the `key = value` vector files under `bench/p007-vectors/`.
//!
//! A line that starts with two spaces continues the value above it. Bytes are
//! lower-case hex, and spaces inside them are not part of the value. Integers are
//! decimal. Lines that start with `#` are comments.

#![allow(
    dead_code,
    reason = "shared by several test binaries, each of which uses a part of it"
)]

use std::collections::BTreeMap;

/// The parsed `key = value` pairs of one vector file.
pub struct Vectors(BTreeMap<String, String>);

impl Vectors {
    /// Parses a vector file. Line endings may be LF or CRLF.
    pub fn parse(text: &str) -> Self {
        let mut values = BTreeMap::<String, String>::new();
        let mut current: Option<String> = None;
        for line in text.lines() {
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(continuation) = line.strip_prefix("  ") {
                let key = current.as_ref().expect("a continuation follows a key");
                let value = values.get_mut(key).expect("the key was inserted");
                value.push(' ');
                value.push_str(continuation.trim());
            } else {
                let (key, value) = line
                    .split_once(" =")
                    .unwrap_or_else(|| panic!("not a `key = value` line: {line}"));
                let previous = values.insert(key.to_owned(), value.trim().to_owned());
                assert!(previous.is_none(), "duplicate key {key}");
                current = Some(key.to_owned());
            }
        }
        Self(values)
    }

    /// The raw text of a value.
    pub fn text(&self, key: &str) -> &str {
        self.0
            .get(key)
            .unwrap_or_else(|| panic!("missing vector key {key}"))
    }

    /// A decimal integer value.
    pub fn int(&self, key: &str) -> u64 {
        self.text(key)
            .parse()
            .unwrap_or_else(|_| panic!("{key} is not a decimal u64"))
    }

    /// A hex value of any length.
    pub fn bytes(&self, key: &str) -> Vec<u8> {
        unhex(self.text(key))
    }

    /// A hex value of exactly `N` bytes.
    pub fn array<const N: usize>(&self, key: &str) -> [u8; N] {
        self.bytes(key)
            .try_into()
            .unwrap_or_else(|_| panic!("{key} is not {N} bytes"))
    }

    /// A list of decimal integers separated by spaces.
    pub fn ints(&self, key: &str) -> Vec<i64> {
        self.text(key)
            .split_whitespace()
            .map(|token| {
                token
                    .parse()
                    .unwrap_or_else(|_| panic!("{key} holds a non-integer {token}"))
            })
            .collect()
    }
}

/// Decodes lower-case hex, ignoring spaces.
pub fn unhex(text: &str) -> Vec<u8> {
    let digits = text
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<Vec<_>>();
    assert!(digits.len().is_multiple_of(2), "odd number of hex digits");
    digits
        .chunks_exact(2)
        .map(|pair| {
            let high = pair[0].to_digit(16).expect("hex digit");
            let low = pair[1].to_digit(16).expect("hex digit");
            assert!(
                !pair[0].is_ascii_uppercase() && !pair[1].is_ascii_uppercase(),
                "hex must be lower-case"
            );
            u8::try_from(high * 16 + low).expect("two hex digits fit a byte")
        })
        .collect()
}
