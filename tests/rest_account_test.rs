use birdie::{
    errors::{BinanceError, BinanceErrorCode},
    rest_api::{Endpoint, RestApiError},
    spot::account::{
        AccountInformationParams, AccountTradeListParams, QueryAllocationsParams,
        QueryCommissionRatesParams, QueryPreventedMatchesParams, QueryUnfilledOrderCountParams,
    },
};

mod common;

#[tokio::test]
async fn rest_account_information() {
    let client = common::setup_rest_api_client();
    let params = AccountInformationParams::new().omit_zero_balances(true);
    let resp = client.account().account_information().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_account_trade_list() {
    let client = common::setup_rest_api_client();
    let params = AccountTradeListParams::new("BTCUSDT");
    let resp = client.account().account_trade_list().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_unfilled_order_count() {
    let client = common::setup_rest_api_client();
    let params = QueryUnfilledOrderCountParams::new();
    let resp = client
        .account()
        .query_unfilled_order_count()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_prevented_matches() {
    let client = common::setup_rest_api_client();
    let params = QueryPreventedMatchesParams::new("BTCUSDT").order_id(1);
    let resp = client
        .account()
        .query_prevented_matches()
        .request(params)
        .await;
    assert!(resp.is_ok());

    // missing param
    let params = QueryPreventedMatchesParams::new("BTCUSDT");
    let resp = client
        .account()
        .query_prevented_matches()
        .request(params)
        .await;
    assert!(resp.is_err());
    let RestApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400 Bad Request");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}

#[tokio::test]
async fn rest_query_allocations() {
    let client = common::setup_rest_api_client();
    let params = QueryAllocationsParams::new("BTCUSDT");
    let resp = client.account().query_allocations().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_query_commission_rates() {
    let client = common::setup_rest_api_client();
    let params = QueryCommissionRatesParams::new("BTCUSDT");
    let resp = client
        .account()
        .query_commission_rates()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
