use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::iter;
use std::ops::Deref;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidEntry(String),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

type Score = f32;

#[derive(Debug, Default)]
pub struct CacheEntry {
    score: Score,
}

impl CacheEntry {
    /// Create a new cache entry
    pub fn new(score: Score) -> Self {
        Self { score }
    }

    /// Get score from entry.
    pub fn score(&self) -> Score {
        self.score
    }
}

#[derive(Debug, Default)]
pub struct Cache {
    inner: HashMap<String, CacheEntry>,
}

impl Cache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, path: String) {
        let mut entry = self.inner.entry(path).or_insert_with(CacheEntry::default);
        entry.score += 1.;
    }

    pub fn aging<S: Into<Option<Score>>>(&mut self, max_score: S) {
        let max_score = max_score.into().unwrap_or(9000.);
        let score = self.inner.values().fold(0., |acc, v| acc + v.score);

        if score > max_score {
            for val in self.inner.values_mut() {
                val.score *= 0.99;
            }
        }
    }

    // TODO: allow async read
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self> {
        let hash_map = reader
            .lines()
            .map(|line| {
                let line = line?;
                let mut iter = line.split('|');

                let path = iter
                    .next()
                    .ok_or_else(|| Error::InvalidEntry(line.to_owned()))?;
                let score = iter
                    .next()
                    .ok_or_else(|| Error::InvalidEntry(line.to_owned()))
                    .and_then(|s| {
                        s.parse::<Score>()
                            .map_err(|_| Error::InvalidEntry(line.to_owned()))
                    })?;

                Ok((path.to_owned(), CacheEntry { score }))
            })
            .collect::<Result<HashMap<String, CacheEntry>>>()?;

        Ok(Cache { inner: hash_map })
    }

    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        for (path, entry) in &self.inner {
            writeln!(writer, "{}|{}", path, entry.score)?;
        }
        Ok(())
    }
}

type Item = (String, CacheEntry);

impl iter::FromIterator<Item> for Cache {
    fn from_iter<I>(iter: I) -> Self
    where
        I: iter::IntoIterator<Item = Item>,
    {
        Cache {
            inner: HashMap::from_iter(iter),
        }
    }
}

impl iter::Extend<Item> for Cache {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Item>,
    {
        self.inner.extend(iter)
    }
}

impl iter::IntoIterator for Cache {
    type Item = Item;
    type IntoIter = std::collections::hash_map::IntoIter<String, CacheEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl Deref for Cache {
    type Target = HashMap<String, CacheEntry>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
