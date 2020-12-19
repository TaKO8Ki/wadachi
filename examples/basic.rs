#[async_std::main]
async fn main() -> surf::Result<()> {
    let activities = wadachi::new("TaKO8KI").from(2019, 11).execute().await?;
    println!("{:?}", activities);
    Ok(())
}
