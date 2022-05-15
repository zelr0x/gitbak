use crate::{gitbak::clone_recurse, http, Auth, BackupCfg};
use reqwest::{Client, Url};
use serde::Deserialize;

static LIST_URL: &str = "https://api.github.com/search/repositories";

pub struct GithubBackup {
    client: Client,
}

impl GithubBackup {
    pub fn new(auth: Auth) -> reqwest::Result<GithubBackup> {
        let client = http::http_client_builder()
            .default_headers(http::default_headers(&auth))
            .build()?;
        Ok(GithubBackup { client })
    }

    pub fn pub_only() -> reqwest::Result<GithubBackup> {
        let client = http::http_client_builder().build()?;
        Ok(GithubBackup { client })
    }

    // TODO: handle paging (incomplete_results field or page+per_page params)
    // TODO: decouple github specific logic (list-repos) from paging and other general stuff.
    // TODO: add ability to clone all branches or select them
    // TODO: add ability to skip forks specifically
    pub async fn backup(&self, cfg: &BackupCfg) -> Result<(), Box<dyn std::error::Error>> {
        let repos = self.list_repos(&cfg.user).await?;
        let selected = select_repos(&repos.items, cfg);
        for repo in selected {
            let dest = &cfg.dest.join(&repo.name);
            println!("Cloning {} (id={}) to {:?}", &repo.name, &repo.id, &dest);
            clone_recurse(&repo.clone_url, &dest)
        }
        Ok(())
    }

     // TODO: maybe add retries with exponential backoff?
    async fn list_repos(
        &self,
        user: &str,
    ) -> Result<RepoList, Box<dyn std::error::Error>> {
        let url = Url::parse(&format!("{}?q=user:{}", LIST_URL, user))?;
        let res = self.client.get(url)
            .send().await?
            .error_for_status()?;

        let body_text = res.text().await?;
        serde_json::from_str(&body_text).map_err(|x| x.into())
    }
}

// TODO: simplify
// TODO: use regex?
fn select_repos<'a>(items: &'a [GithubRepo], cfg: &BackupCfg) -> Vec<&'a GithubRepo> {
    cfg.include.as_ref().map_or_else(
        || {
            cfg.exclude_names.as_ref().map_or_else(
                || items.iter().collect(),
                |excl| items.iter().filter(|r| !excl.contains(&r.name)).collect(),
            )
        },
        |incl| items.iter().filter(|r| incl.contains(&r.name)).collect(),
    )
}

#[derive(Debug, Deserialize)]
struct GithubRepo {
    id: isize,
    name: String,
    // full_name: String,
    clone_url: String,
}

#[derive(Debug, Deserialize)]
struct RepoList {
    items: Vec<GithubRepo>,
    // total_count: usize,
    // incomplete_results: bool,
}
