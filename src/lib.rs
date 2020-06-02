extern crate qrcodegen;

use wasm_bindgen::prelude::*;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

#[wasm_bindgen]
pub fn make_qr(url: String, border: i32) -> String {
  QrCode::encode_text(url.as_str(), QrCodeEcc::Medium)
    .unwrap()
    .to_svg_string(border)
}
