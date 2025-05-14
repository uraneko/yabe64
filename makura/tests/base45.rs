mod encoder {
    use makura::Encoder;

    #[test]
    fn test0() {
        let input = "AB";
        let output = "BB8";
        let enc = Encoder::base45();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "Hello!!";
        let output = "%69 VD92EX0";

        let enc = Encoder::base45();

        assert_eq!(enc.encode(input), output);
    }

    #[test]
    fn test2() {
        let input = "base-45";
        let output = "UJCLQE7W581";

        let enc = Encoder::base45();

        assert_eq!(enc.encode(input), output);
    }
}

mod decoder {
    use makura::BASE45;
    use makura::Bases;
    use makura::Decoder;

    #[test]
    fn test0() {
        let output = "QED8WEX0";
        let input = "ietf!";

        assert_eq!(
            Decoder::decode(output, BASE45)
                .unwrap()
                .into_utf8()
                .unwrap(),
            input
        );
    }
}
