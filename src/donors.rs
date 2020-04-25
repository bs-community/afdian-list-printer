use crate::types::{Dashboard, Donor, Response, User};
use handlebars::{Handlebars, TemplateRenderError};
use reqwest::{ClientBuilder, Error};
use serde::Serialize;
use std::collections::BTreeSet;

pub async fn fetch(token: &str) -> Result<Vec<Donor>, Error> {
    let client = ClientBuilder::new().user_agent("Firefox/75.0").build()?;
    let dashboard = client
        .get("https://afdian.net/api/my/dashboard")
        .header("Cookie", format!("auth_token={}", token))
        .send()
        .await?
        .json::<Response<Dashboard>>()
        .await?
        .data;

    Ok(dashboard.sponsored_history)
}

pub fn render_sponsors(donors: Vec<Donor>) -> Result<String, TemplateRenderError> {
    render(donors, 120)
}

pub fn render_backers(donors: Vec<Donor>) -> Result<String, TemplateRenderError> {
    render(donors, 75)
}

fn render(donors: Vec<Donor>, avatar_size: i16) -> Result<String, TemplateRenderError> {
    let backers = extract_users(donors);

    let template = include_str!("donors.hbs");
    let hbs = Handlebars::new();
    let context = Context {
        avatar_size,
        users: backers,
    };

    hbs.render_template(&template, &context)
}

#[derive(Serialize)]
struct Context {
    avatar_size: i16,
    users: BTreeSet<User>,
}

fn extract_users(donors: Vec<Donor>) -> BTreeSet<User> {
    donors
        .into_iter()
        .map(|donor| {
            let mut user = donor.user;
            user.url_slug = if &user.url_slug == "" {
                String::from("")
            } else {
                format!("https://afdian.net/@{}", user.url_slug)
            };

            user
        })
        .collect()
}
