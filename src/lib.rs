use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let a = "World!";
    let dd = &format!("hello, {}", a);
    alert(dd);
}
