use schemars::{schema_for};
extern crate epdx;

use epdx::epd;

fn main() {
    let schema = schema_for!(epd::EPD);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}