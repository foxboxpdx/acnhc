#![recursion_limit="1024"]
// use wee alloc as the global allocator
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate serde_derive;
extern crate anyhow;

mod pages;
mod routes;
mod api;
mod app;
mod components;
mod types;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
