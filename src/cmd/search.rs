use clap::Clap;

use crate::cache::{Cache, CacheEntry};
use crate::util::read_cache;

#[derive(Clap)]
pub struct Opts {
    #[clap(short, long)]
    project: bool,
    path: Vec<String>,
}

pub fn run(root: String, opts: Opts) {
    let query = opts
        .path
        .iter()
        .map(|p| p.split("/").map(|p| p.to_owned()).collect::<Vec<String>>())
        .flatten()
        .collect::<Vec<String>>()
        .join(" ");

    // TODO: handle error
    let cache = read_cache().unwrap();

    let mut dirs: Cache = crate::util::list_files(&root, 2)
        .into_iter()
        .map(|path| format!("{}/{}", root, path))
        .map(|path| {
            let score = if opts.project {
                999.
            } else {
                cache.get(&path).map(|val| val.score()).unwrap_or(0.)
            };
            (path, CacheEntry::new(score))
        })
        .collect();

    if !opts.project {
        dirs.extend(cache.into_iter());
    }

    let mut result = dirs
        .iter()
        .map(|(path, entry)| {
            let score = score_query(&query, &path);
            (score, path, entry.score())
        })
        .collect::<Vec<(i32, &String, f32)>>();

    result.sort_by(|(isa, _, sa), (isb, _, sb)| isa.cmp(isb).then(sa.partial_cmp(sb).unwrap()));

    // NOTE: for debug purpose only
    // for (score, path) in &result {
    //     println!("[{}] {}", score, path);
    // }
    // TODO: do not use unwrap
    let (_, path, _) = result.pop().unwrap();
    println!("{}", path);
}

// TODO: do we need to move them in another module?
fn score_query(query: &str, path: &str) -> i32 {
    let mut query = query.split(' ').collect::<Vec<&str>>();
    let mut items = path.split('/').collect::<Vec<&str>>();

    let query_len = query.len();
    let items_len = items.len();
    let first_query = query.drain(..1).collect::<Vec<&str>>().pop().unwrap();
    if items_len < query_len {
        return 0;
    }
    let (first_item_pos, first_score) = items
        .iter()
        .take(items_len - query_len + 1)
        .map(|item| score_part(first_query, item))
        .enumerate()
        .max_by_key(|(_, score)| *score)
        .unwrap();

    // Remove unmatchable entries
    if first_score <= 0 {
        return 0;
    }

    let mut global_score = first_score;
    let mut items = items.drain(first_item_pos + 1..);

    for query in query {
        loop {
            let item = match items.next() {
                Some(i) => i,
                None => return 0,
            };
            let score = score_part(query, item);
            // TODO: which score threshold to use?
            if score < 5 {
                continue;
            }
            global_score += score;
            break;
        }
    }

    if items.next().is_none() {
        global_score += 10;
    }

    global_score
}

fn score_part(query: &str, item: &str) -> i32 {
    let mut query_chars = query.chars();
    let mut item_chars = item.chars();

    let mut successive = 0;
    let mut score = 0;
    let mut is_first = true;

    'query: while let Some(q) = query_chars.next() {
        let c = match item_chars.next() {
            Some(c) => c,
            None => {
                score = 0;
                break;
            }
        };

        if q == c {
            score += 1 + successive * 2;
            if is_first {
                score += 4;
            }
            successive += 1;
            is_first = false;
            continue;
        }
        is_first = false;

        loop {
            let c = match item_chars.next() {
                Some(c) => c,
                None => {
                    score = 0;
                    break 'query;
                }
            };

            if q == c {
                score += 1;
                successive = 1;
                break;
            }
        }
    }

    // TODO: which score to use for best match?
    if item_chars.next().is_none() && query_chars.next().is_none() && successive > 0 {
        score += 4;
    }

    score
}
