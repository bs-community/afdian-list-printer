mod config;
mod donors;
mod login;
mod types;

use config::Config;
use regex::Regex;
use std::borrow::Cow;
use std::io::SeekFrom;
use tokio::fs;
use tokio::prelude::*;
use types::Donor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = toml::from_slice(&fs::read("config.toml").await?)?;

    let token = login::login(Cow::from(config.account), Cow::from(config.password)).await?;
    let donors = donors::fetch(&token).await?;

    let gap = config.amount;
    let (sponsors, backers): (Vec<Donor>, _) = donors.into_iter().partition(|donor| {
        let amount = donor
            .total_amount
            .parse::<f32>()
            .expect("failed to parse money amount");
        amount >= gap
    });

    let sponsors = donors::render_sponsors(sponsors)?;
    let backers = if backers.is_empty() {
        String::new()
    } else {
        donors::render_backers(backers)?
    };

    let regex_sponsors = Regex::new(r"(?:###\sSponsors)[^#]+")?;
    let regex_backers = Regex::new(r"(?:###\sBackers)[^#]+")?;
    for path in &config.target_files {
        let mut buffer = String::new();
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .await?;
        file.read_to_string(&mut buffer).await?;

        let rep = format!("### Sponsors\n\n{}\n", sponsors);
        let replaced = regex_sponsors.replace(&buffer, &*rep);

        let rep = if backers.is_empty() {
            String::new()
        } else {
            format!("### Backers\n\n{}\n", backers)
        };
        let replaced = regex_backers.replace(&replaced, &*rep);

        file.seek(SeekFrom::Start(0)).await?;
        file.set_len(0).await?;
        file.write_all(replaced.as_bytes()).await?;
    }

    Ok(())
}
