extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate telegram_bot;

use self::telegram_bot::*;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Horoscope {
    data: HoroscopeItem
}

#[derive(Deserialize, Debug)]
struct HoroscopeItem {
    period: String,
    reports: Vec<Report>
}

#[derive(Deserialize, Debug)]
struct Report {
    id: String,
    period: String,
    text: String,
    sign: u8,
    numbers: Vec<u8>,
    stats: Stat
}

#[derive(Deserialize, Debug)]
struct Stat {
    health: u8,
    love: u8,
    success: u8
}

struct HoroscopeValues {
    name: String,
    code: u8 
}

fn get_horoscope_vector() -> Vec<HoroscopeValues> {
    let mut horoscope_values: Vec<HoroscopeValues> = vec![];
    horoscope_values.push(HoroscopeValues {name: "Овен".to_owned(), code: 1});
    horoscope_values.push(HoroscopeValues {name: "Телец".to_owned(), code: 2});
    horoscope_values.push(HoroscopeValues {name: "Близнецы".to_owned(), code: 3});
    horoscope_values.push(HoroscopeValues {name: "Рак".to_owned(), code: 4});
    horoscope_values.push(HoroscopeValues {name: "Лев".to_owned(), code: 5});
    horoscope_values.push(HoroscopeValues {name: "Дева".to_owned(), code: 6});
    horoscope_values.push(HoroscopeValues {name: "Весы".to_owned(), code: 7});
    horoscope_values.push(HoroscopeValues {name: "Скорпион".to_owned(), code: 8});
    horoscope_values.push(HoroscopeValues {name: "Стрелец".to_owned(), code: 9});
    horoscope_values.push(HoroscopeValues {name: "Козерог".to_owned(), code: 10});
    horoscope_values.push(HoroscopeValues {name: "Водолей".to_owned(), code: 11});
    horoscope_values.push(HoroscopeValues {name: "Рыбы".to_owned(), code: 12});

    horoscope_values
}

pub fn get_buttons() -> InlineKeyboardMarkup {
    let horoscope_values = get_horoscope_vector();

    let mut markup = InlineKeyboardMarkup::new();
    markup.add_empty_row();
    {
        for i in [0, 3, 6, 9].iter() {
            let row = markup.add_empty_row();
            row.push(InlineKeyboardButton::callback(&horoscope_values[*i as usize].name, horoscope_values[*i as usize].code.to_string()));
            row.push(InlineKeyboardButton::callback(&horoscope_values[(*i + 1) as usize].name, horoscope_values[(*i + 1) as usize].code.to_string()));
            row.push(InlineKeyboardButton::callback(&horoscope_values[(*i + 2) as usize].name, horoscope_values[(*i + 2) as usize].code.to_string()));
        }
    }

    markup
}

pub fn get(sign: u8) -> String {
    let horoscope_values = get_horoscope_vector();
    let mut response = reqwest::get("https://horoscope.zborg.ru/api/reports.json?client=telegram.bot")
        .expect("Failed to send request");

    let mut text = String::new();
    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    let horoscope: Horoscope = serde_json::from_str(buf.as_str()).expect("Failed to parse json");

    for h in horoscope_values.iter() {
        if h.code == sign {
            text.push_str("*");
            text.push_str(h.name.as_str());
            text.push_str("*\n---\n");
        }
    }

    if horoscope.data.reports.len() == 0 {
        text.push_str("Сервис временно недоступен :(");
    }

    for h in horoscope.data.reports.iter() {
        if h.sign == sign {
            text.push_str("Здоровье: ");
            text.push_str(h.stats.health.to_string().as_str());
            text.push_str("%, ");

            text.push_str("любовь: ");
            text.push_str(h.stats.love.to_string().as_str());
            text.push_str("%, ");
            
            text.push_str("удача: ");
            text.push_str(h.stats.success.to_string().as_str());
            text.push_str("%.\n---\n");
            
            text.push_str("```\n");
            text.push_str(h.text.as_str());
            text.push_str("```");
        }
    }

    text
}