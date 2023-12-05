use cfg_if::cfg_if;
pub mod front;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::front::Site;

    #[wasm_bindgen]
    pub fn hydrate() {
        leptos::mount_to_body(Site);
    }
}}
