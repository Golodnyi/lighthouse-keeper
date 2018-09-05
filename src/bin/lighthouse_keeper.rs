extern crate serde;
extern crate serde_derive;
extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
extern crate reader;
extern crate structs;
extern crate commands;

use serde_json::*;
use std::env;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use commands::*;

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
            
            let mut chat: structs::Chat;
            let user = structs::User {
                id: message.from.id,
                username: message.from.username.to_owned(),
                date: message.date
            };
        
            match reader::read_file(message.chat.id().to_string()) {
                Ok(data) => {
                    chat = serde_json::from_str(&data.as_str()).unwrap();
                },
                Err(_e) => {
                    chat = structs::Chat {
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

                let morty = structs::User {
                    id: UserId::new(0),
                    username: Some("<b>Морти</b>".to_owned()),
                    date: (structs::get_unix_timestamp() - (86400 * 7)) + 1
                };
                
                chat.users.push(morty);

                command.map(|cmd| match cmd {
                    Command::List => {
                        api.spawn(message.text_reply(list::get(chat)).parse_mode(ParseMode::Html));
                    }
                });
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}