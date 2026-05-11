use reqwest::Client;
use serde::Deserialize;

#[derive(Clone)]
pub struct GithubClient {
    access_token: String,
    client: Client,
}

impl GithubClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            client: Client::new(),
        }
    }

    pub async fn repositories(&self) -> Result<Vec<GithubRepository>, reqwest::Error> {
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

        let repositories = response
            .json::<Vec<GithubRepositoryResponse>>()
            .await?;

        Ok(repositories
            .into_iter()
            .map(GithubRepository::from)
            .collect())
    }
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