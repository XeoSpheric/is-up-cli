mod is_up;
use is_up::is_up;

fn main() {
    let domain = std::env::args().nth(1);
    if domain != None {
        let res: String = is_up(domain.unwrap());
        println!("{}", res);
    }
    println!("No domain provided");
}
