extern crate autotools;
extern crate bindgen;
#[macro_use]
extern crate log;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default().header("wrapper.h");

    match pkg_config::Config::new()
        .atleast_version("3.1.0")
        .probe("liblouis")
    {
        Ok(system_liblouis) => {
            info!(
                "Found recent system liblouis via pkg-config. Version: {}",
                system_liblouis.version
            );
        }
        Err(e) => {
            info!("pkg-config error while trying to detect liblouis: {}", e);
            info!("building liblouis 3.7.0 from source");

            let dest = autotools::build("liblouis-3.7.0");

            env::set_var("PKG_CONFIG_PATH", dest.join("lib/pkgconfig"));
            let our_liblouis = pkg_config::Config::new().atleast_version("3.7.0").probe("liblouis").unwrap();
            for path in our_liblouis.include_paths {
                builder = builder.clang_args(&["-I", path.parent().unwrap().to_str().unwrap()]);
            }
        }
    };

    let bindings = builder.generate().unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
