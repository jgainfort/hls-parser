extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use web_sys::console;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub enum PlaylistType {
    Master,
    Media,
}

#[wasm_bindgen]
pub enum StreamType {
    Event,
    Live,
    Vod,
}

#[wasm_bindgen]
pub struct DescriptiveTag {}

#[wasm_bindgen]
pub struct Playlist {
    playlistType: PlaylistType,
    tags: Vec<DescriptiveTag>,
}

#[wasm_bindgen]
pub fn parse(manifest: &str, url: &str) -> Playlist {
    console::log_2(&"manifest: ".into(), &manifest.into());
    console::log_2(&"url: ".into(), &url.into());

    Playlist {
        playlistType: PlaylistType::Master,
        tags: Vec::new(),
    }
}
