use birdie::{
    margin::account::{
        GetBnbBurnStatusParams, GetSummaryOfMarginAccountParams,
        QueryCrossIsolatedMarginCapitalFlowParams, QueryCrossMarginAccountDetailsParams,
        QueryCrossMarginFeeDataParams, QueryEnabledIsolatedMarginAccountLimitParams,
        QueryIsolatedMarginAccountInfoParams, QueryIsolatedMarginFeeDataParams,
    },
    rest_api::Endpoint,
};

mod common;

#[tokio::test]
async fn rest_margin_get_bnb_burn_status() {
    let client = common::setup_rest_api_client();
    let params = GetBnbBurnStatusParams::new();
    let resp = client
        .margin()
        .account()
        .get_bnb_burn_status()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_get_summary_of_margin_account() {
    let client = common::setup_rest_api_client();
    let params = GetSummaryOfMarginAccountParams::new();
    let resp = client
        .margin()
        .account()
        .get_summary_of_margin_account()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_cross_margin_account_details() {
    let client = common::setup_rest_api_client();
    let params = QueryCrossMarginAccountDetailsParams::new();
    let resp = client
        .margin()
        .account()
        .query_cross_margin_account_details()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_cross_margin_fee_data() {
    let client = common::setup_rest_api_client();
    let params = QueryCrossMarginFeeDataParams::new();
    let resp = client
        .margin()
        .account()
        .query_cross_margin_fee_data()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_enabled_isolated_margin_account_limit() {
    let client = common::setup_rest_api_client();
    let params = QueryEnabledIsolatedMarginAccountLimitParams::new();
    let resp = client
        .margin()
        .account()
        .query_enabled_isolated_margin_account_limit()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_isolated_margin_account_info() {
    let client = common::setup_rest_api_client();
    let params = QueryIsolatedMarginAccountInfoParams::new();
    let resp = client
        .margin()
        .account()
        .query_isolated_margin_account_info()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_isolated_margin_fee_data() {
    let client = common::setup_rest_api_client();
    let params = QueryIsolatedMarginFeeDataParams::new();
    let resp = client
        .margin()
        .account()
        .query_isolated_margin_fee_data()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_isolated_margin_capital_flow() {
    let client = common::setup_rest_api_client();
    let params = QueryCrossIsolatedMarginCapitalFlowParams::new();
    let resp = client
        .margin()
        .account()
        .query_cross_isolated_margin_capital_flow()
        .request(params)
        .await;
    println!("{:?}", resp);
    assert!(resp.is_ok());
}
