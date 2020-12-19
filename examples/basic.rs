#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let activities = wadachi::new("TaKO8KI").from(2020, 12).execute().await?;
    println!("{:?}", activities);
    Ok(())
}
