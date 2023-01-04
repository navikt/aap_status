use std::future::Future;

use ehttp::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    id: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct Pulls {
    pub pull_requests: Vec<PullRequest>,
}

pub struct GitHubApi {
    token: String,
    repos: Vec<&'static str>,
}

impl GitHubApi {
    pub fn repos(&self) -> std::slice::Iter<'_, &'static str> { self.repos.iter() }

    pub fn update_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn create() -> Self {
        Self {
            token: String::from("abc"),
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

    pub fn pull_requests(&self, repo: &&str, callback: impl 'static + Send + FnOnce(Pulls)) {
        let token = &self.token.trim().to_string();
        let request = Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", &token).as_str()),
            ]),
            ..Request::get(format!("https://api.github.com/repos/navikt/{}/pulls", repo))
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            let body = result.unwrap().bytes;
            let pulls: Pulls = serde_json::from_slice(&body).unwrap();
            callback(pulls);
        });
    }

    // pub fn pull_requests(&self, repo: &&str) -> Result<Pulls, reqwest::Error> {
    //     let path = format!("repos/navikt/{}/pulls", repo);
    //     let base_url = Url::parse("https://api.github.com").unwrap();
    //
    //     println!("using token: {}", &self.token.trim().to_string());
    //
    //     let response = self.client
    //         .get(base_url.join(path.as_str()).unwrap())
    //         .header(ACCEPT, "application/vnd.github+json")
    //         .header(USER_AGENT, "rust web-api-client demo")
    //         .bearer_auth(&self.token.trim().to_string())
    //         .send()?;
    //
    //     response.json::<Pulls>()
    // }

    // #[allow(dead_code)]
    // pub async fn pull_requests_async<Fut>(&self, repo: String) -> reqwest::Result<Pulls> {
    //     let path = format!("repos/navikt/{}/pulls", repo);
    //     let base_url = Url::parse("https://api.github.com").unwrap();
    //
    //     let response = self.async_client
    //         .get(base_url.join(path.as_str()).unwrap())
    //         .header(ACCEPT, "application/vnd.github+json")
    //         .header(USER_AGENT, "rust web-api-client demo")
    //         .bearer_auth(&self.token)
    //         .send()
    //         .await?;
    //
    //     response.json::<Pulls>().await
    // }
}

impl Pulls {
    pub fn print(self) {
        let mut init = true;
        for pull_request in self.pull_requests.into_iter() {
            if init {
                println!();
                println!("{}", pull_request.base.repo.name);
                init = false;
            }
            println!("{:?}", pull_request);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequest {
    pub url: String,
    pub html_url: String,
    id: u32,
    base: Base,
    title: String,
    state: String,
    user: User,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Base {
    repo: Repo,
}
