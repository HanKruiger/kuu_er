extern crate qrcodegen;

mod utils;

use utils::to_svg_string;

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn make_qr(url: String, border: i32) -> String {
    let qr_code = &QrCode::encode_text(url.as_str(), QrCodeEcc::Medium).unwrap();
    to_svg_string(&qr_code, border)
}
