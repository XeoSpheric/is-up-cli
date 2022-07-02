use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
  domain: String,
  port: u16,
  status_code: u16,
  response_ip: String,
  response_code: u16,
  response_time: f32
}

pub fn is_up(domain: String) -> String {
  let result = reqwest::blocking::get(&format!("https://isitup.org/{}.json", domain)).unwrap();
  let res = result.text().unwrap();
  let v: Response = serde_json::from_str(&res).unwrap();
  if v.status_code == 1 {
    return format!("{} is up ✔", domain);
  }
  return format!("{} is down 𝙓", domain);
}