use std::cell::OnceCell;
use wasm_bindgen::prelude::*;

const SENDER: OnceCell<i32> = OnceCell::new();

#[wasm_bindgen]
pub fn get() -> i32 {
    SENDER.set(1).unwrap();

    let binding = SENDER;

    let Some(n) = binding.get() else {
        return -1;
    };

    *n
}
