use serde::{Deserialize, Serialize};

pub struct GitHubApi {}

impl Default for GitHubApi {
    fn default() -> Self { Self {} }
}

impl GitHubApi {
    pub fn pull_requests(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Vec<PullRequest>),
    ) {
        let url = format!("https://api.github.com/repos/navikt/{}/pulls", repo);

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim().to_string()).as_str()),
            ]),
            ..ehttp::Request::get(&url)
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                Ok(res) => {
                    match serde_json::from_slice::<DataOrEmpty<Vec<PullRequest>>>(&res.bytes) {
                        Ok(pulls) => {
                            match pulls {
                                DataOrEmpty::Data(prs) => callback(prs),
                                DataOrEmpty::Empty {} => callback(Vec::<PullRequest>::default()),
                            };
                        }
                        Err(e) => println!("error: {:?} when parsing pulls with content {:?}", e, res)
                    }
                }
                Err(e) => println!("Error {:?} from {:?}", e, &url)
            }
        });
    }

    pub fn runs(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Runs),
    ) {
        let url = format!("https://api.github.com/repos/navikt/{}/actions/runs", repo);

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim().to_string()).as_str()),
            ]),
            ..ehttp::Request::get(&url)
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                Ok(res) => {
                    match serde_json::from_slice(&res.bytes) {
                        Ok(runs) => callback(runs),
                        Err(e) => println!("error: {:?} when parsing runs with content {:?}", e, res)
                    }
                }
                Err(e) => println!("Error {:?} from {:?}", e, &url)
            }
        });
    }

    pub fn workflows(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Vec<Workflow>),
    ) {
        let url = format!("https://api.github.com/repos/navikt/{}/actions/workflows", repo);

        let request = ehttp::Request {
            headers: ehttp::headers(&[
                ("Accept", "application/vnd.github+json"),
                ("User-Agent", "rust web-api-client demo"),
                ("Authorization", format!("Bearer {}", token.trim().to_string()).as_str()),
            ]),
            ..ehttp::Request::get(&url)
        };

        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            match result {
                Ok(res) => {
                    match serde_json::from_slice::<WorkflowsResponse>(&res.bytes) {
                        Ok(workflows) => callback(workflows.workflows),
                        Err(e) => println!("error: {:?} when parsing runs with content {:?}", e, res)
                    }
                }
                Err(e) => println!("Error {:?} from {:?}", e, &url)
            }
        });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullsResponse {
    pub pull_requests: Vec<PullRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub pull_requests: Vec<PullRequest>,
    pub runs: Option<Runs>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DataOrEmpty<T> {
    Data(T),
    Empty {},
}

impl Repository {
    pub fn create(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pull_requests: vec![],
            runs: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequest {
    id: i32,
    pub number: i32,
    url: String,
    head: Head,
    base: Base,

    pub html_url: Option<String>,
    pub title: Option<String>,
    body: Option<String>,
    state: Option<String>,
    pub user: Option<User>,
    created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl PullRequest {
    pub fn user(self) -> String { self.user.unwrap_or(User { login: "unknown".to_string() }).login }
    pub fn updated(self) -> String { self.updated_at.unwrap_or(String::from("---")) }
    pub fn title(self) -> String { self.title.unwrap_or(String::from("---")) }
    pub fn pr_num(self) -> i32 { self.number }
    pub fn url(self) -> String { self.html_url.unwrap_or(String::from("---")) }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Head {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runs {
    pub total_count: i32,
    pub workflow_runs: Vec<WorkflowRun>,
}

impl Default for Runs {
    fn default() -> Self {
        Self {
            total_count: 0,
            workflow_runs: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowRun {
    id: i64,
    pub name: Option<String>,
    check_suite_id: Option<i64>,
    check_suite_node_id: Option<String>,
    head_sha: String,
    path: String,
    run_number: i32,
    pub run_attempt: i32,
    pub event: String,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub workflow_id: i64,
    url: String,
    html_url: String,
    pull_requests: Vec<PullRequest>,
    created_at: String,
    updated_at: String,
    actor: Option<Actor>,
    triggering_actor: Option<Actor>,
    pub run_started_at: Option<String>,
    jobs_url: String,
    logs_url: String,
    check_suite_url: String,
    artifacts_url: String,
    cancel_url: String,
    rerun_url: String,
    workflow_url: String,
    display_title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkflowsResponse {
    pub total_count: i32,
    pub workflows: Vec<Workflow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor {
    name: Option<String>,
    email: Option<String>,
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    #[serde(rename = "type")]
    _type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Base {
    #[serde(rename = "ref")]
    _ref: String,
    sha: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    id: i64,
    url: String,
    name: String,
}
