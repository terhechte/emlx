use std::io::BufReader;
use std::io::Cursor;

use plist;
pub use plist::Dictionary;

#[cfg(feature = "use-tracing")]
use tracing;

use super::ParseError;

/// Additional flags on `emlx` messages.
///
/// They're retrieved by parsing the `flags` integer
/// in the metadata plist.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Flags {
    pub is_read: bool,
    pub is_deleted: bool,
    pub is_answered: bool,
    pub is_encrypted: bool,
    pub is_flagged: bool,
    pub is_recent: bool,
    pub is_draft: bool,
    pub is_initial: bool,
    pub is_forwarded: bool,
    pub is_redirected: bool,
    pub attachment_count: u32,
    pub priority: u32,
    pub is_signed: bool,
    pub is_junk: bool,
    pub is_not_junk: bool,
    pub font_size_delta: u32,
    pub has_junk_set: bool,
    pub has_highlighted_text: bool,
    pub unused: bool,
}

impl Flags {
    fn decoding(value: u64) -> Flags {
        enum Decoder<'a> {
            Bool(&'a mut bool),
            Usize(&'a mut u32, usize),
        }
        let mut flags = Flags::default();

        let decoders = [
            (0, Decoder::Bool(&mut flags.is_read)),
            (1, Decoder::Bool(&mut flags.is_deleted)),
            (2, Decoder::Bool(&mut flags.is_answered)),
            (3, Decoder::Bool(&mut flags.is_encrypted)),
            (4, Decoder::Bool(&mut flags.is_flagged)),
            (5, Decoder::Bool(&mut flags.is_recent)),
            (6, Decoder::Bool(&mut flags.is_draft)),
            (7, Decoder::Bool(&mut flags.is_initial)),
            (8, Decoder::Bool(&mut flags.is_forwarded)),
            (9, Decoder::Bool(&mut flags.is_redirected)),
            (10, Decoder::Usize(&mut flags.attachment_count, 6)),
            (16, Decoder::Usize(&mut flags.priority, 7)),
            (23, Decoder::Bool(&mut flags.is_signed)),
            (24, Decoder::Bool(&mut flags.is_junk)),
            (25, Decoder::Bool(&mut flags.is_not_junk)),
            (26, Decoder::Usize(&mut flags.font_size_delta, 7)),
            (29, Decoder::Bool(&mut flags.has_junk_set)),
            (30, Decoder::Bool(&mut flags.has_highlighted_text)),
            (31, Decoder::Bool(&mut flags.unused)),
        ];

        for (index, decoder) in decoders {
            match decoder {
                Decoder::Bool(into) => Self::decode_bool(index, value, into),
                Decoder::Usize(into, shift) => Self::decode_usize(index, value, shift, into),
            }
        }

        flags
    }

    fn decode_bool(index: i32, value: u64, into: &mut bool) {
        *into = (value & 1 << index) > 0
    }

    fn decode_usize(index: i32, value: u64, shift: usize, into: &mut u32) {
        *into = ((value & 1 << index) >> shift) as u32
    }
}

pub fn detect(content: &[u8]) -> Result<(Flags, Dictionary), ParseError> {
    #[cfg(feature = "use-tracing")]
    trace!("Read Plist Part:\n{:?}\n", std::str::from_utf8(content));

    let cursor = Cursor::new(content);
    let reader = BufReader::new(cursor);
    let list = plist::Value::from_reader(reader).unwrap();
    let dictionary = list.into_dictionary().ok_or(ParseError::InvalidPlistData(
        "Plist root is not a dictionary",
    ))?;

    let flags = match dictionary.get("flags") {
        Some(&plist::Value::Integer(data)) => match data.as_unsigned() {
            Some(n) => Flags::decoding(n),
            _ => {
                #[cfg(feature = "use-tracing")]
                tracing::warn!("Invalid Flags in message {:?}", &data);
                Flags::default()
            }
        },
        _ => {
            #[cfg(feature = "use-tracing")]
            tracing::warn!("No Flags in message. Using default.");
            Flags::default()
        }
    };

    Ok((flags, dictionary))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        let value: u64 = 8623489089;
        let flags = Flags::decoding(value);
        assert_eq!(flags.is_read, true);
        assert_eq!(flags.is_draft, true);
        assert_eq!(flags.attachment_count, 0);
    }

    #[test]
    fn test_parse2() {
        let value: u64 = 25770024065;
        let flags = Flags::decoding(value);
        assert_eq!(flags.is_read, true);
        assert_eq!(flags.attachment_count, 16);
        assert_eq!(flags.priority, 512);
    }
}
