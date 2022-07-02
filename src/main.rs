mod is_up;
use is_up::{is_up, Response};
use std::env::args;

fn main() {
    let domain = args().nth(1);
    if domain != None {
        let res: Result<Response, serde_json::Error> = is_up(domain.unwrap());
        match res {
            Ok(res) => {
                if res.status_code == 1 {
                    println!("{} is up", res.domain);
                    return;
                }
                println!("{} is down", res.domain);
                return;

            },
            Err(err) => {
                println!("{}", err);
            }
        }
    }
    println!("No domain provided");
}
