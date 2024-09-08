use birdie::{
    margin::trade::{
        GetForceLiquidationRecordParams, GetSmallLiabilityExchangeCoinListParams,
        GetSmallLiabilityExchangeHistoryParams, QueryAllOcoParams, QueryAllOrdersParams,
        QueryCurrentMarginOrderCountUsageParams, QueryOcoParams, QueryOpenOcoParams,
        QueryOpenOrdersParams, QueryOrderParams, QueryTradeListParams,
        SmallLiabilityExchangeParams,
    },
    rest_api::Endpoint,
};

mod common;

#[tokio::test]
async fn rest_margin_get_force_liquidation_record() {
    let client = common::setup_rest_api_client();
    let params = GetForceLiquidationRecordParams::new();
    let resp = client
        .margin()
        .trade()
        .get_force_liquidation_record()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_get_small_liability_exchange_coin_list() {
    let client = common::setup_rest_api_client();
    let params = GetSmallLiabilityExchangeCoinListParams::new();
    let resp = client
        .margin()
        .trade()
        .get_small_liability_exchange_coin_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_get_small_liability_exchange_history() {
    let client = common::setup_rest_api_client();
    let params = GetSmallLiabilityExchangeHistoryParams::new();
    let resp = client
        .margin()
        .trade()
        .get_small_liability_exchange_history()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_all_oco() {
    let client = common::setup_rest_api_client();
    let params = QueryAllOcoParams::new();
    let resp = client
        .margin()
        .trade()
        .query_all_oco()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_all_orders() {
    let client = common::setup_rest_api_client();
    let params = QueryAllOrdersParams::new("BTCUSDT");
    let resp = client
        .margin()
        .trade()
        .query_all_orders()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_current_margin_order_count_usage() {
    let client = common::setup_rest_api_client();
    let params = QueryCurrentMarginOrderCountUsageParams::new();
    let resp = client
        .margin()
        .trade()
        .query_current_margin_order_count_usage()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

// TODO: response is empty string?
#[ignore]
#[tokio::test]
async fn rest_margin_query_oco() {
    let client = common::setup_rest_api_client();
    let params = QueryOcoParams::new().symbol("BTCUSDT");
    let resp = client.margin().trade().query_oco().request(params).await;
    assert!(resp.is_ok());
}

// TODO: make this work.
#[ignore]
#[tokio::test]
async fn rest_margin_query_open_oco() {
    let client = common::setup_rest_api_client();
    let params = QueryOpenOcoParams::new().symbol("BTCUSDT");
    let resp = client
        .margin()
        .trade()
        .query_open_oco()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_open_orders() {
    let client = common::setup_rest_api_client();
    let params = QueryOpenOrdersParams::new().symbol("BTCUSDT");
    let resp = client
        .margin()
        .trade()
        .query_open_orders()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn rest_margin_query_order() {
    let client = common::setup_rest_api_client();
    let params = QueryOrderParams::new("BTCUSDT").order_id(0);
    let resp = client.margin().trade().query_order().request(params).await;
    assert!(resp.is_err());
}

#[tokio::test]
async fn rest_margin_query_trade_list() {
    let client = common::setup_rest_api_client();
    let params = QueryTradeListParams::new("BTCUSDT");
    let resp = client
        .margin()
        .trade()
        .query_trade_list()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
