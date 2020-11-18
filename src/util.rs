use app_dirs::AppInfo;
use std::path::PathBuf;

use crate::cache::{self, Cache};

pub const APP_INFO: AppInfo = AppInfo {
    name: "flow",
    author: "babariviere",
};

pub fn cache_dir() -> PathBuf {
    app_dirs::get_app_root(app_dirs::AppDataType::UserCache, &APP_INFO).unwrap()
}

// NOTE: maybe move those to cache module?
// TODO: use async
pub fn read_cache() -> cache::Result<Cache> {
    let cache_dir = cache_dir();
    let file = match std::fs::File::open(cache_dir.join("dirs")) {
        Ok(f) => f,
        Err(_) => return Ok(Cache::new()),
    };

    Cache::from_reader(std::io::BufReader::new(file))
}

pub fn write_cache(cache: &Cache) -> cache::Result<()> {
    let cache_dir = cache_dir();
    std::fs::create_dir_all(&cache_dir)?;
    let file = std::fs::File::create(cache_dir.join("dirs"))?;

    cache.to_writer(file)
}
