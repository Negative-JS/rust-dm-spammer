
use chrono::Local;
use colored::Colorize;
use std::{fs, thread, time};
use std::collections::HashMap;
use std::path::{Path};
use std::io::{self, BufRead, stdin};
use std::process::exit;
use rand::{thread_rng};
use rand::seq::{SliceRandom};
use reqwest;
use http::header;

struct Logger {}
impl Logger {
    fn log(text: &str){
        println!("{} {}", Local::now().time().format("[%H:%M:%S]").to_string().red(), text.bright_purple());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    println!("{}", "========================== Discord DM Spammer ==========================".green());
    Logger::log("Проверьте, что файлы tokens.txt и proxies.txt не пустые.");
    Logger::log("Если вы не используете прокси то оставьте в proxies.txt \"NO PROXY\"");
    Logger::log("Нажмите ENTER для продолжения");
    stdin().read_line(&mut "".to_string()).expect("err");

    let binding = fs::read_to_string("message.txt").unwrap_or_else(
        |_e| {
            Logger::log(format!("Не удалось найти файл message.txt").as_str());
            exit(0)
        }
    );
    let message = binding.as_str();
    let tokens: Vec<_> = read_lines("tokens.txt").unwrap_or_else(
        |_e| {
            Logger::log(format!("Не удалось найти файл tokens.txt").as_str());
            exit(0)
        }
    ).map(|x| x.unwrap_or_else(|_e| panic!("канава"))).collect();
    let proxies: Vec<_> = read_lines("proxies.txt").unwrap_or_else(
        |_e| {
            Logger::log(format!("Не удалось найти файл proxies.txt").as_str());
            exit(0)
        }
    ).map(|x| x.unwrap()).collect();
    let rng = &mut thread_rng();
    let vals: Vec<_> = (0..tokens.len())
        .map(
            |_| proxies
                .clone()
                .choose(rng)
                .unwrap_or_else(|| {
                    Logger::log(format!("Прокси пустые").as_str());
                    exit(0)
                })
                .clone()
        )
        .collect();

    let iter = tokens.iter().zip(vals);


    std::thread::scope(|scope| {
        for (token, proxy) in iter {
            scope.spawn(move || {
                let thread = std::thread::current().id();
                Logger::log(format!("{:?} Начинаю работу с токеном {}", &thread, &token).as_str());

                let mut headers = header::HeaderMap::new();

                headers.insert("accept", header::HeaderValue::from_static("*/*"));
                headers.insert("accept-language", header::HeaderValue::from_static("en-US"));
                headers.insert("connection", header::HeaderValue::from_static("keep-alive"));
                headers.insert("connection", header::HeaderValue::from_static("keep-alive"));
                headers.insert("DNT", header::HeaderValue::from_static("1"));
                headers.insert("origin", header::HeaderValue::from_static("https://discord.com"));
                headers.insert("sec-fetch-dest", header::HeaderValue::from_static("empty"));
                headers.insert("sec-fetch-mode", header::HeaderValue::from_static("cors"));
                headers.insert("sec-fetch-site", header::HeaderValue::from_static("same-origin"));
                headers.insert("referer", header::HeaderValue::from_static("https://discord.com/channels/@me"));
                headers.insert("TE", header::HeaderValue::from_static("Trailers"));
                headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) discord/1.0.9004 Chrome/91.0.4472.164 Electron/13.6.6 Safari/537.36"));
                headers.insert("X-Super-Properties", header::HeaderValue::from_static("eyJvcyI6IldpbmRvd3MiLCJicm93c2VyIjoiRGlzY29yZCBDbGllbnQiLCJyZWxlYXNlX2NoYW5uZWwiOiJzdGFibGUiLCJjbGllbnRfdmVyc2lvbiI6IjEuMC45MDA0Iiwib3NfdmVyc2lvbiI6IjEwLjAuMTgzNjIiLCJvc19hcmNoIjoieDY0Iiwic3lzdGVtX2xvY2FsZSI6ImVuLVVTIiwiY2xpZW50X2J1aWxkX251bWJlciI6MTE4MjA1LCJjbGllbnRfZXZlbnRfc291cmNlIjpudWxsfQ=="));
                headers.insert("Authorization", header::HeaderValue::from_str(&token).unwrap_or_else(
                    |_e| header::HeaderValue::from_static("None")
                ));

                let builder = reqwest::blocking::Client::builder()
                    .default_headers(
                        headers
                    );
                let client = match &proxy != "NO PROXY" {
                     true => builder
                        .proxy(reqwest::Proxy::https(proxy)
                            .unwrap_or_else(|_e| {
                                Logger::log(format!("{:?} Неверный формат прокси", &thread).as_str());
                                std::process::exit(0)
                            })
                        )
                        .build()
                        .unwrap_or_else(|_e| {
                            Logger::log(format!("{:?} Клиентская ошибка (канава)", &thread).as_str());
                            exit(1)
                        }),
                    false => builder
                        .build()
                        .unwrap_or_else(|_e| {
                            Logger::log(format!("{:?} Клиентская ошибка (канава)", &thread).as_str());
                            exit(1)
                        })
                };


                let me = client.get("https://discord.com/api/users/@me")
                    .send()
                    .unwrap_or_else(|_e| {
                        Logger::log(format!("{:?} Невалидный токен {} (канава)", &thread, &token).as_str());
                        Logger::log(format!("{:?} {:?} (канава)", &thread, _e).as_str());
                        exit(1)
                    })
                    .json::<HashMap<String, serde_json::Value>>()
                    .unwrap_or_else(|_e| {
                        Logger::log(format!("{:?} Клиентская ошибка (канава)", &thread).as_str());
                        exit(1)
                    });
                if me.contains_key("message") {
                    Logger::log(format!("{:?} Невалидный токен {}", &thread, &token).as_str());
                    return;
                }
                let channels = client.get("https://discord.com/api/users/@me/channels")
                    .send()
                    .unwrap_or_else(|_e| {
                        Logger::log(format!("{:?} Невалидный токен {} (канава)", &thread, &token).as_str());
                        exit(1)
                    })
                    .json::<Vec<HashMap<String, serde_json::Value>>>()
                    .unwrap_or_else(|_e| {
                        Logger::log(format!("{:?} Клиентская ошибка (канава)", &thread).as_str());
                        exit(1)
                    });


                Logger::log(format!("{:?} Запускаю общение с токеном {}#{} ({} каналов)", &thread, &me.get("username").unwrap().as_str().unwrap(), &me.get("discriminator").unwrap().as_str().unwrap(), &channels.len()).as_str());
                for channel in channels {
                    let id = match channel.get("id") {
                        Some(res) => res,
                        None => continue
                    }.as_str().expect("канал в канаву");
                    let mut map = HashMap::new();
                    map.insert("content", format!("{}", &message));
                    let _x = client.post(format!("https://discord.com/api/channels/{}/messages", id))
                        .json(&map)
                        .send()
                        .unwrap_or_else(|_e| {
                            Logger::log(format!("{:?} Клиентская ошибка (канава)", &thread).as_str());
                            exit(1)
                        });
                    thread::sleep(time::Duration::from_secs(1));
                }
                Logger::log(format!("{:?} Завершил общение с токеном {}#{}", &thread, &me.get("username").unwrap().as_str().unwrap(), &me.get("discriminator").unwrap().as_str().unwrap()).as_str());
            });
        }
    });


}
