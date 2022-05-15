use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use git2::{build::RepoBuilder, Cred, Repository};

use crate::Auth;

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

pub(crate) fn clone_recurse<P>(clone_url: &str, destination: P, user: &str, auth: &Option<Auth>)
where
    P: AsRef<Path>,
{
    let _ = auth.as_ref().map_or_else(
        || Repository::clone_recurse(clone_url, &destination),
        |auth| auth_clone_recurse(clone_url, &destination, user, auth),
    );
}

fn auth_clone_recurse<P>(
    clone_url: &str,
    destination: P,
    user: &str,
    auth: &Auth,
) -> Result<Repository, git2::Error>
where
    P: AsRef<Path>,
{
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| match auth {
        Auth::BearerToken(tok) => Cred::userpass_plaintext(username_from_url.unwrap_or(user), tok),
    });

    let mut opts = git2::FetchOptions::new();
    opts.remote_callbacks(callbacks);
    let mut builder = RepoBuilder::new();
    builder.fetch_options(opts);
    builder.clone(clone_url, destination.as_ref())
}
