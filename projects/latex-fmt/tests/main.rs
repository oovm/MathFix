extern crate latex_fmt;

use latex_fmt::Settings;

#[test]
#[allow(unused_must_use)]
fn fmt_file() {
    let s = Settings::default();
    s.format_file("tests/file.tex", "tests/out/file.tex");
}
