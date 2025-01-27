use std::fs;
use std::env;
use dotenv::dotenv;
use std::path::Path;
use serde::Deserialize;
use serde_json::Value;
use reqwest::blocking::Client;

struct ArbiSepScanner {
    key: String,
    url: String,
}

impl ArbiSepScanner {
    fn new(key: String, url: String) -> Self {
        ArbiSepScanner { key, url }
    }

    fn transaction_address(&self, address: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "{}?module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=10&sort=asc&apikey={}",
            self.url, address, self.key
        );

        let response = Client::new().get(&url).send()?.json()?;
        Ok(response)
    }

    fn transaction_hash(&self, hash: &str) -> Result<Value, reqwest::Error> {
        let url = format!(
            "{}?module=account&action=txlistinternal&txhash={}&apikey={}",
            self.url, hash, self.key
        );

        let response = Client::new().get(&url).send()?.json()?;
        Ok(response)
    }
}

#[derive(Deserialize)]
struct Transaction {
    hash: String,
    // Otros campos que necesites
}

#[derive(Deserialize)]
struct Response {
    result: Vec<Transaction>,
}

fn main() {
    dotenv().ok();

    let key = env::var("KEY").expect("KEY not set");
    let url = env::var("ARBITRUM_URL").expect("ARBITRUM_URL not set");
    let address_contract = "0x346Ac3698f6a1Ed8B78C9594284406A4506d0d68";

    let scanner = ArbiSepScanner::new(key, url);

    // Obtener transacciones de una dirección específica
    match scanner.transaction_address(address_contract) {
        Ok(response) => {
            // Crear el directorio de datos si no existe
            let path = Path::new("data");
            if !path.exists() {
                fs::create_dir(path).expect("Failed to create data directory");
            }

            // Escribir la respuesta en un archivo JSON
            let file_path = path.join(format!("{}.json", address_contract));
            fs::write(&file_path, serde_json::to_string_pretty(&response).expect("Failed to serialize JSON"))
                .expect("Failed to write to file");

            // Imprimir el nombre del archivo
            println!(
                "Response saved to {}",
                file_path.to_str().expect("Failed to convert path to string")
            );

            // Leer el archivo JSON
            let data = fs::read_to_string(file_path).expect("Unable to read file");

            // Deserializar el JSON a la estructura Response
            let response: Response = serde_json::from_str(&data).expect("JSON was not well-formatted");

            // Iterar sobre los resultados y consultar cada hash
            for transaction in response.result {
                match scanner.transaction_hash(&transaction.hash) {
                    Ok(response) => {
                        println!("Response for hash {}: {}", transaction.hash, serde_json::to_string_pretty(&response).expect("Failed to serialize JSON"));
                    }
                    Err(e) => eprintln!("Error for hash {}: {}", transaction.hash, e),
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}