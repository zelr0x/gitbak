use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub struct BackupCfg {
    /// A use whose repositories to backup.
    pub(crate) user: String,
    /// A path to the directory used to store the backup data.
    pub(crate) dest: PathBuf,
    /// A set of repo names to include in backup.
    /// If present, backup only repositories with names from that list.
    pub(crate) include: Option<HashSet<String>>,
    /// A set of repo names to exclude from backup.
    pub(crate) exclude_names: Option<HashSet<String>>,
}

impl BackupCfg {
    pub fn new(
        user: String,
        destination: PathBuf,
        only_names: Option<HashSet<String>>,
        exclude_names: Option<HashSet<String>>,
    ) -> Self {
        Self {
            user,
            include: only_names,
            exclude_names,
            dest: destination,
        }
    }
}

pub(crate) fn clone_recurse<P>(clone_url: &str, destination: P)
where
    P: AsRef<Path>,
{
    let _ = git2::Repository::clone_recurse(clone_url, destination);
}
