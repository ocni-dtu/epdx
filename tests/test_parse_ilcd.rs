#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use epdx::*;
    use epdx::epd::Standard;

    macro_rules! parse_ilcd_tests {
    ($($name:ident: $value:expr)*) => {
    $(
        #[test]
        fn $name() -> Result<(), String> {
            let input = $value;

            let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let file_path = root_dir.join("tests/datafixtures/ilcd").join(input);
            let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

            match parse::parse_ilcd(contents) {
                Ok(epd) => {
                    // Assert EPD Info
                    assert!(!epd.id.is_empty());
                    assert!(!epd.name.is_empty());
                    assert!(!matches!(epd.standard, Standard::UNKNOWN));

                    // Assert Impact Category Values
                    assert!(epd.gwp.is_some());
                    assert!(epd.odp.is_some());
                    assert!(epd.ap.is_some());
                    assert!(epd.pocp.is_some());
                    assert!(epd.adpe.is_some());
                    assert!(epd.adpf.is_some());
                    Ok(())
                }
                Err(error) => Err(error.to_string())
            }
        }
    )*
    }
}
    parse_ilcd_tests! {
        ilcd_00c28f1f: "00c28f1f-1d49-4c81-9208-138922a1dd6c.json"
        ilcd_0cb92770: "0cb92770-9007-48c6-bc03-466af8894419.json"
        ilcd_0e80e6e7: "0e80e6e7-6882-47be-8bd8-5cd869a746d9.json"
        ilcd_f63ac879: "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
        ilcd_0b488798: "0b488798-140f-4efa-96e2-55aa46ed129a.json"
        ilcd_0d1e4a59: "0d1e4a59-4901-4973-a26f-1698f65a780f.json"
        ilcd_0e85f255: "0e85f255-4d11-4973-9daa-c03889948351.json"
        ilcd_0b4c397d: "0b4c397d-c7a1-4ceb-9718-184334f6364e.json"
        ilcd_0e0c4e37: "0e0c4e37-b7e6-4a4f-b1c9-d36da0aa16f5.json"
        ilcd_0e9fd868: "0e9fd868-9656-489e-be6c-8251b3d43283.json"
        ilcd_023f3b97: "023f3b97-976a-41c4-b0f1-5357b9dc5b3e.json"
        ilcd_c23b2987: "c23b2987-776d-4d55-91c7-5f2a0f2c50f1.json"
    }

    #[test]
    fn test_parse_ilcd_short() -> Result<(), String> {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let file_path = root_dir.join("tests/datafixtures/ilcd/f63ac879_test.json");
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        match parse::parse_ilcd(contents) {
            Ok(_) => Err(String::from("Did not fail")),
            Err(_) => Ok(())
        }
    }
}
