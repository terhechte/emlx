#[cfg(test)]
mod tests {
    use emlx::*;

    #[test]
    fn test_email1() {
        // 1.emlx is from:
        // https://raw.githubusercontent.com/qqilihq/partial-emlx-converter/master/test/__testdata/input/Messages/114892.partial.emlx
        let contents = include_bytes!("emails/1.emlx");
        let parsed = parse_emlx(contents).unwrap();
        assert_eq!(parsed.dictionary["color"].as_string().unwrap(), "000000");
        assert!(parsed.flags.is_read);
        assert!(parsed.flags.is_not_junk);
    }

    #[test]
    fn test_email2() {
        // 2.emlx is from:
        // https://raw.githubusercontent.com/mikez/emlx/b218ba7ada23239aff68726af744a1b2050f75dd/tests/plaintext.emlx
        let contents = include_bytes!("emails/2.emlx");
        let parsed = parse_emlx(contents).unwrap();
        assert_eq!(
            parsed.dictionary["conversation-id"]
                .as_unsigned_integer()
                .unwrap(),
            123456
        );
        assert!(parsed.flags.is_draft);
    }

    #[test]
    fn test_email3() {
        // 3.emlx is from:
        // https://raw.githubusercontent.com/mikez/emlx/b218ba7ada23239aff68726af744a1b2050f75dd/tests/richtext.emlx
        let contents = include_bytes!("emails/3.emlx");
        let parsed = parse_emlx(contents).unwrap();
        assert!(parsed.flags.is_read);
    }

    #[test]
    fn test_email4() {
        // 4.emlx is from:
        // https://github.com/crb912/emlx_parse/blob/master/test/sample/1.emlx
        let contents = include_bytes!("emails/4.emlx");
        let parsed = parse_emlx(contents).unwrap();
        assert_eq!(
            parsed.dictionary["date-last-viewed"].as_real().unwrap(),
            0.0
        );
    }

    #[test]
    fn test_emails_broken() {
        let contents = include_bytes!("emails/broken1.emlx");
        assert!(parse_emlx(contents).is_err());
        let contents = include_bytes!("emails/broken2.emlx");
        assert!(parse_emlx(contents).is_err());
        let contents = include_bytes!("emails/broken3.emlx");
        assert!(parse_emlx(contents).is_err());
    }
}
