use std::str::FromStr;
use currency_rs::{Currency, CurrencyOpts};

#[derive(Debug)]
struct TradeSize {
    account_equity: f64,
    risk_percent: f64,
    // usually 1 to 2%
    risk_equity: f64,
    // account_equity * risk_percent
    risk_per_unit: f64,
    // price - stop_loss
    price: f64,
    // current price
    stop_loss: f64,
    size: u32, // number of shares to buy which is calculated
}

impl TradeSize {
    fn new(account_equity: f64, risk_percent: f64, price: f64, stop_loss: f64) -> TradeSize {
        let risk_equity = Currency::new_float(account_equity, None).multiply((risk_percent / 100.0) as f64).value();
        let risk_per_unit = Currency::new_float(price, None).subtract(stop_loss).value();
        let size = (risk_equity / risk_per_unit) as u32;
        TradeSize {
            account_equity,
            risk_percent,
            risk_equity,
            risk_per_unit,
            price,
            stop_loss,
            size,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.is_empty() || args.len() != 4 {
        println!();
        println!("Usage: tradesize [ACCOUNT_EQUITY] [PRICE] [STOP_LOSS]");
        std::process::exit(1);
    }

    let account_equity = f64::from_str(&args[1]).unwrap();
    let price = f64::from_str(&args[2]).unwrap();
    let stop_loss = f64::from_str(&args[3]).unwrap();
    let precision = 2;

    println!();
    println!("       Equity: {}", Currency::new_float(account_equity, None).format());
    println!("        Price: {}", Currency::new_float(price, None).format());
    println!("    Stop-loss: {}", Currency::new_float(stop_loss, None).format());
    println!("Per-unit Risk: {}", Currency::new_float(price, None).subtract(stop_loss).format());
    println!();
    let mut risk_percent = 1.0;
    while risk_percent <= 2.0 {
        let ts = TradeSize::new(account_equity, risk_percent, price, stop_loss);
        println!("   {pct:.*}% risk: {shares} shares", precision, pct=ts.risk_percent, shares=ts.size);
        risk_percent += 0.25;
    }
    println!();
}
