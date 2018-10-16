use super::Louis;
use assert_cmd::prelude::*;
use std::process::Command;
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
        louis.translate_simple("de.tbl", sentence, false, 0),
        "d0s } 6 kz7 sz."
    );
}

#[test]
fn translate_simple_en() {
    let sentence = "This is an example sentence with a rare word: syzygy.";
    let louis = API.lock().unwrap();
    assert_eq!(
        louis.translate_simple("en_US.tbl", sentence, false, 0),
        ",? is an example s5t;e )a r>e ~w3 syzygy4"
    );
}

#[test]
fn translate_simple_escape_umlauts() {
    let sentence = "äöü";
    let louis = API.lock().unwrap();
    assert_eq!(
        louis.translate_simple("en_US.tbl", sentence, false, 0),
        "`a`o`u"
    );
}

#[test]
fn translate_simple_miss_everything() {
    let sentence = "はたらく細胞";
    let louis = API.lock().unwrap();
    assert_eq!(
        louis.translate_simple("en_US.tbl", sentence, false, 0),
        r"'\x306f''\x305f''\x3089''\x304f''\x7d30''\x80de'"
    );
}

#[test]
fn translate_simple_dots_unicode() {
    use super::modes::DOTS_UNICODE;
    let sentence = "Turn this sentence into braille dots please!";
    let louis = API.lock().unwrap();
    assert_eq!(louis.translate_simple("en_US.tbl", sentence, false, DOTS_UNICODE), "⠠⠞⠥⠗⠝⠀⠹⠀⠎⠢⠞⠰⠑⠀⠔⠖⠃⠗⠇⠀⠙⠕⠞⠎⠀⠏⠇⠂⠎⠑⠖");
}

#[test]
fn translate_simple_empty() {
    let sentence = "";
    let louis = API.lock().unwrap();
    assert_eq!(louis.translate_simple("de.tbl", sentence, false, 0), "");
}

#[test]
fn example_lou_translate_forward_fr() {
    Command::cargo_example("lou_translate")
        .unwrap()
        .arg("fr-bfu-g2.ctb")
        .with_stdin().buffer("Le braille est un système d'écriture tactile à points saillants.")
        .assert().success()
        .stdout("¨l ;l û u sy d'é:iture tactile à pts s/|ôs.\n");
}

#[test]
fn example_lou_translate_backward_fr() {
    Command::cargo_example("lou_translate")
        .unwrap()
        .arg("-b")
        .arg("fr-bfu-g2.ctb")
        .with_stdin().buffer("¨l ;l û u sy d'é:iture tactile à pts s/|ôs.")
        .assert().success()
        .stdout("Le braille est un système d'écriture tactile à points saillants.\n");
}

#[ignore]
#[test]
fn example_lou_translate_all_tables() {
    // Translate a string using all tables we can find using both the lou_translate from the examples directory
    // and the lou_translate installed locally, then check if they agree.
    // This takes a while, so it's disabled by default.
    // Execute  `cargo test -- --ignored` to run this.
    let sentence = "\
        Here are some tricky characters:\n\
        Whitespace: \r\u{200B}\u{2028}\u{2029}\u{2060}\u{FEFF}\n\
        Multi-byte: 田中さんにあげて下さいパーティーへ行かないか\n\
        Outside of BMP (i.e. UTF-16 needs surrogate pairs): 𐑖𐑞𐑟𐑤𐑣𐑡𐑙𐑲\n\
        Combinations: ❤️é👯‍♂️\n\
        Let's hope it works!! ﾟ･✿ヾ╲(｡◕‿◕｡)╱✿･ﾟ\n";

    let louis = API.lock().unwrap();
    let tables = louis.list_tables();
    for table in tables {
        let ours = Command::cargo_example("lou_translate")
            .unwrap()
            .arg(&table)
            .with_stdin().buffer(sentence)
            .assert().success()
            .get_output()
            .stdout.clone();

        let expected = Command::new("lou_translate")
            .arg(&table)
            .with_stdin().buffer(sentence)
            .assert().success()
            .get_output()
            .stdout.clone();
            
        assert_eq!(ours, expected);
    }
}
