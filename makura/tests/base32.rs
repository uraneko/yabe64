mod encoder {
    use makura::Encoder;

    #[test]
    fn test0() {
        let input = "";
        let output = "";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "MY======";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "MZXQ====";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "MZXW6===";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "MZXW6YQ=";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "MZXW6YTB";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "MZXW6YTBOI======";
        let enc = Encoder::base32();

        assert_eq!(enc.encode(input), output);
    }
}

mod decoder {
    use makura::BASE32;
    use makura::Bases;
    use makura::Decoder;

    #[test]
    fn test0() {
        let input = "";
        let output = "";
        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "MY======";
        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "MZXQ====";
        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "MZXW6===";
        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "MZXW6YQ=";
        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "MZXW6YTB";
        println!("{}", Bases::default().deduce_encoding(output).unwrap());
        assert_eq!(
            Decoder::decode(output, BASE32)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "MZXW6YTBOI======";
        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }
}
