#[async_std::main]
async fn main() -> surf::Result<()> {
    let activities = wadachi::run("TaKO8KI").await?;
    assert_eq!(
        activities,
        vec![wadachi::Activity {
            name: "hgoe".to_string(),
            links: vec![wadachi::Link {
                text: "gheo".to_string(),
                link: "gheo".to_string(),
            }]
        }]
    );
    Ok(())
}
