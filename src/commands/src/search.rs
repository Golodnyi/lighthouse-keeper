extern crate structs;
extern crate telegram_bot;
extern crate humantime;
extern crate db;

use self::telegram_bot::*;
use self::humantime::format_duration;
use std::time::Duration;

pub fn get_buttons(chat_id: ChatId, offset: u32, count: u32) -> InlineKeyboardMarkup {   
    let users = db::get_users(chat_id, offset, count);
    let users_count = db::get_users_count(chat_id);
    let mut markup = InlineKeyboardMarkup::new();
    markup.add_empty_row();
    {
        for user in users {
            let row = markup.add_empty_row();
            row.push(InlineKeyboardButton::callback(user.username.unwrap_or(user.first_name), user.id.to_string()));
        }

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

pub fn get_message(chat_id: ChatId, id: String) -> String {
    let user = db::get_user_by_id(chat_id, id);

    if user.id == UserId::new(0) {
        return "Морти, я понятия не имею о ком ты говоришь!".to_owned()
    }

    let ago = Duration::new(((structs::get_unix_timestamp() + 1) - user.date) as u64, 0);
    let mut answer: String = "Морти, кажется я видел <b>@".to_owned();
    answer.push_str(
        user.username.as_ref().unwrap_or(&user.first_name)
    );
    answer.push_str("</b> ");
    answer.push_str(format_duration(ago).to_string().as_str());
    answer.push_str(" назад");

    answer
}
