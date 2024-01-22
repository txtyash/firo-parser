use std::collections::HashSet;

use firo_parser::parse_origin;

#[test]
fn valid_origin_file() {
    let origin = HashSet::from([
        String::from("/home/evccyr/file.rs"),
        String::from("plugin/file"),
        String::from("folder/name with space.txt"),
    ]);
    let unparsed = include_str!("origin.firo").to_string();
    assert_eq!(origin, parse_origin(unparsed).unwrap());
}
#[test]
fn destination_is_invalid_origin_file() {
    let unparsed = include_str!("destination.firo").to_string();
    assert!(parse_origin(unparsed).is_err());
}
