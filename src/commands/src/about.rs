extern crate structs;
extern crate db;

pub fn get() -> (usize, usize, usize) {
    let chats = db::get_chats();
    let users_count = db::get_all_users_count();
    let silents = db::get_all_silent();

    (chats.len(), users_count, silents.len())
}
