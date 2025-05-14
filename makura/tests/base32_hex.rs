mod encoder {
    use makura::Encoder;

    #[test]
    fn test0() {
        let input = "f";
        let output = "CO======";
        let enc = Encoder::base32_hex();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "fo";
        let output = "CPNG====";
        let enc = Encoder::base32_hex();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test2() {
        let input = "foo";
        let output = "CPNMU===";
        let enc = Encoder::base32_hex();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test3() {
        let input = "foob";
        let output = "CPNMUOG=";
        let enc = Encoder::base32_hex();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test4() {
        let input = "fooba";
        let output = "CPNMUOJ1";
        let enc = Encoder::base32_hex();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test5() {
        let input = "foobar";
        let output = "CPNMUOJ1E8======";
        let enc = Encoder::base32_hex();

        assert_eq!(enc.encode(input), output);
    }
}

mod decoder {
    use makura::BASE32HEX;
    use makura::Decoder;

    #[test]
    fn test0() {
        let input = "f";
        let output = "CO======";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = "fo";
        let output = "CPNG====";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "foo";
        let output = "CPNMU===";

        assert_eq!(
            Decoder::decode(output, BASE32HEX)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = "foob";
        let output = "CPNMUOG=";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = "fooba";
        let output = "CPNMUOJ1";

        assert_eq!(
            Decoder::decode(output, BASE32HEX)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    // #[should_panic]
    // no longer fails with the new deduce_encoding method of Bases
    fn test4_fail() {
        let input = "fooba";
        let output = "CPNMUOJ1";
        // println!("{:?}", Decoder::deduce_encoding(output));
        // -> BASE45 // wrong

        assert_eq!(
            Decoder::decode(output, BASE32HEX)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = "foobar";
        let output = "CPNMUOJ1E8======";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }
}
