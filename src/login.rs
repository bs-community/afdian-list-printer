use crate::types::{LoginRequest, LoginResponse, Response};
use reqwest::{ClientBuilder, Error};
use std::borrow::Cow;

pub async fn login<'a>(account: Cow<'a, str>, password: Cow<'a, str>) -> Result<String, Error> {
    let client = ClientBuilder::new().user_agent("Firefox/75.0").build()?;
    let req = LoginRequest::new().account(account).password(password);

    client
        .post("https://afdian.net/api/passport/login")
        .json(&req)
        .send()
        .await?
        .json::<Response<LoginResponse>>()
        .await
        .map(|resp| resp.data.auth_token)
}
