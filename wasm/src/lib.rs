use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: i64, right: i64) -> i64 {
    engine::add(left, right)
}

#[wasm_bindgen]
pub fn sub(left: i64, right: i64) -> i64 {
    engine::sub(left, right)
}

#[wasm_bindgen]
pub struct MyStruct {
    pub a: i64,
    pub b: i64
}

#[wasm_bindgen]
impl MyStruct {
    #[wasm_bindgen(constructor)]
    pub fn new(a: i64, b: i64) -> Self {
        Self {
            a, b
        }
    }

    pub fn some_method(&self) -> i64 {
        self.a + self.b
    }
}