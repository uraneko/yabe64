use yabe64::Encoder;

#[test]
fn test0() {
    let input = "f";
    let output = "CO======";
    let enc = Encoder::base32();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test1() {
    let input = "fo";
    let output = "CPNG====";
    let enc = Encoder::base32();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test2() {
    let input = "foo";
    let output = "CPNMU===";
    let enc = Encoder::base32();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test3() {
    let input = "foob";
    let output = "CPNMUOG=";
    let enc = Encoder::base32();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test4() {
    let input = "fooba";
    let output = "CPNMUOJ1";
    let enc = Encoder::base32();

    assert_eq!(enc.encode(input), output);
}

#[test]
fn test5() {
    let input = "foobar";
    let output = "CPNMUOJ1E8======";
    let enc = Encoder::base32();

    assert_eq!(enc.encode(input), output);
}
