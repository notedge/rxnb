use wasm_bindgen_test::*;

use katex_wasmbind::KaTeXOptions;

#[test]
fn ready() {
    println!("it works!")
}

#[wasm_bindgen_test]
fn mode() {
    let d = KaTeXOptions::display_mode();
    let i = KaTeXOptions::inline_mode();
    assert_ne!(d.render("\\frac12"), i.render("\\frac12"));
}
