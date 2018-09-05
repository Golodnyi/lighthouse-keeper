extern crate serde;
extern crate serde_json;
extern crate structs;
extern crate humantime;
extern crate reader;
extern crate telegram_bot;

use self::humantime::format_duration;
use std::time::Duration;
use self::telegram_bot::*;

pub fn add_user(chat_id: ChatId, user: structs::User) {
    let mut chat = self::get_users(chat_id);
    let mut found = false;

    for u in chat.users.iter_mut() {
        if user.id == u.id {
            u.date = user.date;
            u.username = user.username.to_owned();
            found = true;
        }
    }

    if !found {
        chat.users.push(user);
    }

    reader::write_file(chat.id.to_string(), json!(chat).to_string()).unwrap();
}

fn get_users(chat_id: ChatId) -> structs::Chat {
    let mut chat: structs::Chat;

    match reader::read_file(chat_id.to_string()) {
        Ok(data) => {
            chat = serde_json::from_str(&data.as_str()).unwrap()
        },
        Err(_e) => {
            chat = structs::Chat {
                id: chat_id,
                users: vec![]
            }
        }
    };

    let morty = structs::User {
        id: UserId::new(0),
        username: Some("<b>Морти</b>".to_owned()),
        date: (structs::get_unix_timestamp() - (86400 * 7)) + 1
    };
                
    chat.users.push(morty);

    chat
}

pub fn get(chat_id: ChatId) -> String {
    let mut chat = self::get_users(chat_id);

    chat.users.sort_by_key(|k| k.date);
    let mut users_list: String = "<b>Это всего лишь роботы, Морти! В роботов можно стрелять.</b>\n".to_string();

    for u in chat.users.iter().rev() {
        let ago = Duration::new(((structs::get_unix_timestamp() + 1) - u.date) as u64, 0);

        users_list.push_str(
            u.username.as_ref().unwrap_or(&"Сквонч".to_owned())
        );
        users_list.push_str(" - <b>");
        users_list.push_str(format_duration(ago).to_string().as_str());
        users_list.push_str("</b>\n");
    }   

    users_list
}