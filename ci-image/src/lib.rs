use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context};
use fs4::{fs_std::FileExt, lock_contended_error};

pub type Result<T> = std::result::Result<T, anyhow::Error>;

const CACHE_STORAGE: &str = "CACHE_STORAGE";
const CACHE_KEY: &str = "CACHE_KEY";
const CACHE_EXTRA_DIRS: &str = "CACHE_EXTRA_DIRS";

#[derive(Debug)]
pub struct CacheOptions {
    pub key: String,
    pub storage: PathBuf,
    pub extra_dirs: HashSet<PathBuf>,
}

impl CacheOptions {
    pub fn from_env() -> Result<Self> {
        Ok(CacheOptions {
            extra_dirs: std::env::var_os(CACHE_EXTRA_DIRS)
                .map(|dirs| -> Result<_> {
                    Ok(dirs
                        .to_str()
                        .ok_or_else(|| anyhow!(UTF8_ERROR))?
                        .split(',')
                        .map(PathBuf::from)
                        .collect())
                })
                .transpose()
                .context("invalid cache extra dirs array")?
                .unwrap_or_default(),
            key: std::env::var(CACHE_KEY).context("missing or invalid cache key")?,
            storage: std::env::var(CACHE_STORAGE)
                .map(PathBuf::from)
                .context("missing or invalid cache storage")?,
        })
    }
}

const UTF8_ERROR: &str = "parameter is not valid utf-8";

pub fn get_root() -> Result<PathBuf> {
    std::env::var("DRONE_WORKSPACE")
        .map(|p| Ok::<_, anyhow::Error>(PathBuf::from(p)))
        .unwrap_or_else(|_| Ok(std::env::current_dir()?))
}

pub fn get_archive_path(storage_path: &Path, key: &str) -> PathBuf {
    storage_path.join(format!("{key}.tar.gz"))
}

pub fn get_lockfile(storage_path: &Path, key: &str) -> Result<File> {
    let lockfile_path = storage_path.join(format!("{key}.lock"));
    let lockfile = File::create(lockfile_path)?;

    if let Err(err) = lockfile.try_lock_exclusive() {
        if err.kind() != lock_contended_error().kind() {
            bail!("lockfile error: {err}")
        }
        println!("waiting for lock to release for cache key {key}...");
        lockfile.lock_exclusive()?;
    }
    Ok(lockfile)
}

pub fn get_cached_dirs(root: &Path) -> Result<HashSet<PathBuf>> {
    let mut dirs = VecDeque::from([root.to_owned()]);
    let mut paths = HashSet::new();

    loop {
        let current_dir = match dirs.pop_front(){
            Some(dir) => dir,
            None => break
        };

        let cached_folders = match File::open(current_dir.join(".cache.json")) {
            Ok(cache_file) => serde_json::from_reader::<_, HashSet<PathBuf>>(cache_file)?,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => HashSet::default(),
            Err(err) => bail!(err),
        };

        let cached_folders = cached_folders.into_iter()
            .map(|path| current_dir.join(path))
            .collect::<HashSet<_>>();

        paths.extend(cached_folders);

        let entries = std::fs::read_dir(&current_dir)?
            .map(|res| Ok(res?))
            .collect::<Result<Vec<_>>>()?;

        for entry in entries {
            if entry.file_type()?.is_dir(){
                let subdir = entry.path();
                if !paths.contains(&subdir){
                    dirs.push_back(subdir);
                }
            }
        }
    }

    Ok(paths)
}
