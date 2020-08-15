// ACNHC-WASM
// Library implementing the ACNH Catalog as a single-page app using 
// Rust-Wasm and Yew
//
// This file defines the top-level App component.  Sub-components are defined
// in the ./pages directory.
mod pages;

use pages::Home;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}


