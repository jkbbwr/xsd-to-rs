
mod pain_001_001_11 {
    use rust_decimal::Decimal;
    #[derive(xsd_to_rs::Schema)]
    #[xsd_to_rs(path = "./xsd/pain.001.001.11.xsd")]
    struct Root;
}

#[test]
fn works() {
}
