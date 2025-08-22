use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use chrono::Utc;

// definition of the pricing trait
trait Pricing {
    fn fetch_price(&mut self) -> Result<(), Box<dyn Error>>;
    fn save_to_file(&self) -> Result<(), Box<dyn Error>>;
}

// bitcoin template
struct Bitcoin {
    price: f64,
    last_updated: String,
}

// ethereum template
struct Ethereum {
    price: f64,
    last_updated: String,
}

// sp500 template
struct SP500 {
    price: f64,
    last_updated: String,
}

// the response structs for json
#[derive(Debug, Deserialize)]
struct CoinGeckoResponse {
    bitcoin: CoinData,
    ethereum: CoinData,
}

#[derive(Debug, Deserialize)]
struct CoinData {
    usd: f64,
    last_updated_at: u64,
}

// implementing bitcoin
impl Pricing for Bitcoin {
    // fetching the data
    fn fetch_price(&mut self) -> Result<(), Box<dyn Error>> {
        let url = "https://www.coindesk.com/price/bitcoin";
        let response: serde_json::Value = ureq::get(url).call();
        if let Some(price) = response["bitcoin"]["usd"].as_f64() {
            self.price = price;
        }

        if let Some(timestamp) = response["bitcoin"]["last_updated_at"].as_u64() {
            self.last_updated = format!("{}", timestamp);
        }

        Ok(())
    }

    // saving the data
    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("bitcoin_price.txt")?;
        writeln!(file, "Bitcoin Price: ${}", self.price)?;
        writeln!(file, "Last Updated: {}", self.last_updated)?;
        Ok(())
    }
}

// implementing ethereum
impl Pricing for Ethereum {
    // fetching the data
    fn fetch_price(&mut self) -> Result<(), Box<dyn Error>> {
        let url = "https://www.coindesk.com/price/ethereum";
        let response: serde_json::Value = ureq::get(url).call();
        if let Some(price) = response["ethereum"]["usd"].as_f64() {
            self.price = price;
        }
        if let Some(timestamp) = response["ethereum"]["last_updated_at"].as_u64() {
            self.last_updated = format!("{}", timestamp);
        }
        Ok(())
    }
    //saving the data
    fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("ethereum_price.txt")?;
        writeln!(file, "Ethereum Price: ${}", self.price)?;
        writeln!(file, "Last Updated: {}", self.last_updated)?;
        Ok(())
    }
}

// implementing sp500
impl Pricing for SP500 {
    fn fetch_price(&mut self) -> Result<(), Box<dyn Error>> {
        let url = "https://query2.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        let response: serde_json::Value = ureq::get(url).call();
        if let Some(price) = response["sp500"]["usd"].as_f64() {
            self.price = price;
        }
        if let Some(timestamp) = response["sp500"]["last_updated_at"].as_u64() {
            self.last_updated = format!("{}", timestamp);
        }
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create("sp500_price.txt")?;
        writeln!(file, "S&P 500 Price: ${}", self.price)?;
        writeln!(file, "Last Updated: {}", self.last_updated)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // start of getting the data [through the main function]
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin { price: 0.0, last_updated: String::new() }),
        Box::new(Ethereum { price: 0.0, last_updated: String::new() }),
        Box::new(SP500 { price: 0.0, last_updated: String::new() }),
    ];


    // definition of loop. since there's no end condition loop keeps going
    loop {
        for asset in assets.iter_mut() {
            asset.fetch_price()?;
            asset.save_to_file()?;
        }
        println!("Prices updated. Continuing without delay...");
        
    }
}