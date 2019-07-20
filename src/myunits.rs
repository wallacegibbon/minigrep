use crate::utils::*;

#[test]
fn case_insensitive() {
    let query = "dUct";
    let content = "Rust:\nsafe, fast, productive.\nPick three.\nblahblah";

    assert_eq!(
        vec!["safe, fast, productive."],
        search_ignore_case(query, content)
    );
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let content = "Rust:\nsafe, fast, productive.\nPick three.\nblahblah";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, content)
    );
}

