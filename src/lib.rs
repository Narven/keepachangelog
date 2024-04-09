use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::Path,
};

pub const DEFAULT_FILENAME: &'static str = "CHANGELOG.md";

/// Added. For new features
pub fn add() {
    todo!()
}

/// Changed. For changes in existing functionality
pub fn change() {
    todo!()
}

/// Deprecated. For soon-to-be removed features
pub fn deprecate() {
    todo!()
}

/// Removed. For now removed features
pub fn remove() {
    todo!()
}

/// Fixed. For any bug fixes
pub fn fix() {
    todo!()
}

/// Security. In case of vulnerabilities
pub fn security() {
    todo!()
}

/// Generates simple template
pub fn create(path: Option<&str>) -> Result<(), Error> {
    dbg!(path.is_some());
    let p = if path.is_none() {
        format!("./{}", DEFAULT_FILENAME)
    } else {
        format!("{}/{}", path.unwrap().to_owned(), DEFAULT_FILENAME)
    };

    let full_path = Path::new(&p);

    let file = File::create(full_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n## [Unreleased]\n")?;

    writer.flush()?;

    Ok(())
}

/// Yanked. Pulled because of a serious bug or security issue
pub fn yank() {
    todo!()
}
