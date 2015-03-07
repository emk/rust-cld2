#![feature(collections)]
#![feature(fs)]
#![feature(io)]
#![feature(path)]
#![feature(plugin)]
#![feature(os)]

#![plugin(regex_macros)]

extern crate regex;
extern crate gcc;
extern crate toml;
extern crate collections;

use collections::borrow::ToOwned;
use std::collections::HashSet;
use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::{Path, PathBuf};
use regex::Regex;

// Fetch the 'package.exclude' list from our Cargo.toml file.  We'll
// use this to decide what sources to admit.
fn get_excluded_sources(manifest: &Path) -> HashSet<String> {
    let mut text = String::new();
    File::open(manifest).unwrap().read_to_string(&mut text).unwrap();
    let toml = toml::Parser::new(&text).parse().unwrap();
    let package = toml.get("package").unwrap().as_table().unwrap();
    let exclude = package.get("exclude").unwrap().as_slice().unwrap();
    exclude.iter().map(|e| {
        let str = e.as_str().unwrap();
        Path::new(str).file_name().unwrap().to_str().unwrap().to_string()
    }).collect()
}

static CC_FILE: Regex = regex!(r"\.cc\z");

// Get all the *.cc files in path that aren't excluded.
fn get_cc_files(dir: &Path, excluded: &HashSet<String>) -> Vec<PathBuf> {
    read_dir(dir).unwrap().map(|entry| {
        entry.unwrap().path()
    }).filter(|p| {
        let filename = p.file_name().unwrap().to_str().unwrap();
        CC_FILE.is_match(filename) && !excluded.contains(filename)
    }).map(|p| p.to_owned()).collect()
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src = Path::new(&manifest_dir);

    // Decide what sources to build.
    let excluded = get_excluded_sources(&src.join("Cargo.toml"));
    let mut sources =
        get_cc_files(&src.join("cld2").join("internal"), &excluded);
    sources.push(src.join("src").join("wrapper.cpp"));

    // Convert the sources back to relative path &str values.
    let rel_sources: Vec<PathBuf> = sources.iter().map(|p| {
        p.relative_from(&src).unwrap().to_owned()
    }).collect();

    // Run the build.
    let mut config = gcc::Config::new();
    config.include(&Path::new("cld2/public"));
    config.include(&Path::new("cld2/internal"));
    for f in rel_sources.iter() {
        config.file(f);
    }
    config.compile("libcld2.a");

    // Decide how to link our C++ runtime.  Feel free to submit patches
    // to make this work on your platform.  Other likely options are "c++"
    // and "c++abi" depending on OS and compiler.
    let cxx_abi = "stdc++";
    println!("cargo:rustc-flags=-l {}", cxx_abi);
}
