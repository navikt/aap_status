use crate::github::pulls::PullRequest;
use crate::github::runs::WorkflowRuns;
use crate::github::workflows::Workflow;

pub struct GitHubApi {}

impl Default for GitHubApi {
    fn default() -> Self { Self {} }
}

pub trait Pulls {
    fn pull_requests(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Vec<PullRequest>),
    );
}

pub trait Runs {
    fn runs(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(WorkflowRuns),
    );
}

pub trait Workflows {
    fn workflows(
        &self,
        token: &mut String,
        repo: &String,
        callback: impl 'static + Send + FnOnce(Vec<Workflow>),
    );
}

pub trait Teams {
    fn teams(
        &self,
        url: &String,
        token: &mut String,
        callback: impl 'static + Send + FnOnce(ehttp::Response),
    );
}
