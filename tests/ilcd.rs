#[cfg(test)]
mod tests {
    use ilcd2epd::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parse_ilcd() -> Result<(), String> {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let file_path = root_dir.join("tests/datafixtures/f63ac879-fa7d-4f91-813e-e816cbdf1927.json");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        parse::parse_ilcd(contents);
        Ok(())
    }
}
