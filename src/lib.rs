pub mod activity;
pub mod filtering;

use filtering::Filtering;

pub fn new(user: &str) -> Filtering {
    Filtering {
        user: user.to_string(),
        from: None,
        to: None,
    }
}

#[cfg(test)]
mod tests {
    const HOGE: &str = "https://github.com/{}?tab=overview&from=2020-11-01&to=2020-11-30";
}
