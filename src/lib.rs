//! This crate parses Apple Mail.app `emlx` files.
//!
//! The files are parsed into three constituents:
//! - The actual message in the `eml` format. This can then be parsed with any Rust `eml` parser.
//! - The metatadata from a `plist` portion of the `emlx`.
//! - The flags of the message, decoded from the `flags` attribute in the metadata part.
//!
//! More information on the `emlx` format can be found [here](https://docs.fileformat.com/email/emlx/) and [here](https://www.jwz.org/blog/2005/07/emlx-flags/).
//!
//! ## Usage
//!
//! ```no_run
//! use emlx;
//! let contents: &[u8] = &[];
//! let parsed = emlx::parse_emlx(contents).unwrap();
//!
//! // Flags are a struct with boolean and usize values
//! let is_read = parsed.flags.is_read;
//!
//! // Dictionary is a key value map to data in the emlx plist part.
//! let subject = parsed.dictionary["subject"].as_string().unwrap();
//!
//! // The actual eml message as bytes
//! let message = std::str::from_utf8(parsed.message).unwrap();
//! ```
//!
//! ## Features
//! The `tracing` feature will enable `tracing` to give more information about the parsing process.

use std::num::ParseIntError;
use std::ops::Range;
use std::str::Utf8Error;

use thiserror::Error;

#[cfg(feature = "use-tracing")]
use tracing::trace;

mod flags;
mod parse;

pub use flags::Dictionary;
pub use flags::Flags;

/// A representation of the parts of a `emlx` message.
///
/// This is the result of calling [`emlx::parse_emlx`].
#[derive(Debug)]
pub struct Mail<'a> {
    /// Just the `eml` data in the `emlx`. This can be parsed
    /// with any `eml` parser to retrieve message, headers, and
    /// attachments.
    pub message: &'a [u8],
    /// The Apple-Specific flags on this message.
    /// If `dictionary["flags"]` contains none or invalid data,
    /// [`Flags`] is initialized with `Default::default`.
    pub flags: Flags,
    /// The additional metadata found in the `Plist` in the `emlx`.
    /// The contents seem to vary across macOS versions.
    pub dictionary: Dictionary,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Byte Count for Message is Empty")]
    EmptyByteCount,
    #[error("Missing 0xa byte terminator")]
    MissingTerminator,
    #[error("Invalid Unicode in file header [{0:?}]: {1}")]
    InvalidUnicode(Range<usize>, Utf8Error),
    #[error("Invalid Byte Count Header. Can't convert {0} to usize: {1}")]
    InvalidByteCount(String, ParseIntError),
    #[error("Message Ended unexpectedly at byte {0}")]
    UnexpectedEnding(usize),
    #[error("Invalid Plist data: {0}")]
    InvalidPlistData(&'static str),
}

/// Parse bytes into a [`Mail`] struct.
///
/// Returns [`ParseError`] if parsing fails.
pub fn parse_emlx<'a>(content: &'a [u8]) -> Result<Mail<'a>, ParseError> {
    let (message, plist) = parse::split(&content)?;

    #[cfg(feature = "use-tracing")]
    trace!("Read Message Part:\n{:?}\n", std::str::from_utf8(message));

    let (flags, dictionary) = flags::detect(plist)?;
    Ok(Mail {
        message,
        flags,
        dictionary,
    })
}
