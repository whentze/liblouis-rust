#[test]
fn louis_version() {
    use super::liblouis_version;
    use semver::Version;
    assert!(liblouis_version() >= Version::parse("3.0.0"));
}

#[test]
fn list_tables() {
    use super::list_tables;
    let tables = list_tables();
    assert!(tables.len() > 0);
}

#[test]
fn translate_simple_de() {
    use super::translate_simple;
    let sentence = "Dies ist ein kurzer Satz.";
    assert_eq!(translate_simple("de.tbl", sentence), "d0s } 6 kz7 sz.");
}