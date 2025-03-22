use yabe64::base64_encode as encoder;

#[test]
fn test1() {
    let input = "";
    let output = "";

    assert_eq!(encoder(input), output);
}

#[test]
fn test2() {
    let input = "f";
    let output = "Zg==";

    assert_eq!(encoder(input), output);
}

#[test]
fn test3() {
    let input = "fo";
    let output = "Zm8=";

    assert_eq!(encoder(input), output);
}

#[test]
fn test4() {
    let input = "foo";
    let output = "Zm9v";

    assert_eq!(encoder(input), output);
}

#[test]
fn test5() {
    let input = "foob";
    let output = "Zm9vYg==";
    assert_eq!(encoder(input), output);
}

#[test]
fn test6() {
    let input = "fooba";
    let output = "Zm9vYmE=";
    assert_eq!(encoder(input), output);
}

#[test]
fn test7() {
    let input = "foobar";
    let output = "Zm9vYmFy";
    assert_eq!(encoder(input), output);
}
