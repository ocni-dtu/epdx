use crate::epd::EPD;

pub fn parse_ilcd(json: String) -> EPD {
    let epd: EPD = serde_json::from_str(&json).unwrap();
    println!("deserialized = {:?}", epd);
    epd
}
