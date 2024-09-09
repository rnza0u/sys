use std::fs::File;

use ci::{get_archive_path, get_lockfile, CacheOptions, Result};
use flate2::read::GzDecoder;
use fs4::fs_std::FileExt;

fn main() -> Result<()> {
    let options = CacheOptions::from_env()?;

    let lockfile = get_lockfile(&options.storage, &options.key)?;
    
    let archive_path = get_archive_path(&options.storage, &options.key);

    println!("reading cache at {}", archive_path.display());
    let mut archive = match File::open(&archive_path) {
        Ok(file) => {
            let mut archive = tar::Archive::new(GzDecoder::new(file));
            archive.set_preserve_mtime(true);
            archive.set_preserve_ownerships(true);
            archive.set_preserve_permissions(true);
            archive.set_overwrite(true);
            archive
        }
        Err(not_found) if not_found.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("no cache archive was found at {}", archive_path.display());
            return Ok(())
        }
        Err(error) => return Err(error.into()),
    };

    println!("restoring cache from {}...", archive_path.display());
    archive.unpack("/")?;

    println!("releasing cache lockfile...");
    lockfile.unlock()?;

    Ok(())
}