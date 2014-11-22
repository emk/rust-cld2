#![feature(phase)]

extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate gcc;
extern crate toml;

use std::collections::HashSet;
use std::default::Default;
use std::io::fs::{File,readdir};
use std::os;
use regex::Regex;

// Fetch the 'package.exclude' list from our Cargo.toml file.  We'll
// use this to decide what sources to admit.
fn get_excluded_sources(manifest: &Path) -> HashSet<String> {
    let text = File::open(manifest).read_to_string().unwrap();
    let toml = toml::Parser::new(text.as_slice()).parse().unwrap();
    let package = toml.get("package").unwrap().as_table().unwrap();
    let exclude = package.get("exclude").unwrap().as_slice().unwrap();
    exclude.iter().map(|e| {
        let str = e.as_str().unwrap();
        Path::new(str).filename_str().unwrap().to_string()
    }).collect()
}

static CC_FILE: Regex = regex!(r"\.cc\z");

// Get all the *.cc files in path that aren't excluded.
fn get_cc_files(dir: &Path, excluded: &HashSet<String>) -> Vec<Path> {
    readdir(dir).unwrap().iter().filter(|p| {
        let filename = p.filename_str().unwrap();
        CC_FILE.is_match(filename) && !excluded.contains(filename)
    }).map(|e| e.clone()).collect()
}

fn main() {
    let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let includes = vec![Path::new("cld2/public"), Path::new("cld2/internal")];

    // Decide what sources to build.
    let excluded = get_excluded_sources(&src.join("Cargo.toml"));
    let mut sources =
        get_cc_files(&src.join("cld2").join("internal"), &excluded);
    sources.push(src.join("src").join("wrapper.cpp"));

    // Convert the sources back to relative path &str values.
    let rel_sources: Vec<Path> = sources.iter().map(|p| {
        p.path_relative_from(&src).unwrap()
    }).collect();
    let rel_sources_str: Vec<&str> = rel_sources.iter().map(|p| {
        p.as_str().unwrap()
    }).collect();

    // Run the build.
    gcc::compile_library("libcld2.a", &gcc::Config {
        include_directories: includes,
        .. Default::default()
    }, rel_sources_str.as_slice());

    // Decide how to link our C++ runtime.  Feel free to submit patches
    // to make this work on your platform.  Other likely options are "c++"
    // and "c++abi" depending on OS and compiler.
    let cxx_abi = "stdc++";
    println!("cargo:rustc-flags=-l {}", cxx_abi);
}
