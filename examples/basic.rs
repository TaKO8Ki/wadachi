#[async_std::main]
async fn main() -> surf::Result<()> {
    let activities = wadachi::new("TaKO8KI").from(12).to(12).execute().await?;
    println!("{:?}", activities);
    Ok(())
}
