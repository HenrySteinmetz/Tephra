use std::{
    fmt::Display,
    ops::Deref,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};

pub mod fs;

use fs::*;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: TagID,
    pub entries: Entries,
    pub subtags: Vec<TagID>,
}

#[derive(Debug, Clone)]
pub struct TagID(String);

#[derive(Debug, Clone)]
pub struct Entries(Vec<PathBuf>);

impl AsRef<String> for TagID {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Deref for TagID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for TagID {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

pub struct InvalidFileName;

impl TryFrom<&Path> for TagID {
    type Error = InvalidFileName;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        Ok(TagID(
            value
                .file_stem()
                .and_then(|osstr| osstr.to_str())
                .ok_or(InvalidFileName)?
                .to_string(),
        ))
    }
}

impl TagID {
    pub fn new(value: &str) -> Self {
        TagID(value.to_string())
    }

    pub fn parse<T: AsRef<str>>(value: T) -> Self {
        TagID(value.as_ref().to_case(Case::Snake))
    }

    pub fn get_path(&self) -> PathBuf {
        get_save_dir().join(format!("{}.json", self))
    }
}
