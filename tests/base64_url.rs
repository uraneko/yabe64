mod encoder {
    use makura::Encoder;

    #[test]
    fn test0() {
        let input = "";
        let output = "";
        let enc = Encoder::base64_url();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "🍜";
        let output = "8J-NnA==";
        let enc = Encoder::base64_url();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    #[should_panic]
    fn test() {
        let input = "🍜";
        let output = "8J-NnA==";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }
}

mod decoder {
    use makura::BASE64URL;
    use makura::Decoder;
    // NOTE base64 and base64 url differ at these two char points 62 (+ | -), 63 (/ | _)
    // 64 url can only be tested on an encoded value that contains either - or _

    #[test]
    fn test_encoding() {
        let input = "8J-NnA==";
        let base = BASE64URL;
        assert_eq!(Decoder::deduce_encoding(input).unwrap(), base);
    }

    #[test]
    fn test1() {
        let input = "🍜";
        let output = "8J-NnA==";

        assert_eq!(Decoder::decode_deduce(output).unwrap(), input);
    }
}
