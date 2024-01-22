use firo_parser::parse_destination;

#[test]
fn valid_destination_file() {
    let destination = Vec::from([
        vec![
            String::from("/home/evccyr/file "),
            String::from("<date>"),
            String::from(" .rs"),
        ],
        vec![
            String::from("plugin/calendar "),
            String::from("<!>"),
            String::from(" 2024.pdf"),
        ],
        vec![String::from("<!>"), String::from(" /tmp/something.txt")],
    ]);
    let unparsed = include_str!("destination.firo").to_string();
    assert_eq!(destination, parse_destination(unparsed).unwrap());
}

#[test]
fn origin_is_valid_destination_file() {
    let destination = Vec::from([
        vec![String::from("/home/evccyr/file.rs")],
        vec![String::from("plugin/file")],
        vec![String::from("folder/name with space.txt")],
    ]);
    let unparsed = include_str!("origin.firo").to_string();
    assert_eq!(destination, parse_destination(unparsed).unwrap().to_vec());
}
