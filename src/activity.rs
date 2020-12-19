#[derive(PartialEq, Debug)]
pub enum Activity {
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
    CreateIssue,
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
