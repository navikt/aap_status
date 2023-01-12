use serde::{Deserialize, Serialize};

use crate::github::github_client::{GitHubApi, Teams};

impl Teams for GitHubApi {
    fn teams(
        &self,
        url: &String,
        token: &mut String,
        callback: impl 'static + Send + FnOnce(ehttp::Response),
    ) {

        let _url = url.to_string();
        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("Access-Control-Allow-Headers", "Link"),
                ("User-Agent", "Rust-wasm-App"),
                ("Authorization", format!("Bearer {}", token.trim().to_string()).as_str()),
            ]),
            ..ehttp::Request::get(&_url)
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                Ok(res) => {
                    callback(res);
                }
                Err(e) => println!("Error {:?} from {:?}", e, &_url)
            }
        });
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub struct Team {
    pub name: String,
    id: i64,
    node_id: String,
    slug: String,
    description: Option<String>,
    privacy: String,
    url: String,
    html_url: String,
    members_url: String,
    repositories_url: String,
    permission: String,
}
