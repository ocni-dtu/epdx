#[cfg(test)]
mod tests {
    use epdx::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parse_ilcd() -> Result<(), String> {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let file_path = root_dir.join("tests/datafixtures/f63ac879-fa7d-4f91-813e-e816cbdf1927.json");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        match parse::parse_ilcd(contents) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }

    #[test]
    fn test_parse_ilcd_short() -> Result<(), String> {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let file_path = root_dir.join("tests/datafixtures/f63ac879_test.json");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        match parse::parse_ilcd(contents) {
            Ok(_) => Err(String::from("Did not fail")),
            Err(error) => Ok(())
        }
    }
}
