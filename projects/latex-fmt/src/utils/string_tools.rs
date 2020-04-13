pub use textwrap::indent;

pub fn split_once(in_string: &str, pat: char) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, pat);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}
