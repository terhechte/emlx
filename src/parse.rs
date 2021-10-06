use super::ParseError;

const LENGTH_TERMINATOR: char = 0x0a as char;

/// Split a `emlx` email into the message and the plist part.
pub fn split(content: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    let (message_end, content) = length(content)?;

    // Make sure there's xml after the message
    if content.len() <= message_end {
        return Err(ParseError::UnexpectedEnding(content.len()));
    }

    Ok((&content[0..message_end], &content[message_end..]))
}

/// Returns the message end, and remaining data
/// from a stream of `emlx` bytes.
fn length(content: &[u8]) -> Result<(usize, &[u8]), ParseError> {
    // Find the terminator and / or whitespace
    let mut terminator_position = 0;
    let mut whitespace_position = 0;
    for (index, byte) in content.iter().enumerate() {
        if byte.is_ascii_whitespace() && whitespace_position == 0 {
            whitespace_position = index;
        }
        if *byte as char == LENGTH_TERMINATOR {
            terminator_position = index;
            break;
        }
    }
    if terminator_position == 0 {
        return Err(ParseError::MissingTerminator);
    }

    // If we have whitespace before the terminator,
    // the whitespace marks the boundary. eg. (where $ = 0x0a)
    // `028    $...` vs `028$...`
    let decimal_range = if whitespace_position > 0 {
        0..whitespace_position
    } else {
        0..terminator_position
    };
    let decimal_slice = &content[decimal_range.clone()];

    let decimal_string = std::str::from_utf8(&decimal_slice)
        .map_err(|error| ParseError::InvalidUnicode(decimal_range, error))?;

    let message_begin = terminator_position + 1;

    let byte_count = decimal_string
        .parse::<usize>()
        .map_err(|error| ParseError::InvalidByteCount(decimal_string.into(), error))?;

    if byte_count == 0 {
        return Err(ParseError::EmptyByteCount);
    }

    let message_end = byte_count;

    // Have to shift the message end by the terminator position
    Ok((message_end, &content[message_begin..]))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_header1() {
        let number = 1234;
        let remainder = "more!";
        let contents = format!("{}   \u{0a}{}", number, remainder);
        let (a, b) = super::length(contents.as_bytes()).unwrap();
        assert_eq!(number, a);
        assert_eq!(remainder, std::str::from_utf8(&b).unwrap());
    }

    #[test]
    fn test_valid_header2() {
        let contents = "12\u{0a}";
        let (_, b) = super::length(contents.as_bytes()).unwrap();
        assert_eq!(&[] as &[u8], b);
    }

    #[test]
    fn test_invalid_header1() {
        let contents = "12";
        assert!(match super::length(contents.as_bytes()) {
            Err(super::ParseError::MissingTerminator) => true,
            _ => false,
        });
    }

    #[test]
    fn test_invalid_header2() {
        let contents = "12$48   \u{0a}";
        assert!(match super::length(contents.as_bytes()) {
            Err(super::ParseError::InvalidByteCount(_, _)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_message1() {
        let contents = "8   \u{0a}12345678Example";
        let (c, b) = super::length(contents.as_bytes()).unwrap();
        let m1 = std::str::from_utf8(&b[0..c]).unwrap();
        let m2 = std::str::from_utf8(&b[c..]).unwrap();
        assert_eq!(m1, "12345678");
        assert_eq!(m2, "Example");
    }
}
