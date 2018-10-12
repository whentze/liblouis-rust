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

#[test]
fn translate_simple_en() {
    use super::translate_simple;
    let sentence = "This is an example sentence with a rare word: syzygy.";
    assert_eq!(translate_simple("en_US.tbl", sentence), ",? is an example s5t;e )a r>e ~w3 syzygy4");
}

#[test]
fn translate_simple_escape_umlauts() {
    use super::translate_simple;
    let sentence = "äöü";
    assert_eq!(translate_simple("en_US.tbl", sentence), "`a`o`u");
}

#[test]
fn translate_simple_miss_everything() {
    use super::translate_simple;
    let sentence = "はたらく細胞";
    assert_eq!(translate_simple("en_US.tbl", sentence), r"'\x306f''\x305f''\x3089''\x304f''\x7d30''\x80de'");
}

#[test]
fn translate_simple_empty() {
    use super::translate_simple;
    let sentence = "";
    assert_eq!(translate_simple("de.tbl", sentence), "");
}