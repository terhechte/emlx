# Emlx Parser

[![crates.io](https://img.shields.io/crates/v/emlx)](https://crates.io/crates/emlx)
[![docs.rs](https://docs.rs/emlx/badge.svg)](https://docs.rs/emlx/)

Parses Apple Mail.app `Emlx` Files.

Retrives the actual message, meta information as plist, and the flags of the message.

The actual message is returned as a `&[u8]` slice in the `eml` format.

This `message` part is almost in the `eml` format, except that
Apple uses `LF` for linebreaks instead of `CRLF`. Currently,
`emlx` has a feature switch (`use-email-parser`) which enables
a custom fork of the [`email-parser`](https://crates.io/crates/email-parser) crate and already parses
the email for you. It can then be found as the `email` property
on the [`Mail`] struct.

This feature switch is **on** by default.

## Usage

``` rust
use emlx;
let contents: &[u8] = ...
let parsed = parse_emlx(contents).unwrap();

// Flags are a struct with boolean and usize values
let is_read = parsed.flags.is_read;

// Dictionary is a key value map to data in the emlx plist part.
let subject = parsed.dictionary["subject"].as_string().unwrap();

// The actual eml message as bytes
let message = std::str::from_utf8(parsed.message).unwrap();
```

Information on the `Emlx` file format was retrieved from these sites:

- [docs.fileformat.com/email/emlx/](https://docs.fileformat.com/email/emlx/)
- [jwz.org/blog/2005/07/emlx-flags/](https://www.jwz.org/blog/2005/07/emlx-flags/)

## Test email data came from

- [qqilihq/partial-emlx-converter](https://github.com/qqilihq/partial-emlx-converter)
- [mikez/emlx](https://github.com/mikez/emlx/tree/b218ba7ada23239aff68726af744a1b2050f75dd)
