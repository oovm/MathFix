extern crate latex_fmt;

use latex_fmt::Settings;

#[test]
#[allow(unused_must_use)]
fn fmt_simple() {
    let s = Settings::default();
    s.format_file("tests/simple.tex", "tests/out/simple.tex");
}

#[test]
#[allow(unused_must_use)]
fn fmt_complex() {
    let s = Settings::default();
    s.format_file("tests/complex.tex", "tests/out/complex.tex");
}
