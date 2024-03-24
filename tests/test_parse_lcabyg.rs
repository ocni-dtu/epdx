#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use epdx::*;
    use epdx::epd::Standard;

    macro_rules! parse_lcabyg_tests {
    ($($name:ident: $value:expr)*) => {
    $(
        #[test]
        fn $name() -> Result<(), String> {
            let input = $value;

            let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let file_path = root_dir.join("tests/datafixtures/lcabyg").join(input);
            let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
            let nodes: Vec<lcabyg::Nodes> = serde_json::from_str(&contents).unwrap();

            let mut stages: Vec<lcabyg::Stage> = vec![];
            for node in nodes {
                match node {
                    lcabyg::Nodes::Node(lcabyg::Node::Stage(_node)) => stages.append(&mut vec![_node])
                }

            };
            let _epd = epd::EPD::from(&stages);

            // Assert EPD Info
            assert!(!_epd.id.is_empty());
            assert!(!_epd.name.is_empty());
            assert!(!matches!(_epd.standard, Standard::UNKNOWN));

            // Assert Impact Category Values
            assert!(_epd.gwp.is_some());
            assert!(_epd.odp.is_some());
            assert!(_epd.ap.is_some());
            assert!(_epd.pocp.is_some());
            assert!(_epd.adpe.is_some());
            assert!(_epd.adpf.is_some());
            Ok(())
        }
    )*
    }
}
    parse_lcabyg_tests! {
        lcabyg_5aa09d72: "5aa09d72.json"
    }
}
