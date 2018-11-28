extern crate chrono;
extern crate telegram_bot;

use telegram_bot::*;
use chrono::*;

pub struct Chat {
    pub id: ChatId,
    pub users: Vec<User>
}

pub struct User {
    pub id: UserId,
    pub username: Option<String>,
    pub first_name: String,
    pub date: i64,
    pub msg: i64
}

pub struct Silent {
    pub chat_id: String,
    pub users: Vec<User>
}

pub fn get_unix_timestamp() -> i64 {
    let now = Utc::now();
    let seconds: i64 = now.timestamp();

    seconds
}

pub fn get_date() -> String {
    let dt = Local::now();

    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}


pub fn get_period() -> String {
    let dt = Local::now();

    dt.format("D%Y%m%d").to_string()
}