use super::translate_simple;

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
    let sentence = "Dies ist ein kurzer Satz.";
    assert_eq!(translate_simple("de.tbl", sentence, 0), "d0s } 6 kz7 sz.");
}

#[test]
fn translate_simple_en() {
    let sentence = "This is an example sentence with a rare word: syzygy.";
    assert_eq!(translate_simple("en_US.tbl", sentence, 0), ",? is an example s5t;e )a r>e ~w3 syzygy4");
}

#[test]
fn translate_simple_escape_umlauts() {
    let sentence = "äöü";
    assert_eq!(translate_simple("en_US.tbl", sentence, 0), "`a`o`u");
}

#[test]
fn translate_simple_miss_everything() {
    let sentence = "はたらく細胞";
    assert_eq!(translate_simple("en_US.tbl", sentence, 0), r"'\x306f''\x305f''\x3089''\x304f''\x7d30''\x80de'");
}

#[test]
fn translate_simple_dots_unicode() {
    use super::mode::DOTS_UNICODE;
    let sentence = "Turn this sentence into braille dots please!";
    assert_eq!(translate_simple("en_US.tbl", sentence, DOTS_UNICODE), "⠠⠞⠥⠗⠝⠀⠹⠀⠎⠢⠞⠰⠑⠀⠔⠖⠃⠗⠇⠀⠙⠕⠞⠎⠀⠏⠇⠂⠎⠑⠖");
}


#[test]
fn translate_simple_empty() {
    let sentence = "";
    assert_eq!(translate_simple("de.tbl", sentence, 0), "");
}