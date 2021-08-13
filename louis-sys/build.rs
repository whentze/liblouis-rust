extern crate autotools;
extern crate bindgen;
#[macro_use]
extern crate log;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut builder = bindgen::Builder::default().header("wrapper.h");

    let liblouis = match pkg_config::Config::new()
        .atleast_version("3.1.0")
        .probe("liblouis")
    {
        Ok(system_liblouis) => {
            info!(
                "Found recent system liblouis via pkg-config. Version: {}",
                system_liblouis.version
            );
            system_liblouis
        }
        Err(e) => {
            info!("pkg-config error while trying to detect liblouis: {}", e);
            info!("building liblouis 3.18.0 from source");

            let dest = autotools::Config::new("liblouis-3.18.0")
                .enable("-ucs4", None)
                .disable("-dependency-tracking", None)
                .without("-yaml", None)
                .build();

            env::set_var("PKG_CONFIG_PATH", dest.join("lib/pkgconfig"));
            pkg_config::Config::new().atleast_version("3.18.0").probe("liblouis").unwrap()
        }
    };
            for path in liblouis.include_paths {
                builder = builder.clang_args(&["-I", path.parent().unwrap().to_str().unwrap()]);
            }

    let bindings = builder.generate().unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
