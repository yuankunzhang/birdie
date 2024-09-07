use birdie::{
    errors::{BinanceError, BinanceErrorCode},
    spot::account::{
        AccountInformationParams, AccountTradeListParams, QueryAllocationsParams,
        QueryCommissionRatesParams, QueryPreventedMatchesParams, QueryUnfilledOrderCountParams,
    },
    web_socket_api::{ConnectionStatus, WebSocket, WebSocketApiError},
};

mod common;

#[tokio::test]
async fn ws_account_information() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = AccountInformationParams::new().omit_zero_balances(true);
    let resp = client.account().account_information().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn account_trade_list() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = AccountTradeListParams::new("BTCUSDT");
    let resp = client.account().account_trade_list().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn query_unfilled_order_count() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = QueryUnfilledOrderCountParams::new();
    let resp = client
        .account()
        .query_unfilled_order_count()
        .request(params)
        .await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn query_prevented_matches() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

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
    let WebSocketApiError::Binance(s, Some(BinanceError { code, .. })) = resp.err().unwrap() else {
        panic!()
    };
    assert_eq!(s, "400");
    assert!(matches!(code, BinanceErrorCode::OptionalParamsBadCombo));
}

#[tokio::test]
async fn query_allocations() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = QueryAllocationsParams::new("BTCUSDT");
    let resp = client.account().query_allocations().request(params).await;
    assert!(resp.is_ok());
}

#[tokio::test]
async fn query_commission_rates() {
    let mut client = common::setup_web_socket_api_client();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    client.connect(tx).await.unwrap();
    assert!(matches!(rx.recv().await, Some(ConnectionStatus::Connected)));

    let params = QueryCommissionRatesParams::new("BTCUSDT");
    let resp = client
        .account()
        .query_commission_rates()
        .request(params)
        .await;
    assert!(resp.is_ok());
}
