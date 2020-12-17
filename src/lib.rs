use scraper::{Html, Selector};

#[derive(PartialEq, Debug)]
pub struct Activity {
    pub name: String,
    pub links: Vec<Link>,
}

#[derive(PartialEq, Debug)]
pub struct Link {
    pub text: String,
    pub link: String,
}

pub async fn run(user: &str) -> surf::Result<Vec<Activity>> {
    let mut res = surf::get(format!(
        "https://github.com/{}?tab=overview&from=2020-11-01&to=2020-11-30",
        user
    ))
    .await?;
    let html_data = res.body_string().await?;
    let mut activities = vec![];
    let document = Html::parse_document(html_data.as_str());
    let timeline_body = Selector::parse("div.TimelineItem-body").unwrap();
    let summary = Selector::parse("summary span.color-text-primary.ws-normal.text-left").unwrap();
    let a = Selector::parse("ul li a").unwrap();
    for element in document.select(&timeline_body) {
        let mut activity = if let Some(activity) = element.select(&summary).next() {
            Activity {
                name: activity
                    .text()
                    .collect::<String>()
                    .split("\n")
                    .into_iter()
                    .map(|x| format!("{} ", x.trim()))
                    .collect::<String>()
                    .trim()
                    .to_string(),
                links: vec![],
            }
        } else {
            continue;
        };
        // }

        for element2 in element.select(&a) {
            activity.links.push(Link {
                text: element2.text().collect::<String>().trim().to_string(),
                link: element2.value().attr("href").unwrap().trim().to_string(),
            })
        }
        activities.push(activity)
    }
    Ok(activities)
}
