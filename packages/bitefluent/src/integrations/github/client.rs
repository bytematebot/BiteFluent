use base64::Engine;
use reqwest::Client;
use serde::Deserialize;

#[derive(Clone)]
pub struct GithubClient {
    access_token: String,
    client: Client,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GithubTreeFile {
    pub path: String,
    pub sha: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GithubRepository {
    pub id: u64,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub default_branch: String,
}

#[derive(Debug, Deserialize)]
struct GithubTreeResponse {
    tree: Vec<GithubTreeItemResponse>,
    truncated: bool,
}

#[derive(Debug, Deserialize)]
struct GithubTreeItemResponse {
    path: String,
    sha: String,

    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Deserialize)]
struct GithubBlobResponse {
    content: String,
    encoding: String,
}

#[derive(Debug, Deserialize)]
struct GithubRepositoryResponse {
    id: u64,
    name: String,
    full_name: String,
    private: bool,
    default_branch: String,
    owner: GithubOwnerResponse,
}

#[derive(Debug, Deserialize)]
struct GithubOwnerResponse {
    login: String,
}

impl GithubClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            client: Client::new(),
        }
    }

    pub async fn repositories(&self) -> anyhow::Result<Vec<GithubRepository>> {
        let response = self
            .client
            .get("https://api.github.com/user/repos")
            .bearer_auth(&self.access_token)
            .header("User-Agent", "BiteFluent")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .query(&[
                ("visibility", "all"),
                ("affiliation", "owner,collaborator,organization_member"),
                ("sort", "updated"),
                ("direction", "desc"),
                ("per_page", "100"),
            ])
            .send()
            .await?
            .error_for_status()?;

        println!(
            "[github] x-oauth-scopes: {:?}",
            response.headers().get("x-oauth-scopes")
        );

        println!(
            "[github] x-accepted-oauth-scopes: {:?}",
            response.headers().get("x-accepted-oauth-scopes")
        );

        let repositories = response.json::<Vec<GithubRepositoryResponse>>().await?;

        Ok(repositories
            .into_iter()
            .map(GithubRepository::from)
            .collect())
    }

    pub async fn repository_tree(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> anyhow::Result<Vec<GithubTreeFile>> {
        let response = self.fetch_tree_recursive(owner, repo, branch).await?;

        if !response.truncated {
            return Ok(response
                .tree
                .into_iter()
                .map(tree_item_to_file)
                .collect());
        }

        eprintln!(
            "[github] recursive tree response was truncated, falling back to manual tree walk"
        );

        self.repository_tree_walk(owner, repo, branch).await
    }

    pub async fn repository_tree_under_path(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        path: &str,
    ) -> anyhow::Result<Vec<GithubTreeFile>> {
        let normalized_path = normalize_repo_path(path);

        if normalized_path == "." {
            return self.repository_tree(owner, repo, branch).await;
        }

        let tree_sha = self
            .find_tree_sha_for_path(owner, repo, branch, &normalized_path)
            .await?;

        self.repository_tree_walk_with_prefix(owner, repo, &tree_sha, normalized_path)
            .await
    }


    pub async fn repository_blob_content(
        &self,
        owner: &str,
        repo: &str,
        blob_sha: &str,
    ) -> anyhow::Result<String> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}/git/blobs/{blob_sha}");

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .header("User-Agent", "BiteFluent")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?
            .error_for_status()?
            .json::<GithubBlobResponse>()
            .await?;

        decode_github_base64(response.content, &response.encoding)
    }

    async fn fetch_tree_recursive(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> anyhow::Result<GithubTreeResponse> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{branch}");

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .header("User-Agent", "BiteFluent")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .query(&[("recursive", "1")])
            .send()
            .await?
            .error_for_status()?
            .json::<GithubTreeResponse>()
            .await?;

        Ok(response)
    }

    async fn fetch_tree(
        &self,
        owner: &str,
        repo: &str,
        tree_ref: &str,
    ) -> anyhow::Result<GithubTreeResponse> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}/git/trees/{tree_ref}");

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.access_token)
            .header("User-Agent", "BiteFluent")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?
            .error_for_status()?
            .json::<GithubTreeResponse>()
            .await?;

        Ok(response)
    }

    async fn repository_tree_walk(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> anyhow::Result<Vec<GithubTreeFile>> {
        let mut files = Vec::new();
        let mut stack = vec![(branch.to_string(), String::new())];

        while let Some((tree_ref, prefix)) = stack.pop() {
            let response = self.fetch_tree(owner, repo, &tree_ref).await?;

            if response.truncated {
                anyhow::bail!(
                    "GitHub returned a truncated tree for '{}'. This usually means the directory is extremely large. Try setting a narrower translations path or excluding generated/vendor folders.",
                    if prefix.is_empty() { "/" } else { &prefix }
                );
            }

            for item in response.tree {
                let path = join_repo_path(&prefix, &item.path);

                match item.kind.as_str() {
                    "blob" => {
                        files.push(GithubTreeFile {
                            path,
                            sha: item.sha,
                            kind: item.kind,
                        });
                    }

                    "tree" => {
                        if !should_skip_tree_path(&path) {
                            stack.push((item.sha, path));
                        }
                    }

                    _ => {}
                }
            }
        }

        files.sort_by(|left, right| left.path.cmp(&right.path));

        Ok(files)
    }

    async fn repository_tree_walk_with_prefix(
        &self,
        owner: &str,
        repo: &str,
        tree_sha: &str,
        root_prefix: String,
    ) -> anyhow::Result<Vec<GithubTreeFile>> {
        let mut files = Vec::new();
        let mut stack = vec![(tree_sha.to_string(), root_prefix)];

        while let Some((tree_ref, prefix)) = stack.pop() {
            let response = self.fetch_tree(owner, repo, &tree_ref).await?;

            if response.truncated {
                anyhow::bail!(
                    "GitHub returned a truncated tree for '{}'. This directory is too large to scan safely.",
                    prefix
                );
            }

            for item in response.tree {
                let path = join_repo_path(&prefix, &item.path);

                match item.kind.as_str() {
                    "blob" => {
                        files.push(GithubTreeFile {
                            path,
                            sha: item.sha,
                            kind: item.kind,
                        });
                    }

                    "tree" => {
                        if !should_skip_tree_path(&path) {
                            stack.push((item.sha, path));
                        }
                    }

                    _ => {}
                }
            }
        }

        files.sort_by(|left, right| left.path.cmp(&right.path));

        Ok(files)
    }

    async fn find_tree_sha_for_path(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        path: &str,
    ) -> anyhow::Result<String> {
        let mut current_ref = branch.to_string();

        for part in path.split('/').filter(|part| !part.is_empty()) {
            let response = self.fetch_tree(owner, repo, &current_ref).await?;

            if response.truncated {
                anyhow::bail!(
                    "GitHub returned a truncated tree while resolving '{}'. Try setting a narrower translations path.",
                    path
                );
            }

            let Some(item) = response
                .tree
                .into_iter()
                .find(|item| item.kind == "tree" && item.path == part)
            else {
                anyhow::bail!("translations path not found in repository: {}", path);
            };

            current_ref = item.sha;
        }

        Ok(current_ref)
    }
}

impl From<GithubRepositoryResponse> for GithubRepository {
    fn from(repository: GithubRepositoryResponse) -> Self {
        Self {
            id: repository.id,
            owner: repository.owner.login,
            name: repository.name,
            full_name: repository.full_name,
            private: repository.private,
            default_branch: repository.default_branch,
        }
    }
}

fn tree_item_to_file(item: GithubTreeItemResponse) -> GithubTreeFile {
    GithubTreeFile {
        path: item.path,
        sha: item.sha,
        kind: item.kind,
    }
}

fn decode_github_base64(content: String, encoding: &str) -> anyhow::Result<String> {
    if encoding != "base64" {
        anyhow::bail!("unsupported GitHub content encoding: {}", encoding);
    }

    let normalized = content.replace('\n', "");
    let bytes = base64::engine::general_purpose::STANDARD.decode(normalized)?;

    Ok(String::from_utf8(bytes)?)
}

fn normalize_repo_path(path: &str) -> String {
    let path = path.trim().trim_matches('/');

    if path.is_empty() || path == "." {
        ".".to_string()
    } else {
        path.to_string()
    }
}

fn join_repo_path(prefix: &str, path: &str) -> String {
    if prefix.is_empty() {
        path.to_string()
    } else {
        format!("{prefix}/{path}")
    }
}

fn should_skip_tree_path(path: &str) -> bool {
    path.split('/').any(|part| {
        matches!(
            part,
            ".git"
                | "node_modules"
                | "target"
                | "vendor"
                | ".next"
                | ".nuxt"
                | "coverage"
                | ".turbo"
                | ".cache"
        )
    })
}