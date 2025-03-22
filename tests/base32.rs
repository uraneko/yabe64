use yabe64::base32_encode as encoder;

#[test]
fn test0() {
    let input = "";
    let output = "";

    assert_eq!(encoder(input), output);
}

#[test]
fn test1() {
    let input = "f";
    let output = "MY======";
    assert_eq!(encoder(input), output);
}

#[test]
fn test2() {
    let input = "fo";
    let output = "MZXQ====";
    assert_eq!(encoder(input), output);
}

#[test]
fn test3() {
    let input = "foo";
    let output = "MZXW6===";
    assert_eq!(encoder(input), output);
}

#[test]
fn test4() {
    let input = "foob";
    let output = "MZXW6YQ=";
    assert_eq!(encoder(input), output);
}

#[test]
fn test5() {
    let input = "fooba";
    let output = "MZXW6YTB";
    assert_eq!(encoder(input), output);
}

#[test]
fn test6() {
    let input = "foobar";
    let output = "MZXW6YTBOI======";
    assert_eq!(encoder(input), output);
}
