use ehttp::Request;
use serde::{Deserialize, Serialize};

pub struct GitHubApi {
    repos: Vec<&'static str>,
}

impl GitHubApi {
    pub fn repos(&self) -> std::slice::Iter<'_, &'static str> { self.repos.iter() }

    pub fn default() -> Self {
        Self {
            repos: vec![
                "aap-andre-ytelser",
                "aap-api",
                "aap-bot",
                "aap-devtools",
                "aap-inntekt",
                "aap-libs",
                "aap-meldeplikt",
                "aap-oppgavestyring",
                "aap-personopplysninger",
                "aap-sink",
                "aap-sykepengedager",
                "aap-utbetaling",
                "aap-vedtak",
            ],
        }
    }

    pub fn pull_requests(
        &self,
        token: &mut String,
        repo: &&str,
        callback: impl 'static + Send + FnOnce(Pulls),
    ) {
        let request = Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim().to_string()).as_str()),
            ]),
            ..Request::get(format!("https://api.github.com/repos/navikt/{}/pulls", repo))
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            let body = result.unwrap().bytes;
            let pulls: Pulls = serde_json::from_slice(&body).unwrap();
            callback(pulls);
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Pulls {
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequest {
    pub url: String,
    pub html_url: String,
    pub number: u32,
    id: u32,
    base: Base,
    pub title: String,
    pub body: String,
    state: String,
    user: User,
    created_at: String,
    pub updated_at: String,
}

impl PullRequest {
    pub fn user(self) -> String {
        self.user.login
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Base {
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    id: u32,
    name: String,
}
