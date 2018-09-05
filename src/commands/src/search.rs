extern crate structs;
extern crate telegram_bot;
extern crate humantime;

use self::telegram_bot::*;
use self::humantime::format_duration;
use std::time::Duration;

fn error() -> String {
    "Морти, ты творишь полную хуйню! /search @username".to_owned()
}

pub fn get(chat_id: ChatId, bot_name: &str, message: &str) -> String {
    if !message.starts_with("/") {
        return self::error();
    }

    let cmd = message.clone();
    let collect: Vec<&str> = cmd.split("@").collect();

    if collect.len() == 3 {
        if collect[1].trim() != bot_name || collect[0].trim() != "/search" {
            return self::error();
        }

        return self::get_message(chat_id, collect[2].trim())
    } else if collect.len() == 2 {
        if collect[0].trim() != "/search" {
            return self::error();
        }

        return self::get_message(chat_id, collect[1].trim())
    }

    self::error()
}

fn get_message(chat_id: ChatId, username: &str) -> String {
    use list;

    let chat = list::get_users(chat_id, false);
    match chat.users.iter().position(|u| u.username == Some(username.to_string())) {
        Some(index) => {
            let ago = Duration::new(((structs::get_unix_timestamp() + 1) - chat.users[index].date) as u64, 0);
            let mut answer: String = "Морти, кажется я видел <b>@".to_owned();
            answer.push_str(
                chat.users[index].username.as_ref().unwrap_or(&"Сквонч".to_owned())
            );
            answer.push_str("</b> ");
            answer.push_str(format_duration(ago).to_string().as_str());
            answer.push_str(" назад");

            return answer;
        },
        None => {}
    }

    "Морти, я понятия не имею о ком ты говоришь!".to_owned()
}
