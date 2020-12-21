use crate::event::{Commit, Event, PullRequest, PullRequestStatus, Repository};
use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

const TIMELINE_BODY: &str = "div.TimelineItem-body";
const ACTIVITY_NAME: &str = "summary span.color-text-primary.ws-normal.text-left";
const OPENED_PULL_REQUEST_REPOSITORY: &str = "summary div span";
const REVIEWED_PULL_REQUEST_REPOSITORY: &str = "summary span span";
const STATUS: &str = "ul li svg";

#[derive(Debug, PartialEq)]
enum WadachiError {
    InvalidMonth(u8),
    InvalidDay(u8),
}

impl std::fmt::Display for WadachiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            WadachiError::InvalidMonth(ref err) => {
                write!(f, "the month {} is greater than 12 or less than 1.", err)
            }
            WadachiError::InvalidDay(ref err) => {
                write!(f, "the day {} is greater than 12 or less than 1.", err)
            }
        }
    }
}

impl Error for WadachiError {}

#[derive(Debug, PartialEq)]
pub struct Filtering {
    pub user: String,
    pub from: Date,
    pub to: Date,
}

#[derive(Debug, PartialEq)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Filtering {
    pub fn from(&mut self, year: u16, month: u8, day: u8) -> &mut Self {
        self.from = Date { year, month, day };
        self
    }

    pub fn to(&mut self, year: u16, month: u8, day: u8) -> &mut Self {
        self.to = Date { year, month, day };
        self
    }

    pub async fn execute(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        if self.from.month < 1 || self.from.month > 12 {
            return Err(Box::new(WadachiError::InvalidMonth(self.from.month)));
        } else if self.to.month < 1 || self.to.month > 12 {
            return Err(Box::new(WadachiError::InvalidMonth(self.to.month)));
        } else if self.from.day < 1 || self.from.day > 31 {
            return Err(Box::new(WadachiError::InvalidDay(self.from.day)));
        } else if self.to.day < 1 || self.to.day > 31 {
            return Err(Box::new(WadachiError::InvalidDay(self.to.day)));
        }

        let timeline_body = Selector::parse(TIMELINE_BODY).unwrap();
        let activity = Selector::parse(ACTIVITY_NAME).unwrap();
        let pull_requests = Selector::parse("details details").unwrap();
        let a = Selector::parse("ul li a").unwrap();
        let status = Selector::parse(STATUS).unwrap();
        let date = Selector::parse("time").unwrap();

        let mut activities = vec![];
        let mut res = surf::get(
            format!(
                "https://github.com/{}?tab=overview&from={}-{}-01&to={}-{}-30",
                self.user, self.from.year, self.from.month, self.to.year, self.from.month
            )
            .as_str(),
        )
        .await?;
        let document = Html::parse_document(res.body_string().await?.as_str());
        for element in document.select(&timeline_body) {
            let activity_name = if let Some(activity) = element.select(&activity).next() {
                activity
                    .text()
                    .collect::<String>()
                    .split('\n')
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
                    activities.push(Event::CreateCommit {
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
                    activities.push(Event::CreateRepository {
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
                        activities.push(Event::CreatePullRequest {
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
                        activities.push(Event::ReviewPullRequest {
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
            }
        }

        Ok(activities)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Date, Filtering};

    #[async_std::test]
    async fn it_returns_error_when_from_month_is_less_than_1(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result1 = Filtering {
            user: "foo".to_string(),
            from: Date {
                year: 2019,
                month: 0,
                day: 1,
            },
            to: Date {
                year: 2019,
                month: 7,
                day: 1,
            },
        }
        .execute()
        .await;
        assert_eq!(result1.is_err(), true);
        if let Err(err) = result1 {
            assert_eq!(
                err.to_string(),
                "the month 0 is greater than 12 or less than 1."
            );
        }

        let result2 = Filtering {
            user: "foo".to_string(),
            from: Date {
                year: 2019,
                month: 1,
                day: 1,
            },
            to: Date {
                year: 2019,
                month: 0,
                day: 1,
            },
        }
        .execute()
        .await;
        assert_eq!(result2.is_err(), true);
        if let Err(err) = result2 {
            assert_eq!(
                err.to_string(),
                "the month 0 is greater than 12 or less than 1."
            );
        }
        Ok(())
    }

    #[async_std::test]
    async fn it_returns_error_when_day_is_less_than_1() -> Result<(), Box<dyn std::error::Error>> {
        let result1 = Filtering {
            user: "foo".to_string(),
            from: Date {
                year: 2019,
                month: 1,
                day: 0,
            },
            to: Date {
                year: 2019,
                month: 7,
                day: 1,
            },
        }
        .execute()
        .await;
        assert_eq!(result1.is_err(), true);
        if let Err(err) = result1 {
            assert_eq!(
                err.to_string(),
                "the day 0 is greater than 12 or less than 1."
            );
        }

        let result2 = Filtering {
            user: "foo".to_string(),
            from: Date {
                year: 2019,
                month: 1,
                day: 1,
            },
            to: Date {
                year: 2019,
                month: 7,
                day: 0,
            },
        }
        .execute()
        .await;
        assert_eq!(result2.is_err(), true);
        if let Err(err) = result2 {
            assert_eq!(
                err.to_string(),
                "the day 0 is greater than 12 or less than 1."
            );
        }
        Ok(())
    }
}
