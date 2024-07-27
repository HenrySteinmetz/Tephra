use std::{
    fs::{create_dir_all, read_dir},
    io,
    path::PathBuf,
};

use directories::BaseDirs;

use super::TagID;

pub fn get_base_dir() -> PathBuf {
    BaseDirs::new()
        .expect("Failed to get base directories!")
        .config_dir()
        .to_path_buf()
        .join("Tephra/")
}

pub fn get_save_dir() -> PathBuf {
    get_base_dir().join("tags/")
}

pub fn initiate_save_dir() -> std::io::Result<bool> {
    let base_dir = get_save_dir();

    if base_dir.exists() {
        return Ok(true);
    }
    create_dir_all(&base_dir)?;
    Ok(false)
}

pub fn get_save_dir_or_create() -> std::io::Result<PathBuf> {
    let base_dir = get_save_dir();
    if base_dir.exists() {
        return Ok(base_dir);
    }
    initiate_save_dir().map(|_| base_dir)
}

pub fn get_all_tags() -> io::Result<Vec<PathBuf>> {
    Ok(read_dir(get_save_dir())?
        .flatten()
        .map(|d| d.path())
        .filter(|p| p.is_file())
        .collect())
}

pub fn get_all_tag_ids() -> std::io::Result<Vec<TagID>> {
    Ok(get_all_tags()?
        .iter()
        .filter_map(|p| TagID::try_from(p.as_path()).ok())
        .collect())
}
