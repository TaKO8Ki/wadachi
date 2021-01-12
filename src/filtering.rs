use crate::event::{self, Event};
use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

const PUSH_EVENT: &str = r"^Created \d{1,} commits in \d{1,} repositor(y|ies)$";
const PULL_REQUEST_REVIEW_EVENT: &str =
    r"^Reviewed \d{1,} pull request(s)? in \d{1,} repositor(y|ies)$";
const PULL_REQUEST_EVENT: &str =
    r"^Opened \d{1,} (other )?pull request(s)? in \d{1,} repositor(y|ies)$";
const CREATE_REPOSITORY_EVENT: &str = r"^Created \d{1,} repositor(y|ies)$";

pub(crate) const TIMELINE_BODY: &str = "div.TimelineItem-body";
pub(crate) const EVENT_NAME: &str = "summary span.color-text-primary.ws-normal.text-left";
pub(crate) const OPENED_PULL_REQUEST_REPOSITORY: &str = "summary div span";
pub(crate) const REVIEWED_PULL_REQUEST_REPOSITORY: &str = "summary span span";
pub(crate) const STATUS: &str = "ul li svg";

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
        let activity = Selector::parse(EVENT_NAME).unwrap();

        let mut events = vec![];
        for element in self.document().await?.select(&timeline_body) {
            let event_name = if let Some(activity) = element.select(&activity).next() {
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

            if event_type(&event_name, PUSH_EVENT) {
                event::fetch_push_event(element, &mut events, event_name);
            } else if event_type(&event_name, CREATE_REPOSITORY_EVENT) {
                event::fetch_create_event(element, &mut events, event_name)
            } else if event_type(&event_name, PULL_REQUEST_EVENT) {
                event::fetch_pull_request_event(element, &mut events, event_name)
            } else if event_type(&event_name, PULL_REQUEST_REVIEW_EVENT) {
                event::fetch_pull_request_review_event(element, &mut events, event_name)
            }
        }
        Ok(events)
    }

    async fn document(&self) -> Result<Html, Box<dyn Error>> {
        let mut res = surf::get(
            format!(
                "https://github.com/{}?tab=overview&from={}-{}-{:02}&to={}-{}-{:02}",
                self.user,
                self.from.year,
                self.from.month,
                self.from.day,
                self.to.year,
                self.to.month,
                self.to.day
            )
            .as_str(),
        )
        .await?;
        Ok(Html::parse_document(res.body_string().await?.as_str()))
    }
}

fn event_type(name: &str, event_type: &str) -> bool {
    Regex::new(event_type).unwrap().is_match(name)
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
