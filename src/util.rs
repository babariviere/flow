use app_dirs::AppInfo;
use std::fs;
use std::path::{Path, PathBuf};

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

/// List files recursively.
/// Only returns directory.
pub fn list_files<P: AsRef<Path>>(path: P, depth: u32) -> Vec<String> {
    if depth == 0 {
        return fs::read_dir(path)
            .unwrap()
            .filter_map(|dir| {
                let dir = dir.ok()?;
                if dir.file_type().ok()?.is_dir() {
                    let file_name = dir.file_name().into_string().ok()?;
                    if file_name.starts_with('.') {
                        return None;
                    }
                    Some(file_name)
                } else {
                    None
                }
            })
            .collect();
    }
    fs::read_dir(path)
        .unwrap()
        .filter_map(|dir| {
            let dir = dir.ok()?;
            if dir.file_type().ok()?.is_dir() {
                let file_name = dir.file_name().into_string().ok()?;
                if file_name.starts_with('.') {
                    return None;
                }
                Some(
                    list_files(dir.path(), depth - 1)
                        .into_iter()
                        .map(|child| format!("{}/{}", file_name, child))
                        .collect::<Vec<String>>(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}
