// Copyright (c) 2024 Yuki Kishimoto
// Distributed under the MIT software license

use lnurl_pay::{api, LightningAddress};

#[tokio::main]
async fn main() {
    let addr = LightningAddress::parse("yuki@getalby.com").unwrap();
    let invoice = api::get_invoice(addr, 1 * 1000, None, None).await.unwrap();
    println!("{invoice}");
}
