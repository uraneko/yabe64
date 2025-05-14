mod encoder {
    use makura::Encoder;

    #[test]
    fn test0() {
        let input = "";
        let output = "";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "Zg==";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "Zm8=";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "Zm9v";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "Zm9vYg==";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "Zm9vYmE=";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test7() {
        let input = "foobar";
        let output = "Zm9vYmFy";
        let enc = Encoder::base64();

        assert_eq!(enc.encode(input), output);
    }
}

mod decoder {
    use makura::BASE64;
    use makura::Decoder;

    #[test]
    fn test0() {
        let input = "";
        let output = "";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "Zg==";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "Zm8=";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "Zm9v";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "Zm9vYg==";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "Zm9vYmE=";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "Zm9vYmFy";

        assert_eq!(
            Decoder::decode(output, BASE64)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }
}
