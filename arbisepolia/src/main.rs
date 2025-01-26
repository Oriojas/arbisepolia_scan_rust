use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use dotenv::dotenv;

struct ArbiSepScanner {
    key: String,
    url: String,
}

impl ArbiSepScanner {
    /// Creates a new instance of `ArbiSepScanner` with the given API key and URL.
    fn new(key: String, url: String) -> Self {
        ArbiSepScanner { key, url }
    }

    /// Gets the transaction list for a given address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the account to query.
    ///
    /// # Return
    ///
    /// A `Result` containing a `Value` representing the transaction list, or an error if the request fails.
    fn transaction_address(&self, address: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "{}?module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=10&sort=asc&apikey={}",
            self.url, address, self.key
        );

        let response = Client::new().get(&url).send()?.json()?;
        Ok(response)
    }
}

/// Downloads all transactions from a specific contract address
/// and saves them to a JSON file at data/response.json.
fn main() {
    dotenv().ok();

    let key = env::var("KEY").expect("KEY not set");
    let url = env::var("ARBITRUM_URL").expect("SEPOLIA_URL not set");
    let address_contract = "0x8c5ff04497062be94e59412163a2e771a8154beb";

    let scanner = ArbiSepScanner::new(key, url);

    // Get transactions from a specific address
    match scanner.transaction_address(address_contract) {
        Ok(response) => {
            // Create the data directory if it doesn't exist
            let path = Path::new("data");
            if !path.exists() {
                fs::create_dir(path).expect("Failed to create data directory");
            }

            // Write the response to a JSON file
            let file_path = path.join(format!("{}.json", address_contract));
            fs::write(&file_path, serde_json::to_string_pretty(&response).expect("Failed to serialize JSON"))
                .expect("Failed to write to file");

            // Print the file name
            println!(
                "Response saved to {}",
                file_path.to_str().expect("Failed to convert path to string")
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}