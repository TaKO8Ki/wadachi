<div align="center">

 # wadachi

 wadachi scrapes your GitHub Activities.

 [![github workflow status](https://img.shields.io/github/workflow/status/TaKO8Ki/wadachi/CI/main)](https://github.com/TaKO8Ki/wadachi/actions) [![crates](https://img.shields.io/crates/v/wadachi.svg?logo=rust)](https://crates.io/crates/wadachi) [![docs](https://img.shields.io/badge/docs-wadachi-8da0cb?labelColor=555555&logo=rust)](https://docs.rs/wadachi)

 [Usage](#Usage) | [Examples](examples) | [Docs](https://docs.rs/wadachi)

</div>

# Usage

```rust
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let activities = wadachi::new("TaKO8KI").from(2020, 12).execute().await?;
    println!("{:?}o", activities);
    Ok(())
}
```
