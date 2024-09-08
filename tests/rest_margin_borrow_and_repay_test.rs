use birdie::{
    margin::borrow_and_repay::{
        GetFutureHourlyInterestRateParams, GetInterestHistoryParams, QueryBorrowRepayRecordsParams,
        QueryMarginInterestRateHistoryParams, QueryMaxBorrowParams,
    },
    rest_api::Endpoint,
};

mod common;

#[tokio::test]
async fn rest_get_future_hourly_interest_rate() {
    let birdie = common::setup();
    let params = GetFutureHourlyInterestRateParams::new("BTCUSDT", "FALSE");
    let resp = birdie
        .rest_api()
        .margin()
        .borrow_and_repay()
        .get_future_hourly_interest_rate()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_interest_history() {
    let birdie = common::setup();
    let params = GetInterestHistoryParams::new();
    let resp = birdie
        .rest_api()
        .margin()
        .borrow_and_repay()
        .get_interest_history()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_borrow_repay_records() {
    let birdie = common::setup();
    let params = QueryBorrowRepayRecordsParams::new("BORROW");
    let resp = birdie
        .rest_api()
        .margin()
        .borrow_and_repay()
        .query_borrow_repay_records()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_margin_interest_rate_history() {
    let birdie = common::setup();
    let params = QueryMarginInterestRateHistoryParams::new("BTC");
    let resp = birdie
        .rest_api()
        .margin()
        .borrow_and_repay()
        .query_margin_interest_rate_history()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_max_borrow() {
    let birdie = common::setup();
    let params = QueryMaxBorrowParams::new("BTC");
    let resp = birdie
        .rest_api()
        .margin()
        .borrow_and_repay()
        .query_max_borrow()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
