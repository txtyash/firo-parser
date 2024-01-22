use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashSet;

#[derive(Parser)]
#[grammar = "firo.pest"]
pub struct FiroParser;

pub fn parse_origin(content: String) -> Result<HashSet<String>> {
    let file = FiroParser::parse(Rule::origin_file, content.as_str())?
        .next()
        .expect("This unwrap never fails!");
    let mut hs = HashSet::new();
    for path in file.into_inner() {
        match path.as_rule() {
            Rule::path_part => _ = hs.insert(path.as_str().to_string()),
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(hs)
}

pub fn parse_destination(content: String) -> Result<Vec<Vec<String>>> {
    let file = FiroParser::parse(Rule::destination_file, content.as_str())?
        .next()
        .expect("Never Fails");
    let mut destination = Vec::new();
    for line in file.into_inner() {
        let mut path = Vec::new();
        match line.as_rule() {
            Rule::path => {
                for token in line.into_inner() {
                    match token.as_rule() {
                        Rule::path_part | Rule::pin => path.push(token.as_str().to_string()),
                        _ => unreachable!("No rules besides path_part & pin at this level."),
                    }
                }
                destination.push(path);
            }
            Rule::EOI => (),
            _ => unreachable!("No rules to match besides EOI & path"),
        }
    }
    Ok(destination)
}
