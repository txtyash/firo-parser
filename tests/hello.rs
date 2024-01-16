use firo_parser::hello;

#[test]
fn test_hello() {
    assert_eq!(String::from("hello"), hello());
}
