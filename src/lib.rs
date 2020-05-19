use wasm_bindgen::prelude::*;
use latex_fmt::Settings;

#[wasm_bindgen]
fn latex_fmt(input: &str) -> String {
    let s = Settings::default();
    s.format(input)
}