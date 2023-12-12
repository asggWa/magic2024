fn main() {
    println!("For stricts secure scopes. Felíz día. Stay Hacking and help deploy...");
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

