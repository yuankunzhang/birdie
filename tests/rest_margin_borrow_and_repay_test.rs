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
    let client = common::setup_rest_api_client();
    let params = GetFutureHourlyInterestRateParams::new("BTCUSDT", "FALSE");
    let resp = client
        .margin()
        .borrow_and_repay()
        .get_future_hourly_interest_rate()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_get_interest_history() {
    let client = common::setup_rest_api_client();
    let params = GetInterestHistoryParams::new();
    let resp = client
        .margin()
        .borrow_and_repay()
        .get_interest_history()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_borrow_repay_records() {
    let client = common::setup_rest_api_client();
    let params = QueryBorrowRepayRecordsParams::new("BORROW");
    let resp = client
        .margin()
        .borrow_and_repay()
        .query_borrow_repay_records()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_margin_interest_rate_history() {
    let client = common::setup_rest_api_client();
    let params = QueryMarginInterestRateHistoryParams::new("BTC");
    let resp = client
        .margin()
        .borrow_and_repay()
        .query_margin_interest_rate_history()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_max_borrow() {
    let client = common::setup_rest_api_client();
    let params = QueryMaxBorrowParams::new("BTC");
    let resp = client
        .margin()
        .borrow_and_repay()
        .query_max_borrow()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
