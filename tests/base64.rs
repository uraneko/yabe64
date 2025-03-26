use yabe64::Encoder;

#[test]
fn test1() {
    let input = "";
    let output = "";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test2() {
    let input = "f";
    let output = "Zg==";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test3() {
    let input = "fo";
    let output = "Zm8=";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test4() {
    let input = "foo";
    let output = "Zm9v";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test5() {
    let input = "foob";
    let output = "Zm9vYg==";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test6() {
    let input = "fooba";
    let output = "Zm9vYmE=";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test7() {
    let input = "foobar";
    let output = "Zm9vYmFy";
    let enc = Encoder::base16();

    assert_eq!(enc.encode(input), output);
}
