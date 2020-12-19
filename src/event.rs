#[derive(PartialEq, Debug)]
pub enum Event {
    CreateCommit {
        name: String,
        commit: Commit,
    },
    CreatePullRequest {
        name: String,
        repository: Repository,
        pull_request: PullRequest,
        status: PullRequestStatus,
        created_at: String,
    },
    ReviewPullRequest {
        name: String,
        repository: Repository,
        pull_request: PullRequest,
        status: PullRequestStatus,
        created_at: String,
    },
    CreateIssue {
        name: String,
        repository: Repository,
        issue: PullRequest,
        status: PullRequestStatus,
        created_at: String,
    },
    CreateRepository {
        name: String,
        repository: Repository,
        created_at: String,
    },
}

#[derive(PartialEq, Debug)]
pub struct PullRequest {
    pub name: String,
    pub url: String,
}

#[derive(PartialEq, Debug)]
pub struct Issue {
    pub name: String,
    pub url: String,
}

#[derive(PartialEq, Debug)]
pub enum PullRequestStatus {
    Opened,
    Closed,
}

#[derive(PartialEq, Debug)]
pub struct Commit {
    pub repository: Repository,
    pub number: usize,
    pub url: String,
}

#[derive(PartialEq, Debug)]
pub struct Repository {
    pub name: String,
    pub url: String,
}
