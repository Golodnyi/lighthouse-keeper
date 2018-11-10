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
    let copy_first_name = user.first_name.clone();
    connection.execute(
        "insert or replace into users (id, chat_id, username, first_name, date, msg) values (?1, ?2, ?3, ?4, ?5, (SELECT msg+1 FROM users WHERE chat_id = ?2 AND id = ?1 LIMIT 1));",
        &[&user.id.to_string(), &chat_id.to_string(), &user.username.unwrap_or(copy_first_name), &user.first_name, &user.date]
    ).unwrap();

    let timestamp = structs::get_unix_timestamp();
    connection.execute(
        "insert or replace into stats (chat_id, timestamp) values (?1, ?2);",
        &[&chat_id.to_string(), &timestamp]
    ).unwrap();

    connection.close().expect("connection not closed");

    true
}

pub fn get_users_count(chat_id: ChatId) -> usize {
    let connection = self::connect();
    let mut users: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT * FROM users WHERE chat_id = ?1").unwrap();
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
    }

    connection.close().expect("connection not closed");

    users.len()
}

pub fn get_users(chat_id: ChatId, offset: u32, count: u32) -> Vec<structs::User> {
    let connection = self::connect();
    let mut users: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 ORDER BY msg DESC LIMIT ?2, ?3").unwrap();
        let users_iter = stmt.query_map(&[&chat_id.to_string(), &offset, &count], |row| {
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

pub fn get_user_by_id(chat_id: ChatId, id: String) -> structs::User {
    let connection = self::connect();
    let mut user: structs::User = structs::User {
        id: UserId::new(0),
        username: Some("Сквонч".to_owned()),
        first_name: "Сквонч".to_owned(),
        date: 0,
        msg: 0
    };

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 AND id = ?2 LIMIT 1").unwrap();
  

        let users_iter = stmt.query_map(&[&chat_id.to_string(), &id], |row| {
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

pub fn get_user_by_username(chat_id: ChatId, username: String) -> structs::User {
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

pub fn left_user(chat_id: ChatId, user_id: UserId) {
    let connection = self::connect();
    connection.execute(
        "delete from users where chat_id = ?1 and id = ?2;",
        &[&chat_id.to_string(), &user_id.to_string()]
    ).unwrap();

    connection.close().expect("connection not closed");
}

pub fn leave_from_chat(chat_id: ChatId) {
    let connection = self::connect();
    connection.execute(
        "delete from users where chat_id = ?1;",
        &[&chat_id.to_string()]
    ).unwrap();

    connection.close().expect("connection not closed");
}

pub fn get_chats() -> Vec<String> {
    let connection = self::connect();
    let mut chats: Vec<String> = vec![];

    {
        let mut stmt = connection.prepare("SELECT chat_id FROM users GROUP BY chat_id ORDER BY chat_id ASC").unwrap();
        let chats_iter = stmt.query_map(&[], |row| {
            row.get(0)
        }).unwrap();

        for chat in chats_iter
        {
            chats.push(chat.unwrap());
        }
    }

    connection.close().expect("connection not closed");

    chats
}

pub fn get_all_users_count() -> usize {
    let connection = self::connect();
    let mut users: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT * FROM users GROUP BY id").unwrap();
        let users_iter = stmt.query_map(&[], |row| {
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

    users.len()
}

pub fn get_all_silent() -> Vec<structs::User> {
    let connection = self::connect();
    let mut silent: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE date <= ?1 ORDER BY username ASC").unwrap();
        let month_ago = structs::get_unix_timestamp() - 86400 * 30;
        let silent_iter = stmt.query_map(&[&month_ago], |row| {
            structs::User {
                id: UserId::new(row.get(0)),
                username: Some(row.get(1)),
                first_name: row.get(2),
                date: row.get(3),
                msg: row.get(4)
            }
        }).unwrap();

        for s in silent_iter
        {
            silent.push(s.unwrap());
        }
    }

    connection.close().expect("connection not closed");

    silent
}

pub fn get_silent(chat_id: &String) -> Vec<structs::User> {
    let connection = self::connect();
    let mut silent: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 AND date <= ?2 ORDER BY username ASC").unwrap();
        let month_ago = structs::get_unix_timestamp() - 86400 * 30;
        let silent_iter = stmt.query_map(&[chat_id, &month_ago], |row| {
            structs::User {
                id: UserId::new(row.get(0)),
                username: Some(row.get(1)),
                first_name: row.get(2),
                date: row.get(3),
                msg: row.get(4)
            }
        }).unwrap();

        for s in silent_iter
        {
            let user = s.unwrap();

            connection.execute(
                "UPDATE users SET warning = ?1 WHERE chat_id = ?2 AND id = ?3 AND warning = 0",
                &[&(structs::get_unix_timestamp() - 3600), chat_id, &user.id.to_string()]
            ).unwrap();

            silent.push(user);
        }
    }

    connection.close().expect("connection not closed");

    silent
}

pub fn get_silent_for_kick(chat_id: &String) -> Vec<structs::User> {
    let connection = self::connect();
    let mut silent: Vec<structs::User> = vec![];

    {
        let mut stmt = connection.prepare("SELECT id, username, first_name, date, msg FROM users WHERE chat_id = ?1 AND date <= ?2 AND warning <= ?3 AND warning > 0 ORDER BY username ASC").unwrap();
        let day_ago = structs::get_unix_timestamp() - 86400;
        let month_ago = structs::get_unix_timestamp() - 86400 * 30;
        let silent_iter = stmt.query_map(&[chat_id, &month_ago, &day_ago], |row| {
            structs::User {
                id: UserId::new(row.get(0)),
                username: Some(row.get(1)),
                first_name: row.get(2),
                date: row.get(3),
                msg: row.get(4)
            }
        }).unwrap();

        for s in silent_iter
        {
            let user = s.unwrap();

            let month_ago = structs::get_unix_timestamp() - 86400 * 30;

            connection.execute(
                "UPDATE users SET warning = 0 WHERE chat_id = ?1 AND id = ?2 AND date >= ?3 AND warning > 0",
                &[chat_id, &user.id.to_string(), &month_ago]
            ).unwrap();

            silent.push(user);
        }
    }

    connection.close().expect("connection not closed");

    silent
}

pub fn can_write_silent(chat_id: ChatId) -> bool {
    let mut exist = false;
    let mut can_write = false;
    let connection = self::connect();
    {
        let mut stmt = connection.prepare("SELECT last_check FROM info WHERE chat_id = ?1 LIMIT 1").unwrap();
        let day_ago = structs::get_unix_timestamp() - 86400;
        let silent_iter = stmt.query_map(&[&chat_id.to_string()], |row| {
            let data: i64 = row.get(0);
            exist = true;
            data
        }).unwrap();

        for s in silent_iter {
            if (s.unwrap_or(0) - day_ago) < 0 {
                can_write = true;
            }
        }
    }

    if can_write || !exist {
        connection.execute(
            "insert or replace into info (chat_id, last_check) values (?1, ?2);",
            &[&chat_id.to_string(), &structs::get_unix_timestamp()]
        ).unwrap();
        can_write = true;
    }
    connection.close().expect("connection not closed");

    can_write
}