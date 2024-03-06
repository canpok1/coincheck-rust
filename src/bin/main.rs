use coincheck_rust::client::{Client, DefaultClient};

#[tokio::main]
async fn main() {
    let access_key = "";
    let secret_key = "";
    let pair = "btc_jpy";

    let coincheck = DefaultClient::new(&access_key, &secret_key).unwrap();

    {
        let res = coincheck.get_ticker(&pair).await.unwrap();
        println!("get_ticker() => {:?}", res);
    }

    {
        let res = coincheck.get_order_books(&pair).await.unwrap();
        println!(
            "get_order_books() => asks:{:?}, bids:{:?}",
            res.asks, res.bids
        );
    }
}
