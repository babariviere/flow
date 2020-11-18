use clap::Clap;
use fs2::FileExt;

use crate::util::{cache_dir, read_cache, write_cache};

#[derive(Clap)]
pub struct Opts {
    /// Path to add inside the cache
    path: String,
}

fn lock() -> std::fs::File {
    let cache = cache_dir().join("dirs.lock");
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(cache)
        .unwrap();
    file.lock_exclusive().unwrap();
    file
}

pub fn run(opts: Opts) {
    let path = std::path::PathBuf::from(opts.path);
    let path = std::fs::canonicalize(path).unwrap().display().to_string();

    let _lock = lock();
    // TODO: handle error
    let mut cache = read_cache().unwrap();
    cache.add(path);
    cache.aging(None);
    write_cache(&cache).unwrap();
}
