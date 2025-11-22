#[cfg(test)]
mod tests{
    use bank_converter::models::camt053::DocumentCamt053;
    use super::*;
    #[test]
    fn read_xml_camt053_test()  {
        let doc = r#"<Document xmlns="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02"
                            xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="urn:iso:std:iso:20022:tech:xsd:camt.053.001.02 camt.053.001.02.xsd">
                         <!-- Danske Bank CAMT053 example file for Denmark -->
                        <!-- Balances in the file does not match the transactions booked -->
                       <!-- Payment types included: 4 credits and 2 debits -->
                       <BkToCstmrStmt></BkToCstmrStmt></Document>"#;
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