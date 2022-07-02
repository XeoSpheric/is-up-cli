use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub domain: String,
  pub status_code: u32,
}

pub fn is_up(domain: String) -> Result<Response, serde_json::Error> {
  let result = reqwest::blocking::get(&format!("https://isitup.org/{}.json", domain)).unwrap().text().unwrap();
  return serde_json::from_str(&result);
}