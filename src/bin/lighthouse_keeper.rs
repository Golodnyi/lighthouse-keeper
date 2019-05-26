extern crate futures;
extern crate telegram_bot;
extern crate telegram_bot_raw;
extern crate tokio_core;
extern crate structs;
extern crate commands;
extern crate db;

use std::env;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use commands::*;
use std::thread;
use std::time::Duration;
use telegram_bot_raw::types::*;

enum Command {
    Search,
    Messages,
    Horoscope,
    About,
    Unknown
}

fn get_command(message: &str, bot_name: &str) -> Option<Command> {
    use Command::*;
    
    if !message.starts_with("/") {
        return Some(Unknown);
    }

    let mut cmd: Vec<&str> = message.split(' ').collect();

    if cmd[0].ends_with(bot_name) {
        cmd = cmd[0].split('@').collect();
    }

    match cmd[0] {
        "/search" => Some(Search),
        "/messages" => Some(Messages),
        "/horoscope" => Some(Horoscope),
        "/about" => Some(About),
        _ => Some(Unknown),
    }
}

fn parse_user(message: &Message) -> structs::User {
    structs::User {
        id: message.from.id,
        username: message.from.username.to_owned(),
        date: message.date,
        first_name: message.from.first_name.to_owned(),
        msg: 0
    }
}

fn init() -> (Core, Api) {
    let core = Core::new().unwrap();

    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) =>
            panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing! {}", e),
    };

    let api = Api::configure(token).build(core.handle()).unwrap();

    (core, api)
}

fn main() {
    println!("start {:?}", structs::get_date());
    let (mut core, api) = self::init();
    let future = api.send(GetMe);
    let bot = core.run(future);
    let mut bot_id: UserId = UserId::new(0);

    match bot {
        Ok(ref data) => {
            bot_id = data.id;
        },
        Err(ref _e) => {}
    }    
    thread::spawn(move || {
        let (mut core_thread, api_thread) = self::init();

        loop {
            db::delete_old_stats();

            let silent = silent::get();
            let silent_for_kick = silent::get_for_kick();

            for s in silent {
                let mut count_users = 0;
                let mut message: String = "Начнем судную ночь, я определил участников, у них есть ~24 часа чтоб подать признаки жизни:\n".to_string();
                let chat_id = ChatId::new(s.chat_id.parse::<i64>().unwrap_or(0));

                for u in s.users {
                    let chat_member = api_thread.send(GetChatMember::new(chat_id, &u.id));
                    match core_thread.run(chat_member) {
                        Ok(data) => {
                            if data.status != ChatMemberStatus::Member {
                                continue;
                            }
                        },
                        Err(_e) => {
                            db::left_user(chat_id, u.id);
                            continue;
                        }
                    }

                    count_users += 1;
                    message.push_str("@");
                    message.push_str(u.username.as_ref().unwrap_or(&u.first_name));
                    message.push_str(" ");
                }

                message.push_str("\nВы можете бежать, но вам не спрятаться, сска!");

                if bot_id != UserId::new(0) {
                    let chat_member = api_thread.send(GetChatMember::new(chat_id, &bot_id));
                    match core_thread.run(chat_member) {
                        Ok(data) => {
                            if data.status != ChatMemberStatus::Administrator {
                                continue;
                            }
                        },
                        Err(_e) => {
                            continue;
                        }
                    }
                } else {
                    continue;
                }

                if db::can_write_silent(chat_id) == false {
                    continue;
                }

                if chat_id != ChatId::new(0) && count_users > 0 {
                    let send = api_thread.send(chat_id.text(message));
                    core_thread.run(send).unwrap();
                } else {
                    for se in silent_for_kick {
                        let mut message: String = "Судная ночь начата сска:\n".to_string();
                        let chat_id = ChatId::new(se.chat_id.parse::<i64>().unwrap_or(0));
                        let mut count_users = 0;

                        if bot_id != UserId::new(0) {
                            let chat_member = api_thread.send(GetChatMember::new(chat_id, &bot_id));
                            match core_thread.run(chat_member) {
                                Ok(data) => {
                                    if data.status != ChatMemberStatus::Administrator {
                                        continue;
                                    }
                                },
                                Err(_e) => {
                                    continue;
                                }
                            }
                        } else {
                            continue;
                        }

                        for u in s.users {
                            let chat_member = api_thread.send(GetChatMember::new(chat_id, &u.id));
                            match core_thread.run(chat_member) {
                                Ok(data) => {
                                    if data.status != ChatMemberStatus::Member {
                                        continue;
                                    }
                                },
                                Err(_e) => {
                                    db::left_user(chat_id, u.id);
                                    continue;
                                }
                            }

                            count_users += 1;
                            message.push_str("@");
                            message.push_str(u.username.as_ref().unwrap_or(&u.first_name));
                            message.push_str(" - убит\n");
                            let kick = api_thread.send(KickChatMember::new(chat_id, &u.id));
                            core_thread.run(kick).unwrap();
                            db::left_user(chat_id, u.id);
                        }

                        if chat_id != ChatId::new(0) && count_users > 0 {
                            let send = api_thread.send(chat_id.text(message));
                            core_thread.run(send).unwrap();
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(43200000));
        }
    });

    let future = api.stream().for_each(|update| {
        if let UpdateKind::CallbackQuery(message) = &update.kind {
            let chat_id = message.message.chat.id();
            let msg = message.clone();
            let reply_message = *msg.message.reply_to_message.unwrap();
            let user_id = message.data.to_owned();
            let text = search::get_message(chat_id, user_id);
            match reply_message {
                MessageOrChannelPost::Message(msg) => {
                    if let MessageKind::Text { ref data, .. } = msg.kind {
                        let command = get_command(data, "lighthouseKeeperBot");
                        command.map(|cmd| match cmd {
                            Command::Search => {
                                if message.data.starts_with("forward") {
                                    let params: Vec<&str> = message.data.split('_').collect();
                                    let offset: i32 = params[1].parse::<i32>().unwrap();
                                    let text = format!("Выберите пользователя");
                                    let markup = search::get_buttons(chat_id, 8 * offset as u32, 8);
                                    api.spawn(message.message.edit_text(text).reply_markup(markup));
                                } else if message.data.starts_with("preview").to_owned() {
                                    let params: Vec<&str> = message.data.split('_').collect();
                                    let offset: i32 = params[1].parse::<i32>().unwrap();
                                    let text = format!("Выберите пользователя");
                                    let markup = search::get_buttons(chat_id, 8 * offset as u32 - 8, 8);
                                    api.spawn(message.message.edit_text(text).reply_markup(markup));
                                } else {
                                    api.spawn(message.message.edit_text(text).parse_mode(ParseMode::Html));
                                }
                            },
                            Command::Messages => {
                                if message.data.starts_with("forward") {
                                    let params: Vec<&str> = message.data.split('_').collect();
                                    let offset: i32 = params[1].parse::<i32>().unwrap();
                                    let markup = messages::get_buttons(chat_id, 8 * offset as u32, 8);
                                    let text = messages::get(chat_id, offset as u32 * 8, 8);
                                    api.spawn(message.message.edit_text(text).reply_markup(markup).parse_mode(ParseMode::Html));
                                } else if message.data.starts_with("preview").to_owned() {
                                    let params: Vec<&str> = message.data.split('_').collect();
                                    let offset: i32 = params[1].parse::<i32>().unwrap();
                                    let markup = messages::get_buttons(chat_id, 8 * offset as u32 - 8, 8);
                                    let text = messages::get(chat_id, offset as u32 * 8 - 8, 8);
                                    api.spawn(message.message.edit_text(text).reply_markup(markup).parse_mode(ParseMode::Html));
                                }
                            },
                            Command::Horoscope => {
                                let data: String = message.data.to_owned();
                                api.spawn(message.message.edit_text(horoscope::get(data)).parse_mode(ParseMode::Markdown));
                            }
                            Command::About => {

                            }
                            Command::Unknown => {
                            }
                        });
                    }
                },
                MessageOrChannelPost::ChannelPost(_msg) => {

                }
            }
            
        }

        if let UpdateKind::Message(message) = update.kind {
            let chat_id = message.chat.id();

            if ChatId::new(0) < chat_id {
                api.spawn(message.text_reply(
                    format!("Морти! Я работую только в групповых чатах!\nВ основном я занимаюсь подсчетом количества сообщений пользователя и фиксацией даты последней активности, но, если мне дать права администратора (с функцией кика), я смогу выгонять из чата всех, кто молчал больше месяца. Я называю это судная ночь! Вопросы по работе бота к @Golodnyi")
                ));

                return Ok(())
            }

            if let MessageKind::NewChatMembers {ref data, ..} = message.kind {
                match bot {
                    Ok(ref b) => {
                        for user in data {
                            let u = structs::User {
                                id: user.id,
                                username: user.username.to_owned(),
                                date: structs::get_unix_timestamp(),
                                first_name:user.first_name.to_owned(),
                                msg: 0
                            };

                            if b.id != user.id && user.is_bot.unwrap_or(false) == false {
                                db::set_user(chat_id, u);
                            }
                        }
                    },
                    Err(_) => {}
                }
            }

            if let MessageKind::LeftChatMember{ref data, ..} = message.kind {
                match bot {
                    Ok(ref b) => {
                        if b.id == data.id {
                            db::leave_from_chat(chat_id);
                        } else {
                            db::left_user(chat_id, data.id);
                        }
                    },
                    Err(_) => {}
                }
            }

            if let MessageKind::Audio {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Document {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Photo {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Sticker {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Video {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Voice {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::VideoNote {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Contact {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Location {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Venue {data: _, ..} = message.kind {
                let user = self::parse_user(&message);
                db::set_user(chat_id, user);
            }

            if let MessageKind::Text {ref data, ..} = message.kind {
                let user = self::parse_user(&message);

                let command = get_command(data, "lighthouseKeeperBot");
                command.map(|cmd| match cmd {
                    Command::Search => {
                        let text = format!("Выберите пользователя");
                        let mut message = message.text_reply(text);
                        let markup = search::get_buttons(chat_id, 0, 8);
                        message.reply_markup(markup);
                        api.spawn(message);
                    },
                    Command::Messages => {
                        let markup = messages::get_buttons(chat_id, 0, 8);
                        api.spawn(message.text_reply(messages::get(chat_id, 0, 8)).reply_markup(markup).parse_mode(ParseMode::Html));
                    },
                    Command::Horoscope => {
                        let markup = horoscope::get_buttons();
                        api.spawn(message.text_reply("Морти, кто ты по гороскопу?").reply_markup(markup).parse_mode(ParseMode::Html));
                    },
                    Command::About => {
                        let data = about::get();
                        let mut msg: String = "Всего чатов: ".to_string();
                        msg.push_str(&data.0.to_string());
                        msg.push_str("\n");
                        msg.push_str("Уникальных пользователей: ");
                        msg.push_str(&data.1.to_string());
                        msg.push_str("\n");
                        msg.push_str("Участников судной ночи: ");
                        msg.push_str(&data.2.to_string());
                        api.spawn(message.text_reply(msg).parse_mode(ParseMode::Html));
                    }
                    Command::Unknown => {
                        db::set_user(chat_id, user);
                    }
                });
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}