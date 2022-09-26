use std::str::FromStr;

use comfy_table::{CellAlignment, Table};
use currency_rs::{Currency};

#[derive(Debug, Clone, Copy)]
struct TradeSize {
    pub account_equity: f64,
    risk_percent: f64,
    // usually 1 to 2%
    risk_equity: f64,
    // account_equity * risk_percent
    risk_per_unit: f64,
    // price - stop_loss
    price: f64,
    // current price
    stop_loss: f64,
    // number of shares to buy which is calculated
    num_shares: u32,
    // price to buy number of shares
    total_price: f64,
}

impl TradeSize {
    fn new(account_equity: f64, risk_percent: f64, price: f64, stop_loss: f64) -> TradeSize {
        let risk_equity = Currency::new_float(account_equity, None).multiply((risk_percent / 100.0) as f64).value();
        let risk_per_unit = Currency::new_float(price, None).subtract(stop_loss).value();
        let num_shares = (risk_equity / risk_per_unit) as u32;
        let total_price = Currency::new_float(price, None).multiply(num_shares as f64).value();
        TradeSize {
            account_equity,
            risk_percent,
            risk_equity,
            risk_per_unit,
            price,
            stop_loss,
            num_shares,
            total_price,
        }
    }

    fn per_unit_risk(&self) -> Currency<'static> {
        Currency::new_float(self.price, None).subtract(self.stop_loss)
    }

    fn from_args(account_equity: f64, price: f64, stop_loss: f64) -> TradeSize {
        TradeSize {
            account_equity,
            risk_percent: 0.0,
            risk_equity: 0.0,
            risk_per_unit: 0.0,
            price,
            stop_loss,
            num_shares: 0,
            total_price: 0.0,
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
    let trade_size = TradeSize::from_args(account_equity, price, stop_loss);

    input_table(trade_size);

    risk_table(trade_size);

    println!();
}

fn input_table(ts: TradeSize) {
    let mut input_table = Table::new();
    input_table
        .set_header(vec!["Equity", "Price", "Stop-loss", "Per-unit Risk"])
        .add_row(vec![
            Currency::new_float(ts.account_equity, None).format().as_str(),
            Currency::new_float(ts.price, None).format().as_str(),
            Currency::new_float(ts.stop_loss, None).format().as_str(),
            ts.per_unit_risk().format().as_str(),
        ])
    ;

    // right justify all columns in input_table
    for (_, column) in input_table.column_iter_mut().enumerate() {
        column.set_cell_alignment(CellAlignment::Right);
    }

    println!();
    println!("Inputs:");
    println!("{input_table}");
}

fn risk_table(ts: TradeSize) {
    let mut risk_table = Table::new();
    let mut risk_percent = 1.0;
    const MAX_RISK_PERCENT: f64 = 2.0;
    const RISK_INCREMENT: f64 = 0.25;
    while risk_percent <= MAX_RISK_PERCENT {
        let ts = TradeSize::new(ts.account_equity, risk_percent, ts.price, ts.stop_loss);
        risk_table.set_header(vec!["% Risk", "Shares", "Total Price"]);
        risk_table.add_row(vec![
            format!("{pct:.*}%", 2, pct = ts.risk_percent),
            ts.num_shares.to_string(),
            Currency::new_float(ts.total_price, None).format(),
        ]);
        risk_percent += RISK_INCREMENT;
    }

    // right justify all columns in risk_table
    for (_, column) in risk_table.column_iter_mut().enumerate() {
        column.set_cell_alignment(CellAlignment::Right);
    }

    println!();
    println!("Outputs:");
    println!("{risk_table}");
}
