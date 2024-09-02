use birdie::{errors::{BinanceError, BinanceErrorCode}, rest::{
    account::{AccountInformationParams, AccountTradeListParams, QueryAllocationsParams, QueryPreventedMatchesParams, QueryUnfilledOrderCountParams},
    Endpoint, RestError,
}};

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

#[tokio::test]
async fn query_unfilled_order_count() {
    let birdie = common::setup();
    let params = QueryUnfilledOrderCountParams::new();
    let resp = birdie
        .rest()
        .account()
        .query_unfilled_order_count()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn query_prevented_matches() {
    let birdie = common::setup();
    let params = QueryPreventedMatchesParams::new("BTCUSDT").order_id(1);
    let resp = birdie
        .rest()
        .account()
        .query_prevented_matches()
        .request(params)
        .await;
    assert!(resp.is_ok());

    // missing param
    let params = QueryPreventedMatchesParams::new("BTCUSDT");
    let resp = birdie
        .rest()
        .account()
        .query_prevented_matches()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}

#[tokio::test]
async fn query_allocations() {
    let birdie = common::setup();
    let params = QueryAllocationsParams::new("BTCUSDT");
    let resp = birdie
        .rest()
        .account()
        .query_allocations()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
