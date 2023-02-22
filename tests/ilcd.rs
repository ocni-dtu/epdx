#[cfg(test)]
mod tests {
    use ilcd2epd::*;
    use std::fs;

    #[test]
    fn test_parse_ilcd() -> Result<(), String> {
        let result = String::from("hello");
        let file_path = "/home/christian/git/ilcd2epd/tests/datafixtures/f63ac879-fa7d-4f91-813e-e816cbdf1927.json";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        //assert_eq!(ilcd::parse_ilcd(contents), result);
        ilcd::parse_ilcd(contents);
        Ok(())
    }
}
