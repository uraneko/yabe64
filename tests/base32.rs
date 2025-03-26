use yabe64::Encoder;

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
