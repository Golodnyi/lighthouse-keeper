extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;
extern crate structs;
extern crate commands;

use std::env;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;
use commands::*;

enum Command {
    List,
    Search
}

fn get_command(message: &str, bot_name: &str) -> Option<Command> {
    use Command::*;
    
    if !message.starts_with("/") {
        return None;
    }

    let mut cmd: Vec<&str> = message.split(' ').collect();

    if cmd[0].ends_with(bot_name) {
        cmd = cmd[0].split('@').collect();
    }

    match cmd[0] {
        "/list" => Some(List),
        "/search" => Some(Search),
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
            let chat_id = message.chat.id();

            if ChatId::new(0) < chat_id {
                api.spawn(message.text_reply(
                    format!("Морти! Я работую только в групповых чатах!")
                ));

                return Ok(())
            }

            let user = structs::User {
                id: message.from.id,
                username: message.from.username.to_owned(),
                date: message.date
            };

            list::add_user(chat_id, user);

            if let MessageKind::Text {ref data, ..} = message.kind {
                let command = get_command(data, "lighthouseKeeperBot");
                command.map(|cmd| match cmd {
                    Command::List => {
                        api.spawn(message.text_reply(list::get(chat_id)).parse_mode(ParseMode::Html));
                    },
                    Command::Search => {
                        api.spawn(message.text_reply(search::get(chat_id, "lighthouseKeeperBot", data)).parse_mode(ParseMode::Html));
                    }
                });
            }
        }
        Ok(())
    });

    core.run(future).unwrap();
}