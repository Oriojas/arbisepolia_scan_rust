use dotenv::dotenv;
use std::env;

fn main() {

    dotenv().ok();

    println!("Abitrum URL: {}", env::var("ARBITRUM_URL").unwrap());
}
