use anyhow::{bail, Context};
use ci::{get_archive_path, get_cached_folders, get_lockfile, get_root, CacheOptions, Result};
use flate2::{write::GzEncoder, Compression};
use fs4::fs_std::FileExt;
use std::{
    borrow::Cow,
    fs::File
};

fn main() -> Result<()> {
    let options = CacheOptions::from_env()?;
    let root = get_root().context("could not get workspace root")?;

    println!("cache options: {options:?}");

    let lockfile = get_lockfile(&options.storage, &options.key)?;

    let archive_path = get_archive_path(&options.storage, &options.key);
    println!("building cache at {}", archive_path.display());
    let output = File::create(&archive_path)?;
    let encoder = GzEncoder::new(output, Compression::default());
    let mut tar = tar::Builder::new(encoder);
    tar.follow_symlinks(false);

    for source_path in get_cached_folders(&root, options.extra_dirs)?
    {

        match std::fs::metadata(&*source_path) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => bail!(
                "only directory can be cached ({} is not a directory)",
                source_path.display()
            ),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                eprintln!(
                    "folder {} does not exist. ignoring...",
                    source_path.display()
                );
                continue;
            }
            Err(err) => return Err(err.into()),
        };

        let inner_path = source_path.strip_prefix("/")?;
        println!(
            "caching folder {} in archive path {}...",
            source_path.display(),
            inner_path.display()
        );
        tar.append_dir_all(inner_path, &*source_path)?;
    }
    
    println!("finalizing cache archive...");
    tar.finish()?;

    println!("releasing cache lockfile...");
    lockfile.unlock()?;
    Ok(())
}
