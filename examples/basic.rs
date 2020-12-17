#[async_std::main]
async fn main() -> surf::Result<()> {
    let activities = wadachi::run("TaKO8KI").await?;
    println!("{:?}", activities);
    Ok(())
}
