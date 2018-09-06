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
        "insert or replace into users (id, chat_id, username, first_name, date, msg) values (?1, ?2, ?3, ?4, ?5, (SELECT msg+1 FROM users WHERE chat_id = ?2 AND id = ?1 LIMIT 1));",
        &[&user.id.to_string(), &chat_id.to_string(), &user.username.unwrap_or("Сквонч".to_owned()), &user.first_name, &user.date]
    ).unwrap();

    connection.close().expect("connection not closed");

    true
}

pub fn get_users(chat_id: ChatId, order_field: String) -> Vec<structs::User> {
    let connection = self::connect();
    let mut users: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 ORDER BY ?2 DESC").unwrap();

        let users_iter = stmt.query_map(&[&chat_id.to_string(), &order_field], |row| {
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
    }

    connection.close().expect("connection not closed");

    users
}

pub fn get_user(chat_id: ChatId, username: String) -> structs::User {
    let connection = self::connect();
    let mut user: structs::User = structs::User {
        id: UserId::new(0),
        username: Some("Сквонч".to_owned()),
        first_name: "Сквонч".to_owned(),
        date: 0,
        msg: 0
    };

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 AND UPPER(username) = ?2 LIMIT 1").unwrap();
  

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

    }

    connection.close().expect("connection not closed");

    user
}