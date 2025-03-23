use yabe64::base32_hex_encode as encoder;

#[test]
fn test0() {
    let input = "f";
    let output = "CO======";
    assert_eq!(encoder(input), output);
}

#[test]
fn test1() {
    let input = "fo";
    let output = "CPNG====";
    assert_eq!(encoder(input), output);
}

#[test]
fn test2() {
    let input = "foo";
    let output = "CPNMU===";
    assert_eq!(encoder(input), output);
}

#[test]
fn test3() {
    let input = "foob";
    let output = "CPNMUOG=";
    assert_eq!(encoder(input), output);
}

#[test]
fn test4() {
    let input = "fooba";
    let output = "CPNMUOJ1";
    assert_eq!(encoder(input), output);
}

#[test]
fn test5() {
    let input = "foobar";
    let output = "CPNMUOJ1E8======";
    assert_eq!(encoder(input), output);
}
