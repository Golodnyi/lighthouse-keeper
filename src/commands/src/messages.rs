extern crate structs;
extern crate humantime;
extern crate telegram_bot;
extern crate db;

use self::telegram_bot::*;

pub fn get(chat_id: ChatId, offset: u32, count: u32) -> String {
    let mut chat = structs::Chat {
        id: chat_id,
        users: vec![]
    };

    chat.users = db::get_users(chat_id, offset, count);

    let mut users_list: String = "<b>Это всего лишь роботы, Морти! В роботов можно стрелять.</b>\n".to_string();

    for u in chat.users.iter() {
        users_list.push_str(
            u.username.as_ref().unwrap_or(&u.first_name)
        );
        users_list.push_str(" - <b>");
        users_list.push_str(u.msg.to_string().as_str());
        users_list.push_str("</b> сообщений\n");
    }   

    users_list
}

pub fn get_buttons(chat_id: ChatId, offset: u32, count: u32) -> InlineKeyboardMarkup {
    let users_count = db::get_users_count(chat_id);
    let mut markup = InlineKeyboardMarkup::new();
    markup.add_empty_row();
    {
        let row = markup.add_empty_row();
        if offset > 0 {
            let mut text: String = "preview_".to_owned();
            text.push_str((offset / 8).to_string().as_str());
            row.push(InlineKeyboardButton::callback("<<", text));
        }

        if (offset + count) < users_count as u32 {
            let mut text: String = "forward_".to_owned();
            text.push_str(((offset + count)/8).to_string().as_str());
            row.push(InlineKeyboardButton::callback(">>", text));
        }
    }

    markup
}