use anyhow::{bail, Context};
use ci::{get_archive_path, get_cached_dirs, get_lockfile, get_root, CacheOptions, Result};
use flate2::{write::GzEncoder, Compression};
use fs4::fs_std::FileExt;
use std::fs::File;

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

    for cached_folder in get_cached_dirs(&root)?.into_iter().chain(options.extra_dirs) {
        
        let cached_folder = if cached_folder.is_absolute(){
            cached_folder
        } else {
            root.join(cached_folder)
        };
        
        match std::fs::metadata(&*cached_folder) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => bail!(
                "only directory can be cached ({} is not a directory)",
                cached_folder.display()
            ),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                eprintln!(
                    "folder {} does not exist. ignoring...",
                    cached_folder.display()
                );
                continue;
            }
            Err(err) => return Err(err.into()),
        };

        let inner_path = cached_folder.strip_prefix("/")?;
        println!(
            "caching folder {} in archive path {}...",
            cached_folder.display(),
            inner_path.display()
        );
        tar.append_dir_all(inner_path, &*cached_folder)?;
    }

    println!("finalizing cache archive...");
    tar.finish()?;

    println!("releasing cache lockfile...");
    lockfile.unlock()?;
    Ok(())
}
