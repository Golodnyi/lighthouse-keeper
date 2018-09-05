extern crate chrono;
extern crate telegram_bot;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use telegram_bot::*;
use chrono::*;

#[derive(Serialize, Deserialize)]
pub struct Chat {
    pub id: ChatId,
    pub users: Vec<User>
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: Option<String>,
    pub date: i64,
}

pub fn get_unix_timestamp() -> i64 {
    let now = Utc::now();
    let seconds: i64 = now.timestamp();

    seconds
}