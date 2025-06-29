use reqwest::blocking::Client;
use dotenv::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod notify;

#[derive(Deserialize, Debug)]
struct Ticker {
    ticker: String,

    #[serde(rename = "currentShare")]
    current_share: f64,

    #[serde(rename = "expectedShare")]
    expected_share: f64,
}

#[derive(Deserialize, Debug)]
struct PieSettings {
    name: String,

    #[serde(rename = "creationDate")]
    creation_date: f64,
    id: u32,
}

#[derive(Deserialize, Debug)]
struct Pie {
    settings: PieSettings,
    instruments: Vec<Ticker>,
}

#[derive(Deserialize, Debug, Clone)]
struct ShortQuote {
    price: f64,
    // volume: u64,
}

#[derive(Deserialize, Debug, Clone)]
struct StockChange {

    #[serde(rename = "1D")]
    one_day: f64,

    // #[serde(rename = "5D")]
    // five_day: f64,

    // #[serde(rename = "1M")]
    // one_month: f64,

    // #[serde(rename = "3M")]
    // three_month: f64,

    // #[serde(rename = "6M")]
    // six_month: f64,

    // ytd: f64,
    // one_year: f64,
    // three_year: f64,
    // five_year: f64,
    // ten_year: f64,
    // max: f64,
}

struct StockInfo {
    symbol: String,
    short_quote: ShortQuote,
    change: StockChange,
}


fn fetch_email_details() -> (String, String) {

    let email = std::env::var("EMAIL").expect("EMAIL must be set.");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD must be set.");

    (email, password)

}

fn fetch_pie(id: u32) -> Pie{

    let api_key = std::env::var("TRADING_API_TOKEN").expect("TRADING_API_TOKEN must be set.");

    let url = format!("https://live.trading212.com/api/v0/equity/pies/{}", id);

    let client = Client::new();
    let response = client.get(&url)
        .header("Authorization", api_key)
        .send()
        .expect("Failed to fetch pie data");
    
    // Response text: "{\"instruments\":[{\"ticker\":\"MSFT_US_EQ\",\"result\":{\"priceAvgInvestedValue\":0.98,\"priceAvgValue\":1.00,\"priceAvgResult\":0.02,\"priceAvgResultCoef\":0.0204},\"expectedShare\":1.0000,\"currentShare\":1.0000,\"ownedQuantity\":0.0028064000,\"issues\":[]}],\"settings\":{\"id\":4667358,\"instrumentShares\":null,\"name\":\"Dynamic Pie\",\"icon\":\"Coins\",\"goal\":null,\"creationDate\":1750267596.000000000,\"endDate\":null,\"initialInvestment\":null,\"dividendCashAction\":\"REINVEST\",\"publicUrl\":null}}"

    if response.status().is_success() {
        let pie_data: Pie = response.json().expect("Failed to read response text");
        return pie_data;
    } else {
        panic!("Failed to fetch pie data: {}", response.status());
    }

}

fn fetch_ticker_info(ticker: &str) -> Result<StockInfo, String> {

    let api_key = std::env::var("FINANCIALMODELINGPREP_API_TOKEN").expect("FINANCIALMODELINGPREP_API_TOKEN must be set.");

    let url_sq = format!("https://financialmodelingprep.com/api/v3/quote-short/{}?apikey={}", ticker, api_key);

    let client_sq = Client::new();
    let response_sq = client_sq.get(&url_sq)
        .send()
        .expect("Failed to fetch ticker data");
    
    if !response_sq.status().is_success() {
        return Err(format!("Failed to fetch ticker data: {}", response_sq.status()));
    }

    let sq_vec = response_sq.json::<Vec<ShortQuote>>().expect("Failed to read response");
    if sq_vec.is_empty() {
        return Err(format!("No ShortQuote found for {}", ticker));
    }
    let sq = sq_vec.first();

    let url_sc = format!("https://financialmodelingprep.com/api/v3/stock-price-change/{}?apikey={}", ticker, api_key);

    let client_sc = Client::new();
    let response_sc = client_sc.get(&url_sc)
        .send()
        .expect("Failed to fetch stock change data");

    if !response_sc.status().is_success() {
        return Err(format!("Failed to fetch stock change data: {}", response_sc.status()));
    }

    let sc_vec = response_sc.json::<Vec<StockChange>>().expect("Failed to read response");
    if sc_vec.is_empty() {
        return Err(format!("No StockChange found for {}", ticker));
    }
    let sc = sc_vec.first();


    Ok(StockInfo {
        symbol: ticker.to_string(),
        short_quote: sq.cloned().unwrap(),
        change: sc.cloned().unwrap(),
    })

}

fn read_txt_to_dict() -> HashMap<String, String> {
    let file = File::open("./symbols.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if let Some((key, value)) = l.split_once(',') {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    map
}

type StockInfoResult = (Vec<StockInfo>, Vec<String>);
fn fetch_stock_info(tickers: Vec<&str>) -> StockInfoResult {
    let mut stock_infos = Vec::new();
    let mut stock_fails = Vec::new();

    for ticker in tickers {
        match fetch_ticker_info(ticker) {
            Ok(info) => stock_infos.push(info),
            Err(e) => {
                eprintln!("Failed to fetch info for {}: {}", ticker, e);
                stock_fails.push(ticker.to_string());
            }
        }
    }

    (stock_infos, stock_fails)
}

fn make_email(stock_info: Vec<StockInfo>, error_tickers: Vec<String>) -> String {
    let mut email_body = String::new();
    email_body.push_str("<h1>Stock Information</h1><ul>");

    for info in stock_info {
        let change = info.change.one_day;
        let color = if change >= 0.0 { "green" } else { "red" };
        email_body.push_str(&format!(
            "<li><strong>{}</strong>: Price: ${:.2} <span style=\"color:{}\">({:+.2}%)</span></li>",
            info.symbol,
            info.short_quote.price,
            color,
            change
        ));
    }

    email_body.push_str("</ul>");

    if !error_tickers.is_empty() {
        
        email_body.push_str("<h3>Failed to Fetch Stock Information</h3><ul>");

        for ticker in error_tickers {
            email_body.push_str(&format!("<li>{}</li>", ticker));
        }

        email_body.push_str("</ul>");
    }

    email_body
}

fn main() {

    dotenv().expect("Failed to load .env file");

    let convertor = read_txt_to_dict();

    let pie: Pie = fetch_pie(4667358);

    let tickers: Vec<&str> = pie
        .instruments
        .iter()
        .map(|i| {
            let base_ticker = i.ticker.split('_').next().unwrap_or(&i.ticker);
            if let Some(val) = convertor.get(base_ticker) {
                val.as_str()
            } else {
                base_ticker
            }
        })
        .collect();

    let (stock_infos, stock_fails) = fetch_stock_info(tickers);

    let email_body = make_email(stock_infos, stock_fails);

    let (email, password) = fetch_email_details();

    let res_email = notify::send_msg_blocking(&notify::Message {
        from_email: email.clone(),
        from_password: password.clone(),
        to: email.clone(),
        subject: "Daily Pie Email!".to_string(),
        html_body: email_body,
    });

    match res_email {
        Ok(_) => println!("Email sent successfully."),
        Err(e) => eprintln!("Failed to send email: {}", e),
    }
    
}
