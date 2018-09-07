extern crate structs;
extern crate humantime;
extern crate telegram_bot;
extern crate db;

use self::humantime::format_duration;
use std::time::Duration;
use self::telegram_bot::*;

pub fn get_users(chat_id: ChatId, with_morty: bool) -> structs::Chat {
    let mut chat = structs::Chat {
        id: chat_id,
        users: vec![]
    };

    chat.users = db::get_users(chat_id, "date".to_owned());

    if with_morty {
        let morty = structs::User {
            id: UserId::new(0),
            username: Some("<b>Морти</b>".to_owned()),
            date: (structs::get_unix_timestamp() - (86400 * 7)) + 1,
            first_name: "морти".to_string(),
            msg: 0
        };
                    
        chat.users.push(morty);
    }

    chat
}

pub fn get(chat_id: ChatId) -> String {
    let chat = self::get_users(chat_id, true);

    let mut users_list: String = "<b>Это всего лишь роботы, Морти! В роботов можно стрелять.</b>\n".to_string();

    for u in chat.users.iter() {
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