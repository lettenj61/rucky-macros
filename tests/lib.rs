#![allow(unused_imports)]

#[macro_use]
extern crate rucky_macros;

import_crates! {
    toml;
    rustc_serialize;
}

#[test]
fn rustc_compiles_this() {
    // This function will be successfully compiled
    // if macro expanded correctly.
}

#[test]
fn specify_member() {
    import! {
        std::io *;
        toml {Value, Table};
        rustc_serialize::json {Json, Builder, AsJson};
        std::path {Path, PathBuf};
    }
}

#[test]
fn use_imported_member() {
    import!(toml::Table);

    let t = Table::new();
    println!("{:?}", t);
}
