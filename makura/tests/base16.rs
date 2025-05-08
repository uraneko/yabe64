mod encoder {
    use makura::Encoder;

    #[test]
    fn test0() {
        let input = "";
        let output = "";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "f";
        let output = "66";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "666F";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "666F6F";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "666F6F62";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "666F6F6261";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "666F6F626172";
        let enc = Encoder::base16();

        assert_eq!(enc.encode(input), output);
    }
}

mod decoder {
    use makura::BASE16;
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
        let output = "66";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test2() {
        let input = "fo";
        let output = "666F";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test3() {
        let input = "foo";
        let output = "666F6F";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test4() {
        let input = "foob";
        let output = "666F6F62";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test5() {
        let input = "fooba";
        let output = "666F6F6261";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }

    #[test]
    fn test6() {
        let input = "foobar";
        let output = "666F6F626172";

        assert_eq!(
            Decoder::decode_deduce(output).unwrap().into_utf8().unwrap(),
            input
        );
    }
}
