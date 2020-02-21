mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FibSequence {
    previous: u32,
    next: u32
}

#[wasm_bindgen]
impl FibSequence  {

    pub fn new() -> FibSequence {
        FibSequence {
            previous: 0,
            next: 1
        }
    }

    pub fn gen_fibnumber(&mut self) -> u32 {
        let temp = self.previous + self.next;
        self.previous = self.next;
        self.next = temp;
        self.previous

    }

}