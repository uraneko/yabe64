use yabe64::base16::base16_encode as encoder;

#[test]
fn test0() {
    let input = "";
    let output = "";
    assert_eq!(encoder(input), output);
}

#[test]
fn test1() {
    let input = "f";
    let output = "66";
    assert_eq!(encoder(input), output);
}

#[test]
fn test2() {
    let input = "fo";
    let output = "666F";
    assert_eq!(encoder(input), output);
}

#[test]
fn test3() {
    let input = "foo";
    let output = "666F6F";
    assert_eq!(encoder(input), output);
}

#[test]
fn test4() {
    let input = "foob";
    let output = "666F6F62";
    assert_eq!(encoder(input), output);
}

#[test]
fn test5() {
    let input = "fooba";
    let output = "666F6F6261";
    assert_eq!(encoder(input), output);
}

#[test]
fn test6() {
    let input = "foobar";
    let output = "666F6F626172";
    assert_eq!(encoder(input), output);
}
