extern crate structs;
extern crate humantime;

use self::humantime::format_duration;
use std::time::Duration;

pub fn get(mut chat: structs::Chat) -> String {
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