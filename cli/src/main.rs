use clap::{App, Arg};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("wadachi")
        .version("1.0")
        .author("Takayuki Maeda <takoyaki0316@gmail.com>")
        .about("wadachi scrapes your GitHub Activities")
        .arg(
            Arg::with_name("NAME")
                .help("Sets the GitHub user name")
                .required(true)
                .index(1),
        )
        .get_matches();
    let activities = wadachi::new(matches.value_of("NAME").unwrap())
        .from(2020, 12, 1)
        .execute()
        .await?;
    for activity in activities {
        match activity {
            wadachi::event::Event::PullRequest { repository, .. } => {
                println!("repository: {:?}", repository)
            }
            _ => (),
        }
    }
    Ok(())
}
