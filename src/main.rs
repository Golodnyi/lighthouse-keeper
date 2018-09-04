extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate humantime;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
extern crate reader;
extern crate chrono;

use humantime::format_duration;
use serde_json::*;
use std::env;
use chrono::*;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Chat {
    id: ChatId,
    users: Vec<User>
}

#[derive(Serialize, Deserialize)]
struct User {
    id: UserId,
    username: Option<String>,
    date: i64,
}

enum Command {
    List
}

fn get_command(message: &str, bot_name: &str) -> Option<Command> {
    use Command::*;
    
    if !message.starts_with("/") {
        return None;
    }

    let mut cmd = message.clone();

    if cmd.ends_with(bot_name) {
        cmd = cmd.rsplitn(2, '@').skip(1).next().unwrap();
    }

    match cmd {
        "/list" => Some(List),
        _ => None,
    }
}

pub fn get_unix_timestamp() -> i64 {
    let now = Utc::now();
    let seconds: i64 = now.timestamp();

    seconds
}

fn main() {
    let mut core = Core::new().unwrap();

    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) =>
            panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing! {}", e),
    };

    let api = Api::configure(token).build(core.handle()).unwrap();
    
    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            if ChatId::new(0) < message.chat.id() {
                api.spawn(message.text_reply(
                    format!("Морти! Я работую только в групповых чатах!")
                ));

                return Ok(())
            }
            
            let mut chat: Chat;
            let user = User {
                id: message.from.id,
                username: message.from.username.to_owned(),
                date: message.date
            };
        

            match reader::read_file(message.chat.id().to_string()) {
                Ok(data) => {
                    chat = serde_json::from_str(&data.as_str()).unwrap();
                },
                Err(_e) => {
                    chat = Chat {
                        id: message.chat.id(),
                        users: vec![]
                    }
                }
            };

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
     
            if let MessageKind::Text {ref data, ..} = message.kind {
                let command = get_command(data, "lighthouseKeeperBot");

                let morty = User {
                    id: UserId::new(0),
                    username: Some("`Морти`".to_owned()),
                    date: (self::get_unix_timestamp() - (86400 * 7)) + 1
                };
                chat.users.push(morty);
                command.map(|cmd| match cmd {
                    Command::List => {
                        chat.users.sort_by_key(|k| k.date);
                        let mut users_list: String = "`Это всего лишь роботы, Морти! В роботов можно стрелять.`\n".to_string();
                        for u in chat.users.iter().rev() {
                            users_list.push_str(
                                u.username.as_ref().unwrap()
                            );
                            users_list.push_str(" - `");
                            let ago = Duration::new(((self::get_unix_timestamp() + 1) - u.date) as u64, 0);
                            users_list.push_str(format_duration(ago).to_string().as_str());
                            users_list.push_str("`\n");
                        }
                        api.spawn(message.text_reply(users_list).parse_mode(ParseMode::Markdown));
                    }
                });
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}