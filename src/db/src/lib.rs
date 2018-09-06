extern crate rusqlite;
extern crate telegram_bot;
extern crate structs;

use rusqlite::Connection;
use telegram_bot::*;

fn connect() -> Connection {
    Connection::open("database.sqlite3").unwrap()
}

pub fn set_user(chat_id: ChatId, user: structs::User) -> bool {
    let connection = self::connect();
    connection.execute(
        "insert or replace into users (id, chat_id, username, first_name, date) values (?1, ?2, ?3, ?4, ?5);",
        &[&user.id.to_string(), &chat_id.to_string(), &user.username.unwrap_or("Сквонч".to_owned()), &user.first_name, &user.date]
    ).unwrap();

    connection.execute(
        "UPDATE users SET msg = msg + 1 WHERE id = ?1 AND chat_id = ?2;",
        &[&user.id.to_string(), &chat_id.to_string()]
    ).unwrap();

    true
}

pub fn get_users(chat_id: ChatId) -> Vec<structs::User> {
    let connection = self::connect();

    let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1").unwrap();
    let mut users: Vec<structs::User> = vec![];

    let users_iter = stmt.query_map(&[&chat_id.to_string()], |row| {
        structs::User {
            id: UserId::new(row.get(0)),
            username: Some(row.get(1)),
            first_name: row.get(2),
            date: row.get(3),
            msg: row.get(4)
        }
    }).unwrap();

    for user in users_iter
    {
        users.push(user.unwrap());
    }

    users
}

pub fn get_user(chat_id: ChatId, username: String) -> structs::User {
    let connection = self::connect();

    let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 AND UPPER(username) = ?2 LIMIT 1").unwrap();
    let mut user: structs::User = structs::User {
        id: UserId::new(0),
        username: Some("Сквонч".to_owned()),
        first_name: "Сквонч".to_owned(),
        date: 0,
        msg: 0
    };

    let users_iter = stmt.query_map(&[&chat_id.to_string(), &username.to_uppercase()], |row| {
        structs::User {
            id: UserId::new(row.get(0)),
            username: Some(row.get(1)),
            first_name: row.get(2),
            date: row.get(3),
            msg: row.get(4)
        }
    }).unwrap();

    for u in users_iter
    {
        user = u.unwrap();
    }

    user
}