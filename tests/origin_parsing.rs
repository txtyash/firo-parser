mod helpers;
use std::collections::HashSet;

use firo_parser::parse_origin;
use helpers::file_to_string;

#[test]
fn valid_origin_file() {
    let origin = HashSet::from([
        String::from("/home/evccyr/file.rs"),
        String::from("plugin/file"),
        String::from("folder/name with space.txt"),
    ]);
    let unparsed = file_to_string("/home/yash/github/firo-parser/tests/origin.firo").unwrap();
    assert_eq!(origin, parse_origin(unparsed).unwrap());
}
#[test]
fn destination_is_invalid_origin_file() {
    let unparsed = file_to_string("/home/yash/github/firo-parser/tests/destination.firo").unwrap();
    assert!(parse_origin(unparsed).is_err());
}
