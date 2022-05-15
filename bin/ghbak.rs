use std::{collections::HashSet, path::PathBuf};

use clap::Parser;
use gitbak::{Auth, BackupCfg, GithubBackup};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Parser, Debug)]
#[clap(name = "GH Backup")]
#[derive(Zeroize, ZeroizeOnDrop)]
struct Args {
    /// A user whose repositories to backup.
    user: String,
    /// A path to the directory used to store the backup data.
    #[clap(parse(from_os_str))]
    #[zeroize(skip)]
    dest: PathBuf,
    /// Github auth token.
    #[clap(short, long)]
    token: Option<String>,
    /// A set of repo names to include in backup as a comma-delimited string.
    /// If present, backup only repositories with names from that list.
    #[clap(short, long, use_value_delimiter = true)]
    #[zeroize(skip)]
    include: Option<Vec<String>>,
    /// A set of repo names to exclude from backup as a comma-delimited string.
    #[clap(short = 'x', long = "exclude")]
    #[zeroize(skip)]
    exclude: Option<Vec<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();
    let user = args.user.trim().into();
    let dest = std::mem::take(&mut args.dest);
    let only_names = args.include.as_ref().map(|xs| to_set(&xs));
    let exclude_names = args.exclude.as_ref().map(|xs| to_set(&xs));
    let cfg = BackupCfg::new(user, dest, only_names, exclude_names);
    let bak = args.token.as_mut().map_or_else(
        || GithubBackup::pub_only().unwrap(),
        |tok| GithubBackup::new(Auth::BearerToken(Box::new(std::mem::take(tok)))).unwrap(),
    );
    bak.backup(&cfg).await
}

#[inline]
fn to_set(s: &Vec<String>) -> HashSet<String> {
    s.iter().map(|x| x.trim().into()).collect()
}
