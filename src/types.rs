use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize)]
pub struct LoginRequest {
    account: String,
    password: String,
    mp_token: i8,
}

impl LoginRequest {
    pub fn new() -> Self {
        LoginRequest {
            account: String::new(),
            password: String::new(),
            mp_token: -1,
        }
    }

    pub fn account(mut self, account: Cow<str>) -> Self {
        self.account = account.into_owned();
        self
    }

    pub fn password(mut self, password: Cow<str>) -> Self {
        self.password = password.into_owned();
        self
    }
}

#[derive(Deserialize)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub auth_token: String,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub url_slug: String,
    pub avatar: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Donor {
    pub total_amount: String,
    pub user: User,
    pub update_time: u32,
}

#[derive(Deserialize, Debug)]
pub struct Dashboard {
    pub sponsored_history: Vec<Donor>,
}
