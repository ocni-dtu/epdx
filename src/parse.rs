use crate::epd::EPD;

/// Parse a ILCD formatted EPD in an EPDx struct
///
/// # Arguments
///
/// * `json`: JSON formatted string containing the "full" EPD on ILCD format
///
/// returns: EPD
pub fn parse_ilcd(json: String) -> EPD {
    let epd: EPD = serde_json::from_str(&json).unwrap();
    epd
}
