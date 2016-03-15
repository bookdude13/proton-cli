//! This module initializes a project.

use std::fs;
use std::io::{Result, Error, ErrorKind};

/// Creates a folder. The folder must not exist or must be empty.
///
/// Impure.
pub fn make_project_folder(root: &str) -> Result<()> {
    // Make the folder - ignore error.
    let _ = fs::create_dir(root);

    fs::read_dir(root)
        .and_then(|iter| {
            let count = iter.count();
            if count == 0 {
                Ok(())
            } else {
                dir_not_empty_err(root, count)
            }
        })
}

fn dir_not_empty_err(root: &str, count: usize) -> Result<()> {
    Err(Error::new(ErrorKind::Other, format!("{} was not empty: {} files exist", root, count)))
}