use std::{fs::{remove_dir, remove_dir_all, remove_file}, io::Result};

pub(crate) fn delete(
    directory: bool,
    path: String,
    recurse: bool
) -> Result<()> {
    if directory {
        if recurse {
            remove_dir_all(path)
        }
        else {
            remove_dir(path)
        }
    }
    else {
        remove_file(path)
    }
}