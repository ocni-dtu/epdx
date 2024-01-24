#[allow(dead_code)]
pub mod ilcd;
mod utils;
pub mod parse;
pub mod epd;

#[cfg(feature = "jsbindings")]
mod javascript;

#[cfg(feature = "pybindings")]
mod python;

#[cfg(feature = "default")]
pub fn convert_ilcd(json: String) -> String {
    let epd = parse::parse_ilcd(json);
    match epd {
        Ok(epd) => serde_json::to_string(&epd).unwrap(),
        Err(_) => String::from("")
    }
}