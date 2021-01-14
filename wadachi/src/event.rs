use crate::filtering;
use scraper::{ElementRef, Selector};

#[derive(PartialEq, Debug)]
pub enum Event {
    Push {
        name: String,
        commit: Commit,
    },
    PullRequest {
        name: String,
        repository: Repository,
        pull_request: PullRequest,
        status: PullRequestStatus,
        created_at: String,
    },
    PullRequestReview {
        name: String,
        repository: Repository,
        pull_request: PullRequest,
        status: PullRequestStatus,
        created_at: String,
    },
    Issue {
        name: String,
        repository: Repository,
        issue: Issue,
        status: IssueStatus,
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
pub enum IssueStatus {
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

pub(crate) fn fetch_push_event(element: ElementRef, events: &mut Vec<Event>, event_name: String) {
    let a = Selector::parse("ul li a").unwrap();
    let repository_iter = element.select(&a).step_by(2);
    let commit_iter = element.select(&a).skip(1).step_by(2);
    for (repository, commit) in repository_iter.zip(commit_iter) {
        events.push(Event::Push {
            name: event_name.clone(),
            commit: Commit {
                repository: Repository {
                    name: repository.text().collect::<String>().trim().to_string(),
                    url: repository.value().attr("href").unwrap().trim().to_string(),
                },
                number: 3,
                url: commit.value().attr("href").unwrap().trim().to_string(),
            },
        })
    }
}

pub(crate) fn fetch_pull_request_event(
    element: ElementRef,
    events: &mut Vec<Event>,
    event_name: String,
) {
    let pull_requests = Selector::parse("details details").unwrap();
    let a = Selector::parse("ul li a").unwrap();
    let status = Selector::parse(filtering::STATUS).unwrap();
    let date = Selector::parse("time").unwrap();
    for repository in element.select(&pull_requests) {
        let repository_name = repository
            .select(&Selector::parse(filtering::OPENED_PULL_REQUEST_REPOSITORY).unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string();
        let pull_request_iter = repository.select(&a);
        let status_iter = repository.select(&status);
        let date_iter = repository.select(&date);
        for ((pull_request, date), status) in pull_request_iter.zip(date_iter).zip(status_iter) {
            events.push(Event::PullRequest {
                name: event_name.clone(),
                repository: Repository {
                    name: repository_name.clone(),
                    url: format!("/{}", repository_name.clone()),
                },
                pull_request: PullRequest {
                    name: pull_request.text().collect::<String>().trim().to_string(),
                    url: pull_request
                        .value()
                        .attr("href")
                        .unwrap()
                        .trim()
                        .to_string(),
                },
                status: if status.value().attr("class").unwrap().contains("text-green") {
                    PullRequestStatus::Opened
                } else {
                    PullRequestStatus::Closed
                },
                created_at: date.text().collect::<String>().trim().to_string(),
            })
        }
    }
}

pub(crate) fn fetch_pull_request_review_event(
    element: ElementRef,
    events: &mut Vec<Event>,
    event_name: String,
) {
    let pull_requests = Selector::parse("details details").unwrap();
    let a = Selector::parse("ul li a").unwrap();
    let status = Selector::parse(filtering::STATUS).unwrap();
    let date = Selector::parse("time").unwrap();
    for repository in element.select(&pull_requests) {
        let repository_name = repository
            .select(&Selector::parse(filtering::REVIEWED_PULL_REQUEST_REPOSITORY).unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string();
        let pull_request_iter = repository.select(&a);
        let status_iter = repository.select(&status);
        let date_iter = repository.select(&date);
        for ((pull_request, date), status) in pull_request_iter.zip(date_iter).zip(status_iter) {
            events.push(Event::PullRequestReview {
                name: event_name.clone(),
                repository: Repository {
                    name: repository_name.clone(),
                    url: format!("/{}", repository_name.clone()),
                },
                pull_request: PullRequest {
                    name: pull_request.text().collect::<String>().trim().to_string(),
                    url: pull_request
                        .value()
                        .attr("href")
                        .unwrap()
                        .trim()
                        .to_string(),
                },
                status: if status.value().attr("class").unwrap().contains("text-green") {
                    PullRequestStatus::Opened
                } else {
                    PullRequestStatus::Closed
                },
                created_at: date.text().collect::<String>().trim().to_string(),
            })
        }
    }
}

pub(crate) fn fetch_create_repository_event(
    element: ElementRef,
    events: &mut Vec<Event>,
    event_name: String,
) {
    let a = Selector::parse("ul li a").unwrap();
    let date = Selector::parse("time").unwrap();
    let repository_iter = element.select(&a);
    let date_iter = element.select(&date);
    for (repository, date) in repository_iter.zip(date_iter) {
        events.push(Event::CreateRepository {
            name: event_name.clone(),
            repository: Repository {
                name: repository.text().collect::<String>().trim().to_string(),
                url: repository.value().attr("href").unwrap().trim().to_string(),
            },
            created_at: date.text().collect::<String>().trim().to_string(),
        })
    }
}

pub(crate) fn fetch_issue_event(element: ElementRef, events: &mut Vec<Event>, event_name: String) {
    let issues = Selector::parse("details details").unwrap();
    let a = Selector::parse("ul li a").unwrap();
    let status = Selector::parse(filtering::STATUS).unwrap();
    let date = Selector::parse("time").unwrap();
    for repository in element.select(&issues) {
        let repository_name = repository
            .select(&Selector::parse(filtering::OPENED_PULL_REQUEST_REPOSITORY).unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<String>()
            .trim()
            .to_string();
        let issue_iter = repository.select(&a);
        let status_iter = repository.select(&status);
        let date_iter = repository.select(&date);
        for ((pull_request, date), status) in issue_iter.zip(date_iter).zip(status_iter) {
            events.push(Event::Issue {
                name: event_name.clone(),
                repository: Repository {
                    name: repository_name.clone(),
                    url: format!("/{}", repository_name.clone()),
                },
                issue: Issue {
                    name: pull_request.text().collect::<String>().trim().to_string(),
                    url: pull_request
                        .value()
                        .attr("href")
                        .unwrap()
                        .trim()
                        .to_string(),
                },
                status: if status.value().attr("class").unwrap().contains("text-green") {
                    IssueStatus::Opened
                } else {
                    IssueStatus::Closed
                },
                created_at: date.text().collect::<String>().trim().to_string(),
            })
        }
    }
}
