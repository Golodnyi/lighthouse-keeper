extern crate structs;
extern crate humantime;
extern crate telegram_bot;
extern crate db;

use self::telegram_bot::*;

pub fn get(chat_id: ChatId) -> String {
    let mut chat = structs::Chat {
        id: chat_id,
        users: vec![]
    };

    chat.users = db::get_users(chat_id);

    let mut users_list: String = "<b>Это всего лишь роботы, Морти! В роботов можно стрелять.</b>\n".to_string();

    for u in chat.users.iter() {
        users_list.push_str(
            u.username.as_ref().unwrap_or(&"Сквонч".to_owned())
        );
        users_list.push_str(" - <b>");
        users_list.push_str(u.msg.to_string().as_str());
        users_list.push_str("</b> сообщений\n");
    }   

    users_list
}