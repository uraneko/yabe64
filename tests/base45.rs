mod encoder {
    use yabe64::base45_encode;

    #[test]
    fn test0() {
        let input = "AB";
        let output = "BB8";

        assert_eq!(base45_encode(input), output);
    }

    #[test]
    fn test1() {
        let input = "Hello!!";
        let output = "%69 VD92EX0";

        assert_eq!(base45_encode(input), output);
    }

    #[test]
    fn test2() {
        let input = "base-45";
        let output = "UJCLQE7W581";

        assert_eq!(base45_encode(input), output);
    }
}

