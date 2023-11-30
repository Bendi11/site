use front::Site;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    leptos::mount_to_body(Site);
}
