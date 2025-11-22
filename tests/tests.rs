#[cfg(test)]
mod tests{
    use bank_converter::models::camt053::{DocumentCamt053};
    #[test]
    fn read_xml_camt053_test()  {
        let doc = r#"<?xml version="1.0" encoding="UTF-8"?><document></document>"#;
        match DocumentCamt053::from_read(&mut doc.as_bytes())
        {
            Ok(camt053) => {
                let camt_def = DocumentCamt053::default();
                assert_eq!(camt053, camt_def);
            },
            Err(e) => assert!(false, "Test failed to parse CAMT053 XML: {}", e)
        }
    }


}