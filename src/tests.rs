use super::Louis;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref API: Mutex<Louis> = Mutex::new(Louis::new().unwrap());
}

#[test]
fn louis_version() {
    use semver::Version;
    let louis = API.lock().unwrap();
    assert!(louis.version() >= Version::parse("3.0.0"));
}

#[test]
fn list_tables() {
    let louis = API.lock().unwrap();
    let tables = louis.list_tables();
    assert!(tables.len() > 0);
}

#[test]
fn translate_simple_de() {
    let sentence = "Dies ist ein kurzer Satz.";
    let louis = API.lock().unwrap();
    assert_eq!(
        louis.translate_simple("de.tbl", sentence, 0),
        "d0s } 6 kz7 sz."
    );
}

#[test]
fn translate_simple_en() {
    let sentence = "This is an example sentence with a rare word: syzygy.";
    let louis = API.lock().unwrap();
    assert_eq!(
        louis.translate_simple("en_US.tbl", sentence, 0),
        ",? is an example s5t;e )a r>e ~w3 syzygy4"
    );
}

#[test]
fn translate_simple_escape_umlauts() {
    let sentence = "äöü";
    let louis = API.lock().unwrap();
    assert_eq!(louis.translate_simple("en_US.tbl", sentence, 0), "`a`o`u");
}

#[test]
fn translate_simple_miss_everything() {
    let sentence = "はたらく細胞";
    let louis = API.lock().unwrap();
    assert_eq!(
        louis.translate_simple("en_US.tbl", sentence, 0),
        r"'\x306f''\x305f''\x3089''\x304f''\x7d30''\x80de'"
    );
}

#[test]
fn translate_simple_dots_unicode() {
    use super::modes::DOTS_UNICODE;
    let sentence = "Turn this sentence into braille dots please!";
    let louis = API.lock().unwrap();
    assert_eq!(louis.translate_simple("en_US.tbl", sentence, DOTS_UNICODE), "⠠⠞⠥⠗⠝⠀⠹⠀⠎⠢⠞⠰⠑⠀⠔⠖⠃⠗⠇⠀⠙⠕⠞⠎⠀⠏⠇⠂⠎⠑⠖");
}

#[test]
fn translate_simple_empty() {
    let sentence = "";
    let louis = API.lock().unwrap();
    assert_eq!(louis.translate_simple("de.tbl", sentence, 0), "");
}