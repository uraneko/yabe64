mod encoder {
    use yabe64::Encoder;

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
    use yabe64::B45;
    use yabe64::Decoder;

    #[test]
    fn test0() {
        let input = "QED8WEX0";
        let output = "ietf!";

        assert_eq!(Decoder::new().decode(input), output);
    }
}
