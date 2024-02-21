#[cfg(test)]
mod tests {
    use epdx::*;
    use std::fs;
    use std::path::Path;

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
            epd::EPD::from(&stages);
            Ok(())
        }
    )*
    }
}
    parse_lcabyg_tests! {
        lcabyg_5aa09d72: "5aa09d72.json"
    }
}
