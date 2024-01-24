use serde_json::Error;
use crate::epd::EPD;

/// Parse a ILCD formatted EPD in an EPDx struct
///
/// # Arguments
///
/// * `json`: JSON formatted string containing the "full" EPD on ILCD format
///
/// returns: EPD
pub fn parse_ilcd(json: String) -> Result<EPD, Error> {
    let epd = serde_json::from_str(&json);
    match epd {
        Ok(epd) => Ok(epd),
        Err(err) => Err(err)
    }
}
