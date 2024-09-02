use birdie::rest::{
    account::{AccountInformationParams, AccountTradeListParams},
    Endpoint,
};

mod common;

#[tokio::test]
async fn account_information() {
    let birdie = common::setup();
    let params = AccountInformationParams::new().omit_zero_balances(true);
    let resp = birdie
        .rest()
        .account()
        .account_information()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn account_trade_list() {
    let birdie = common::setup();
    let params = AccountTradeListParams::new("BTCUSDT");
    let resp = birdie
        .rest()
        .account()
        .account_trade_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
