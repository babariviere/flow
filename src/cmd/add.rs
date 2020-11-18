use clap::Clap;

use crate::util::{read_cache, write_cache};

#[derive(Clap)]
pub struct Opts {
    /// Path to add inside the cache
    path: String,
}

pub fn run(opts: Opts) {
    let path = std::path::PathBuf::from(opts.path);
    let path = std::fs::canonicalize(path).unwrap().display().to_string();

    // TODO: handle error
    let mut cache = read_cache().unwrap();
    cache.add(path);
    cache.aging(None);
    write_cache(&cache).unwrap();
}
