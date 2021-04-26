use wasm_bindgen::prelude::*;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use console_error_panic_hook;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn decode(bytes: Vec<u8>) -> Vec<u8> {
    console_error_panic_hook::set_once();

    let img_buf = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap().into_rgba8();
    let width = img_buf.width();
    let mut data = img_buf.into_raw();
    // add width bytes to end (a u32 as four u8 values):
    data.push((width >> 24) as u8);
    data.push((width >> 16) as u8);
    data.push((width >> 8 ) as u8);
    data.push((width >> 0 ) as u8);
    data
}