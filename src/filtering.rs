use activity::{Activity, Commit, PullRequest, PullRequestStatus, Repository};
use regex::Regex;
use scraper::{Html, Selector};

use crate::activity;

const TIMELINE_BODY: &str = "div.TimelineItem-body";
const ACTIVITY_NAME: &str = "summary span.color-text-primary.ws-normal.text-left";
const OPENED_PULL_REQUEST_REPOSITORY: &str = "summary div span";
const REVIEWED_PULL_REQUEST_REPOSITORY: &str = "summary span span";
const STATUS: &str = "ul li svg";
pub struct Filtering {
    pub user: String,
    pub from: Option<u8>,
    pub to: Option<u8>,
}

impl Filtering {
    pub fn from(&mut self, from: u8) -> &mut Self {
        self.from = Some(from);
        self
    }

    pub fn to(&mut self, to: u8) -> &mut Self {
        self.to = Some(to);
        self
    }

    pub async fn execute(&self) -> surf::Result<Vec<Activity>> {
        let timeline_body = Selector::parse(TIMELINE_BODY).unwrap();
        let activity = Selector::parse(ACTIVITY_NAME).unwrap();
        let pull_requests = Selector::parse("details details").unwrap();
        let a = Selector::parse("ul li a").unwrap();
        let status = Selector::parse(STATUS).unwrap();
        let date = Selector::parse("time").unwrap();

        let mut res = surf::get(format!(
            "https://github.com/{}?tab=overview&from=2020-11-01&to=2020-11-30",
            self.user
        ))
        .await?;
        let html_data = res.body_string().await?;
        let mut activities = vec![];
        let document = Html::parse_document(html_data.as_str());

        for element in document.select(&timeline_body) {
            let activity_name = if let Some(activity) = element.select(&activity).next() {
                activity
                    .text()
                    .collect::<String>()
                    .split("\n")
                    .into_iter()
                    .map(|x| {
                        if x.trim() != "" {
                            return format!("{} ", x.trim());
                        }
                        x.trim().to_string()
                    })
                    .collect::<String>()
                    .trim()
                    .to_string()
            } else {
                continue;
            };

            if Regex::new(r"^Created \d{1,} commits in \d{1,} repositor(y|ies)$")
                .unwrap()
                .is_match(activity_name.as_str())
            {
                let repository_iter = element.select(&a).step_by(2);
                let commit_iter = element.select(&a).skip(1).step_by(2);
                for (repository, commit) in repository_iter.zip(commit_iter) {
                    activities.push(Activity::CreateCommit {
                        name: activity_name.clone(),
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
            } else if Regex::new(r"^Created \d{1,} repositor(y|ies)$")
                .unwrap()
                .is_match(activity_name.as_str())
            {
                let repository_iter = element.select(&a);
                let date_iter = element.select(&date);
                for (repository, date) in repository_iter.zip(date_iter) {
                    activities.push(Activity::CreateRepository {
                        name: activity_name.clone(),
                        repository: Repository {
                            name: repository.text().collect::<String>().trim().to_string(),
                            url: repository.value().attr("href").unwrap().trim().to_string(),
                        },
                        created_at: date.text().collect::<String>().trim().to_string(),
                    })
                }
            } else if Regex::new(
                r"^Opened \d{1,} (other )?pull request(s)? in \d{1,} repositor(y|ies)$",
            )
            .unwrap()
            .is_match(activity_name.as_str())
            {
                for repository in element.select(&pull_requests) {
                    let repository_name = repository
                        .select(&Selector::parse(OPENED_PULL_REQUEST_REPOSITORY).unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_string();
                    let pull_request_iter = repository.select(&a);
                    let status_iter = repository.select(&status);
                    let date_iter = repository.select(&date);
                    for ((pull_request, date), status) in
                        pull_request_iter.zip(date_iter).zip(status_iter)
                    {
                        activities.push(Activity::CreatePullRequest {
                            name: activity_name.clone(),
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
                            status: if status.value().attr("class").unwrap().contains("text-green")
                            {
                                PullRequestStatus::Opened
                            } else {
                                PullRequestStatus::Closed
                            },
                            created_at: date.text().collect::<String>().trim().to_string(),
                        })
                    }
                }
            } else if Regex::new(r"^Reviewed \d{1,} pull request(s)? in \d{1,} repositor(y|ies)$")
                .unwrap()
                .is_match(activity_name.as_str())
            {
                for repository in element.select(&pull_requests) {
                    let repository_name = repository
                        .select(&Selector::parse(REVIEWED_PULL_REQUEST_REPOSITORY).unwrap())
                        .next()
                        .unwrap()
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_string();
                    let pull_request_iter = repository.select(&a);
                    let status_iter = repository.select(&status);
                    let date_iter = repository.select(&date);
                    for ((pull_request, date), status) in
                        pull_request_iter.zip(date_iter).zip(status_iter)
                    {
                        activities.push(Activity::ReviewPullRequest {
                            name: activity_name.clone(),
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
                            status: if status.value().attr("class").unwrap().contains("text-green")
                            {
                                PullRequestStatus::Opened
                            } else {
                                PullRequestStatus::Closed
                            },
                            created_at: date.text().collect::<String>().trim().to_string(),
                        })
                    }
                }
            } else if Regex::new(r"^Opened \d{1,} (other )?issue(s)? in \d{1,} repositor(y|ies)$")
                .unwrap()
                .is_match(activity_name.as_str())
            {
            }
        }
        Ok(activities)
    }
}
