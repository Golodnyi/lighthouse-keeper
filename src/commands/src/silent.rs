extern crate structs;
extern crate db;

pub fn get() -> Vec<structs::Silent> {
    let chats = db::get_chats();
    let mut silent: Vec<structs::Silent> = vec![];

    for chat in chats {
        let users = db::get_silent(&chat);

        if users.len() > 0 {
            silent.push(
                structs::Silent {
                    chat_id: chat,
                    users: users
                }
            );
        }
    }

    silent
}

pub fn get_for_kick() -> Vec<structs::Silent> {
    let chats = db::get_chats();
    let mut silent: Vec<structs::Silent> = vec![];

    for chat in chats {
        let users = db::get_silent_for_kick(&chat);

        if users.len() > 0 {
            silent.push(
                structs::Silent {
                    chat_id: chat,
                    users: users
                }
            );
        }
    }

    silent
}